<template>
  <div class="login-container">
    <el-card class="login-card">
      <div class="login-header">
        <el-button text @click="goHome" class="back-btn">
          <el-icon><ArrowLeft /></el-icon>
          返回首页
        </el-button>
        <h1>{{ subjectName }}登录</h1>
        <p class="subtitle">选择您的登录方式</p>
      </div>

      <!-- 登录方式切换 -->
      <el-tabs v-model="loginMethod" class="login-tabs">
        <el-tab-pane label="密码登录" name="password">
          <el-form :model="passwordForm" :rules="passwordRules" ref="passwordFormRef" class="login-form">
            <el-form-item prop="username">
              <el-input 
                v-model="passwordForm.username" 
                placeholder="用户名/邮箱/手机号"
                prefix-icon="User"
                size="large"
              />
            </el-form-item>
            
            <el-form-item prop="password">
              <el-input 
                v-model="passwordForm.password" 
                type="password"
                placeholder="密码"
                prefix-icon="Lock"
                size="large"
                show-password
                @keyup.enter="handlePasswordLogin"
              />
            </el-form-item>

            <el-form-item>
              <el-select v-model="passwordForm.device" placeholder="选择设备类型" size="large" style="width: 100%">
                <el-option label="Web" value="1" />
                <el-option label="Android" value="2" />
                <el-option label="iOS" value="3" />
                <el-option label="Desktop" value="4" />
              </el-select>
            </el-form-item>

            <el-form-item>
              <el-button 
                type="primary" 
                size="large" 
                :loading="loading"
                @click="handlePasswordLogin"
                style="width: 100%"
              >
                登录
              </el-button>
            </el-form-item>
          </el-form>
        </el-tab-pane>

        <el-tab-pane label="OTP验证码登录" name="otp">
          <el-form :model="otpForm" :rules="otpRules" ref="otpFormRef" class="login-form">
            <el-form-item prop="contact">
              <el-input 
                v-model="otpForm.contact" 
                placeholder="邮箱或手机号"
                prefix-icon="Message"
                size="large"
              />
            </el-form-item>

            <el-form-item prop="otp_code">
              <div style="display: flex; gap: 10px">
                <el-input 
                  v-model="otpForm.otp_code" 
                  placeholder="6位验证码"
                  prefix-icon="Key"
                  size="large"
                  maxlength="6"
                  @keyup.enter="handleOtpLogin"
                />
                <el-button 
                  type="success" 
                  size="large"
                  :loading="sendingOtp"
                  :disabled="countdown > 0"
                  @click="handleSendOtp"
                  style="min-width: 120px"
                >
                  {{ countdown > 0 ? `${countdown}s` : '发送验证码' }}
                </el-button>
              </div>
            </el-form-item>

            <el-form-item>
              <el-select v-model="otpForm.device" placeholder="选择设备类型" size="large" style="width: 100%">
                <el-option label="Web" value="1" />
                <el-option label="Android" value="2" />
                <el-option label="iOS" value="3" />
                <el-option label="Desktop" value="4" />
              </el-select>
            </el-form-item>

            <el-form-item>
              <el-button 
                type="primary" 
                size="large" 
                :loading="loading"
                @click="handleOtpLogin"
                style="width: 100%"
              >
                登录
              </el-button>
            </el-form-item>
          </el-form>
        </el-tab-pane>

        <el-tab-pane label="Passkey登录" name="passkey">
          <div class="passkey-section">
            <el-icon :size="80" color="#409EFF"><Key /></el-icon>
            <h3>使用 Passkey 登录</h3>
            <p>更安全、更便捷的无密码登录方式</p>
            <el-button 
              type="primary" 
              size="large" 
              :loading="loading"
              @click="handlePasskeyLogin"
              style="margin-top: 20px"
            >
              使用 Passkey 登录
            </el-button>
            <el-alert 
              title="Passkey 功能开发中" 
              type="info" 
              :closable="false"
              style="margin-top: 20px"
            />
          </div>
        </el-tab-pane>
      </el-tabs>

      <div class="login-footer">
        <p>还没有账号？ <router-link to="/register">立即注册</router-link></p>
      </div>
    </el-card>

    <!-- 2FA 验证对话框 -->
    <el-dialog 
      v-model="show2faDialog" 
      title="双因素认证" 
      width="400px"
      :close-on-click-modal="false"
      :close-on-press-escape="false"
    >
      <div style="text-align: center; padding: 20px 0;">
        <el-icon :size="60" color="#409EFF"><Lock /></el-icon>
        <p style="margin-top: 15px; color: #606266;">请输入认证器应用中的 6 位验证码</p>
        <el-input 
          v-model="faCode" 
          placeholder="000000"
          maxlength="6"
          size="large"
          style="margin-top: 20px; text-align: center;"
          @keyup.enter="handleSubmit2FA"
        />
      </div>
      <template #footer>
        <el-button @click="show2faDialog = false">取消</el-button>
        <el-button 
          type="primary" 
          :loading="faLoading"
          @click="handleSubmit2FA"
        >
          验证
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { useUserStore } from '@/stores/user'
import { passwordLogin, otpLogin, sendOtp, verify2FA } from '@/api/auth'
import { UserKind, DeviceType, LoginResponse } from '@/types'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const loginMethod = ref('password')
const loading = ref(false)
const sendingOtp = ref(false)
const countdown = ref(0)

// 2FA 相关状态
const show2faDialog = ref(false)
const tempToken = ref('')
const faCode = ref('')
const faLoading = ref(false)

const userKind = computed(() => route.params.userKind as UserKind)

const subjectName = computed(() => {
  const names: Record<UserKind, string> = {
    [UserKind.Member]: '会员',
    [UserKind.Community]: '社区运营',
    [UserKind.Platform]: '平台运营'
  }
  return names[userKind.value] || '用户'
})

// 密码登录表单
const passwordFormRef = ref<FormInstance>()
const passwordForm = ref({
  username: '',
  password: '',
  device: DeviceType.Web
})

const passwordRules: FormRules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
}

// OTP 登录表单
const otpFormRef = ref<FormInstance>()
const otpForm = ref({
  contact: '',
  otp_code: '',
  device: DeviceType.Web
})

const otpRules: FormRules = {
  contact: [{ required: true, message: '请输入邮箱或手机号', trigger: 'blur' }],
  otp_code: [
    { required: true, message: '请输入验证码', trigger: 'blur' },
    { pattern: /^\d{6}$/, message: '验证码必须为6位数字', trigger: 'blur' }
  ]
}

// 监听设备类型变化，立即保存到 localStorage
watch(
  () => [passwordForm.value.device, otpForm.value.device],
  ([pwdDevice, otpDevice]) => {
    // 根据当前登录方式保存对应的设备类型
    if (loginMethod.value === 'password') {
      localStorage.setItem('device_type', pwdDevice)
      console.log('[Login] Password device changed to:', pwdDevice)
    } else if (loginMethod.value === 'otp') {
      localStorage.setItem('device_type', otpDevice)
      console.log('[Login] OTP device changed to:', otpDevice)
    }
  }
)

// 监听登录方式切换，保存当前选择的设备类型
watch(loginMethod, (newMethod) => {
  const device = newMethod === 'password' 
    ? passwordForm.value.device 
    : otpForm.value.device
  localStorage.setItem('device_type', device)
  console.log('[Login] Method changed to:', newMethod, ', Device:', device)
})

// 密码登录
async function handlePasswordLogin() {
  if (!passwordFormRef.value) return
  
  await passwordFormRef.value.validate(async (valid) => {
    if (!valid) return
    
    loading.value = true
    try {
      const response = await passwordLogin(userKind.value, {
        username: passwordForm.value.username,
        password: passwordForm.value.password
      })
      
      console.log('[Login] Password login response:', response)
      console.log('[Login] Response data:', response.data)
      
      // 检查是否需要 2FA 验证
      if ('requires_2fa' in response.data && response.data.requires_2fa) {
        // 需要 2FA 验证
        tempToken.value = response.data.token
        show2faDialog.value = true
        ElMessage.info('请输入 2FA 验证码')
      } else {
        // 不需要 2FA，直接登录
        const loginData = response.data as LoginResponse
        
        // 保存设备类型到 localStorage
        localStorage.setItem('device_type', passwordForm.value.device)
        
        userStore.setLoginInfo(loginData, userKind.value)
        ElMessage.success('登录成功')
        
        // 根据用户类型跳转到对应的 Dashboard
        const dashboardPath = getDashboardPath(userKind.value)
        router.push(dashboardPath)
      }
    } catch (error) {
      console.error('Password login error:', error)
    } finally {
      loading.value = false
    }
  })
}

// 提交 2FA 验证码
async function handleSubmit2FA() {
  if (!faCode.value || faCode.value.length !== 6) {
    ElMessage.error('请输入6位验证码')
    return
  }
  
  faLoading.value = true
  try {
    const response = await verify2FA(userKind.value, {
      token: tempToken.value,
      code: faCode.value
    })
    
    // 保存设备类型到 localStorage
    localStorage.setItem('device_type', passwordForm.value.device)
    
    userStore.setLoginInfo(response.data, userKind.value)
    ElMessage.success('登录成功')
    
    // 关闭对话框并跳转
    show2faDialog.value = false
    faCode.value = ''
    tempToken.value = ''
    
    const dashboardPath = getDashboardPath(userKind.value)
    router.push(dashboardPath)
  } catch (error: any) {
    console.error('2FA verification error:', error)
    ElMessage.error(error.message || '2FA 验证失败')
  } finally {
    faLoading.value = false
  }
}

// 发送 OTP
async function handleSendOtp() {
  if (!otpFormRef.value) return
  
  await otpFormRef.value.validateField('contact', async (valid) => {
    if (!valid) return
    
    sendingOtp.value = true
    try {
      const contact = otpForm.value.contact
      const data = contact.includes('@') 
        ? { email: contact }
        : { phone: contact }
      
      await sendOtp(userKind.value, data)
      ElMessage.success('验证码已发送')
      
      // 开始倒计时
      countdown.value = 60
      const timer = setInterval(() => {
        countdown.value--
        if (countdown.value <= 0) {
          clearInterval(timer)
        }
      }, 1000)
    } catch (error) {
      console.error('Send OTP error:', error)
    } finally {
      sendingOtp.value = false
    }
  })
}

// OTP 登录
async function handleOtpLogin() {
  if (!otpFormRef.value) return
  
  await otpFormRef.value.validate(async (valid) => {
    if (!valid) return
    
    loading.value = true
    try {
      const response = await otpLogin(userKind.value, {
        contact: otpForm.value.contact,
        otp_code: otpForm.value.otp_code
      })
      
      // 保存设备类型到 localStorage
      localStorage.setItem('device_type', otpForm.value.device)
      
      userStore.setLoginInfo(response.data, userKind.value)
      ElMessage.success('登录成功')
      
      // 根据用户类型跳转到对应的 Dashboard
      const dashboardPath = getDashboardPath(userKind.value)
      router.push(dashboardPath)
    } catch (error) {
      console.error('OTP login error:', error)
    } finally {
      loading.value = false
    }
  })
}

// Passkey 登录(待实现)
function handlePasskeyLogin() {
  ElMessage.info('Passkey 功能开发中')
}

// 根据用户类型获取对应的 Dashboard 路径
function getDashboardPath(kind: UserKind): string {
  const paths: Record<UserKind, string> = {
    [UserKind.Member]: '/dashboard/member',
    [UserKind.Community]: '/dashboard/community',
    [UserKind.Platform]: '/dashboard/platform'
  }
  return paths[kind] || '/'
}

function goHome() {
  router.push('/')
}

onMounted(() => {
  // 验证 userKind 是否有效
  if (!Object.values(UserKind).includes(userKind.value)) {
    ElMessage.error('无效的用户类型')
    router.push('/')
    return
  }
  
  // 如果已经登录,直接跳转到对应的 Dashboard
  if (userStore.isLoggedIn && userStore.userKind === userKind.value) {
    const dashboardPath = getDashboardPath(userKind.value)
    router.push(dashboardPath)
  }
})
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 20px;
}

.login-card {
  width: 100%;
  max-width: 500px;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.login-header {
  text-align: center;
  margin-bottom: 30px;
}

.back-btn {
  position: absolute;
  top: 20px;
  left: 20px;
  font-size: 16px;
  background: #40A1D7;
}

.login-header h1 {
  font-size: 32px;
  color: #2d3748;
  margin: 10px 0;
}

.subtitle {
  color: #718096;
  font-size: 14px;
}

.login-tabs {
  margin-bottom: 20px;
}

.login-form {
  padding: 20px 0;
}

.passkey-section {
  text-align: center;
  padding: 40px 20px;
}

.passkey-section h3 {
  font-size: 24px;
  color: #2d3748;
  margin: 20px 0 10px;
}

.passkey-section p {
  color: #718096;
  font-size: 14px;
}

.login-footer {
  text-align: center;
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid #e2e8f0;
}

.login-footer a {
  color: #409EFF;
  text-decoration: none;
}

.login-footer a:hover {
  text-decoration: underline;
}
</style>
