import { ElMessage, ElMessageBox } from 'element-plus'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import type { UserKind } from '@/types'
import {
  passwordLogin,
  verify2FA,
  otpLogin,
  sendOtp,
  passkeyLoginBegin,
  passkeyLoginComplete,
  getCurrentUser
} from '@/api/auth'
import { startAuthentication } from '@simplewebauthn/browser'
import type { LoginResponse } from '@/types'

/**
 * 登录结果
 */
export interface LoginResult {
  success: boolean
  requires2FA?: boolean
  tempToken?: string
  message?: string
}

/**
 * 认证服务 - 统一管理所有认证逻辑
 */
export class AuthService {
  private router: ReturnType<typeof useRouter>
  private userStore: ReturnType<typeof useUserStore>

  constructor() {
    this.router = useRouter()
    this.userStore = useUserStore()
  }

  /**
   * 密码登录
   */
  async loginWithPassword(
    userKind: UserKind,
    username: string,
    password: string
  ): Promise<LoginResult> {
    try {
      const response = await passwordLogin(userKind, {
        username,
        password
      })

      if (response.code !== 200 && response.code !== 0) {
        return {
          success: false,
          message: response.message || '登录失败'
        }
      }

      // 检查是否需要 2FA
      if ('requires_2fa' in response.data && response.data.requires_2fa) {
        return {
          success: true,
          requires2FA: true,
          tempToken: response.data.token
        }
      }

      // 直接登录成功
      const loginData = response.data as LoginResponse
      this.handleLoginSuccess(loginData, userKind)
      
      return {
        success: true,
        message: '登录成功'
      }
    } catch (error: any) {
      console.error('Password login error:', error)
      return {
        success: false,
        message: error.message || '登录失败，请重试'
      }
    }
  }

  /**
   * 2FA 验证
   */
  async verify2FA(
    userKind: UserKind,
    token: string,
    code: string
  ): Promise<LoginResult> {
    try {
      const response = await verify2FA(userKind, { token, code })

      if (response.code !== 200 && response.code !== 0) {
        return {
          success: false,
          message: response.message || '验证码错误'
        }
      }

      this.handleLoginSuccess(response.data, userKind)
      
      return {
        success: true,
        message: '验证成功'
      }
    } catch (error: any) {
      console.error('2FA verification error:', error)
      return {
        success: false,
        message: error.message || '验证失败，请重试'
      }
    }
  }

  /**
   * OTP 登录
   */
  async loginWithOtp(
    userKind: UserKind,
    contact: string,
    otpCode: string
  ): Promise<LoginResult> {
    try {
      const response = await otpLogin(userKind, {
        contact,
        otp_code: otpCode
      })

      if (response.code !== 200 && response.code !== 0) {
        return {
          success: false,
          message: response.message || 'OTP 登录失败'
        }
      }

      this.handleLoginSuccess(response.data, userKind)
      
      return {
        success: true,
        message: '登录成功'
      }
    } catch (error: any) {
      console.error('OTP login error:', error)
      return {
        success: false,
        message: error.message || '登录失败，请重试'
      }
    }
  }

  /**
   * 发送 OTP 验证码
   */
  async sendOtpCode(
    userKind: UserKind,
    email?: string,
    phone?: string
  ): Promise<{ success: boolean; message: string }> {
    try {
      const response = await sendOtp(userKind, { email, phone })

      if (response.code !== 200 && response.code !== 0) {
        return {
          success: false,
          message: response.message || '发送失败'
        }
      }

      return {
        success: true,
        message: response.data.message || '验证码已发送'
      }
    } catch (error: any) {
      console.error('Send OTP error:', error)
      return {
        success: false,
        message: error.message || '发送失败，请重试'
      }
    }
  }

  /**
   * Passkey 登录
   */
  async loginWithPasskey(
    userKind: UserKind,
    username: string
  ): Promise<LoginResult> {
    try {
      // 1. 开始 Passkey 登录流程
      const beginResponse = await passkeyLoginBegin(userKind, { username })

      // 检查是否返回了错误
      if (beginResponse.code && beginResponse.code !== 200 && beginResponse.code !== 0) {
        return {
          success: false,
          message: beginResponse.message || '该用户未启用 Passkey，请使用密码登录'
        }
      }

      // 2. 获取挑战选项
      const options = beginResponse.publicKey || beginResponse
      
      if (!options || !options.challenge) {
        console.error('[Passkey] Invalid challenge object:', options)
        return {
          success: false,
          message: '获取挑战失败，请重试'
        }
      }

      // 3. 调用浏览器 WebAuthn API
      const credential = await startAuthentication(options)

      // 4. 完成 Passkey 登录
      const completeResponse = await passkeyLoginComplete(userKind, {
        username,
        credential
      })

      if (completeResponse.code !== 200 && completeResponse.code !== 0) {
        return {
          success: false,
          message: completeResponse.message || 'Passkey 登录失败'
        }
      }

      this.handleLoginSuccess(completeResponse.data, userKind)
      
      return {
        success: true,
        message: 'Passkey 登录成功'
      }
    } catch (error: any) {
      console.error('Passkey login error:', error)
      
      if (error.name === 'NotAllowedError') {
        return {
          success: false,
          message: '用户取消了认证或超时'
        }
      }
      
      if (error.message && error.message.includes('allowCredentials')) {
        return {
          success: false,
          message: '该用户未启用 Passkey，请使用密码登录'
        }
      }

      return {
        success: false,
        message: error.message || 'Passkey 登录失败'
      }
    }
  }

  /**
   * 获取当前用户信息
   */
  async fetchUserInfo(userKind: UserKind): Promise<boolean> {
    try {
      const response = await getCurrentUser(userKind)
      
      if (response.code !== 200 && response.code !== 0) {
        ElMessage.error(response.message || '获取用户信息失败')
        return false
      }

      // 更新 store 中的用户信息
      if (this.userStore.userInfo) {
        Object.assign(this.userStore.userInfo, response.data)
      }

      return true
    } catch (error) {
      console.error('Fetch user info error:', error)
      ElMessage.error('获取用户信息失败')
      return false
    }
  }

  /**
   * 处理登录成功
   */
  private handleLoginSuccess(loginData: LoginResponse, userKind: UserKind): void {
    // 保存设备类型
    localStorage.setItem('device_type', '1') // Web
    
    // 保存登录信息
    this.userStore.setLoginInfo(loginData, userKind)
    
    // 跳转到对应的 Dashboard
    const dashboardPath = this.getDashboardPath(userKind)
    this.router.push(dashboardPath)
  }

  /**
   * 根据用户类型获取 Dashboard 路径
   */
  private getDashboardPath(userKind: UserKind): string {
    const paths: Record<UserKind, string> = {
      member: '/dashboard/member',
      community: '/dashboard/community',
      platform: '/dashboard/platform'
    }
    return paths[userKind] || '/'
  }

  /**
   * 登出
   */
  async logout(): Promise<void> {
    try {
      await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      })
      
      await this.userStore.logout()
      ElMessage.success('已退出登录')
      this.router.push('/')
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Logout error:', error)
      }
    }
  }

  /**
   * 返回首页
   */
  goHome(): void {
    this.router.push('/')
  }
}

// 导出单例
export const authService = new AuthService()
