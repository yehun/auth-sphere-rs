import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { useUserStore } from '@/stores/user'
import type { UserKind } from '@/types'
import { authService } from '@/services/auth'

/**
 * Dashboard 基础 Composable
 * 提供通用的 Dashboard 功能
 */
export function useDashboard(userKind: UserKind) {
  const router = useRouter()
  const userStore = useUserStore()
  
  // 状态
  const loadingUserInfo = ref(false)
  const mfaEnabled = ref(false)
  const hasPasskey = ref(false)
  const mfaLoading = ref(false)
  const passkeyLoading = ref(false)

  /**
   * 获取用户信息
   */
  async function fetchUserInfo() {
    try {
      loadingUserInfo.value = true
      const success = await authService.fetchUserInfo(userKind)
      
      if (success && userStore.userInfo) {
        // 更新 MFA 和 Passkey 状态
        mfaEnabled.value = userStore.userInfo.is_mfa || false
        hasPasskey.value = userStore.userInfo.is_passkey || false
      }
    } catch (error) {
      console.error('Fetch user info error:', error)
    } finally {
      loadingUserInfo.value = false
    }
  }

  /**
   * 验证登录状态
   */
  function checkAuth(): boolean {
    if (!userStore.isLoggedIn || userStore.userKind !== userKind) {
      ElMessage.warning('请先登录')
      const loginPath = `/login/${userKind}`
      router.push(loginPath)
      return false
    }
    return true
  }

  /**
   * 登出
   */
  async function handleLogout() {
    await authService.logout()
  }

  /**
   * 返回首页
   */
  function handleGoHome() {
    authService.goHome()
  }

  /**
   * 初始化
   */
  function init() {
    onMounted(() => {
      if (checkAuth()) {
        fetchUserInfo()
      }
    })
  }

  return {
    // 状态
    loadingUserInfo,
    mfaEnabled,
    hasPasskey,
    mfaLoading,
    passkeyLoading,
    
    // 方法
    fetchUserInfo,
    checkAuth,
    handleLogout,
    handleGoHome,
    init
  }
}
