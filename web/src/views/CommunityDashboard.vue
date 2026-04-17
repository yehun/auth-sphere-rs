<template>
  <div class="dashboard-container">
    <el-card class="dashboard-card">
      <div class="dashboard-header">
        <el-tag type="warning" size="large">社区运营</el-tag>
        <h1>欢迎，{{ userStore.userInfo?.nickname }}</h1>
        <p class="subtitle">社区管理后台</p>
      </div>

      <el-divider />

      <div class="user-info-section">
        <h2>用户信息</h2>
        <el-descriptions :column="1" border>
          <el-descriptions-item label="用户ID">
            {{ userStore.userInfo?.id }}
          </el-descriptions-item>
          <el-descriptions-item label="用户名">
            {{ userStore.userInfo?.username }}
          </el-descriptions-item>
          <el-descriptions-item label="昵称">
            {{ userStore.userInfo?.nickname }}
          </el-descriptions-item>
          <el-descriptions-item label="用户类型">
            <el-tag type="warning">{{ subjectName }}</el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </div>

      <el-divider />

      <div class="token-section">
        <h2>认证令牌</h2>
        <el-alert 
          title="请妥善保管您的访问令牌" 
          type="warning" 
          :closable="false"
          style="margin-bottom: 15px"
        />
        <div class="token-display">
          <label>Token:</label>
          <el-input 
            :model-value="userStore.token" 
            readonly
            type="textarea"
            :rows="2"
          />
        </div>
      </div>

      <el-divider />

      <div class="mfa-section">
        <h2>MFA 双因素认证</h2>
        <div class="mfa-control">
          <span class="mfa-label">MFA 状态</span>
          <el-switch
            v-model="mfaEnabled"
            :loading="mfaLoading"
            @change="handleMfaChange"
            active-text="已开启"
            inactive-text="未开启"
          />
        </div>
        <p class="mfa-description">
          MFA 提供额外的安全保护，需要验证码才能登录
        </p>
      </div>

      <el-divider />

      <div class="passkey-section">
        <h2>Passkey 无密码登录</h2>
        <div class="passkey-control">
          <span class="passkey-label">已注册的 Passkey</span>
          <el-tag :type="hasPasskey ? 'success' : 'info'" size="large">
            {{ hasPasskey ? '已启用' : '未注册' }}
          </el-tag>
        </div>
        <p class="passkey-description">
          Passkey 提供比密码更安全、更便捷的登录体验
        </p>
        <div class="passkey-actions">
          <el-button 
            type="primary" 
            size="large" 
            @click="handleRegisterPasskey"
            :loading="passkeyLoading"
            :disabled="hasPasskey"
            style="margin-right: 10px"
          >
            <el-icon><Key /></el-icon>
            {{ hasPasskey ? '已注册' : '注册 Passkey' }}
          </el-button>
          <el-button 
            type="danger" 
            size="large" 
            @click="handleDeactivatePasskey"
            :loading="passkeyLoading"
            :disabled="!hasPasskey"
          >
            <el-icon><Close /></el-icon>
            关闭 Passkey
          </el-button>
        </div>
      </div>

      <el-divider />

      <div class="actions-section">
        <el-button size="large" @click="handleGoHome" style="margin-right: 10px">
          <el-icon><HomeFilled /></el-icon>
          返回首页
        </el-button>
        <el-button type="danger" size="large" @click="handleLogout">
          <el-icon><SwitchButton /></el-icon>
          退出登录
        </el-button>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useUserStore } from '@/stores/user'
import { UserKind } from '@/types'
import { passkeyRegisterBegin, passkeyRegisterComplete, deactivatePasskey, getCurrentUser, generateMfaQRCode, activateMfa, deactivateMfa } from '@/api/auth'
import { startRegistration } from '@simplewebauthn/browser'

const router = useRouter()
const userStore = useUserStore()

const subjectName = computed(() => '社区运营')
const hasPasskey = ref(false)
const passkeyLoading = ref(false)
const mfaEnabled = ref(false)
const mfaLoading = ref(false)

// 注册 Passkey
async function handleRegisterPasskey() {
  if (!userStore.userInfo) {
    ElMessage.error('用户信息不存在')
    return
  }
  
  try {
    passkeyLoading.value = true
    
    const response = await passkeyRegisterBegin(UserKind.Community, {
      username: userStore.userInfo.username
    })
    
    console.log('[Passkey] Registration challenge received:', response)
    const options = response.publicKey || response
    
    console.log('[Passkey] Using options:', options)
    
    if (!options || !options.challenge) {
      console.error('[Passkey] Invalid challenge object:', options)
      ElMessage.error('获取挑战失败，请重试')
      return
    }
    
    const credential = await startRegistration(options)
    console.log('[Passkey] Registration completed:', credential)
    
    const completeResponse = await passkeyRegisterComplete(UserKind.Community, {
      username: userStore.userInfo.username,
      credential
    })
    
    if (completeResponse.code !== 200 && completeResponse.code !== 0) {
      ElMessage.error(completeResponse.message || 'Passkey 注册失败')
      return
    }
    
    hasPasskey.value = true
    ElMessage.success('Passkey 注册成功！')
  } catch (error: any) {
    console.error('Passkey registration error:', error)
    if (error.name === 'NotAllowedError') {
      ElMessage.warning('用户取消了 Passkey 注册')
    } else {
      ElMessage.error(error.message || 'Passkey 注册失败')
    }
  } finally {
    passkeyLoading.value = false
  }
}

// 关闭 Passkey
async function handleDeactivatePasskey() {
  try {
    await ElMessageBox.confirm(
      '关闭 Passkey 后，您将无法使用生物识别登录。确定要关闭吗？',
      '提示',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    passkeyLoading.value = true
    const response = await deactivatePasskey(UserKind.Community)
    
    if (response.code !== 200 && response.code !== 0) {
      ElMessage.error(response.message || '关闭 Passkey 失败')
      return
    }
    
    hasPasskey.value = false
    ElMessage.success('Passkey 已关闭')
    
    // 重新获取用户信息
    await fetchUserInfo()
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Deactivate Passkey error:', error)
      ElMessage.error(error.message || '关闭 Passkey 失败')
    }
  } finally {
    passkeyLoading.value = false
  }
}

// 获取用户信息
async function fetchUserInfo() {
  try {
    const response = await getCurrentUser(UserKind.Community)
    
    // 更新 store 中的用户信息
    if (userStore.userInfo) {
      userStore.userInfo.id = response.data.id
      userStore.userInfo.username = response.data.username
      userStore.userInfo.nickname = response.data.nickname
      userStore.userInfo.user_type = response.data.user_type
      userStore.userInfo.is_mfa = response.data.is_mfa
      userStore.userInfo.is_passkey = response.data.is_passkey
    }
    
    // 根据用户信息更新状态
    hasPasskey.value = response.data.is_passkey || false
    mfaEnabled.value = response.data.is_mfa || false
    
    console.log('[CommunityDashboard] User info fetched:', response.data)
  } catch (error) {
    console.error('[CommunityDashboard] Fetch user info error:', error)
    ElMessage.error('获取用户信息失败')
  }
}

// MFA 开关变化处理
async function handleMfaChange(enabled: boolean) {
  if (enabled) {
    await handleEnableMfa()
  } else {
    await handleDisableMfa()
  }
}

// 开启 MFA
async function handleEnableMfa() {
  try {
    mfaLoading.value = true
    
    const response = await generateMfaQRCode(UserKind.Community)
    const { qr_code, secret, uri } = response.data
    
    await ElMessageBox.confirm(
      `<div style="text-align: center;">
        <p style="margin-bottom: 15px; color: #606266;">请使用认证器应用扫描二维码</p>
        <img src="data:image/png;base64,${qr_code}" style="max-width: 250px; border: 1px solid #dcdfe6; padding: 10px; border-radius: 8px;" />
        <p style="margin-top: 15px; font-size: 12px; color: #909399;">密钥: ${secret}</p>
        <p style="margin-top: 15px; font-size: 12px; color: #909399;">🔗: ${uri}</p>
       </div>`,
      '开启 MFA 验证',
      {
        confirmButtonText: '我已扫描并激活',
        cancelButtonText: '取消',
        dangerouslyUseHTMLString: true,
        type: 'info'
      }
    )
    
    await activateMfa(UserKind.Community, '')
    
    mfaEnabled.value = true
    ElMessage.success('MFA 已开启')
    
    await fetchUserInfo()
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Enable MFA error:', error)
      ElMessage.error(error.message || '开启 MFA 失败')
      mfaEnabled.value = false
    }
  } finally {
    mfaLoading.value = false
  }
}

// 关闭 MFA
async function handleDisableMfa() {
  try {
    await ElMessageBox.confirm(
      '关闭 MFA 会降低账户安全性，确定要关闭吗？',
      '提示',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    mfaLoading.value = true
    await deactivateMfa(UserKind.Community)
    
    mfaEnabled.value = false
    ElMessage.success('MFA 已关闭')
    
    await fetchUserInfo()
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Disable MFA error:', error)
      ElMessage.error(error.message || '关闭 MFA 失败')
      mfaEnabled.value = true
    }
  } finally {
    mfaLoading.value = false
  }
}

// 返回首页
function handleGoHome() {
  router.push('/')
}

async function handleLogout() {
  try {
    await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await userStore.logout()
    ElMessage.success('已退出登录')
    router.push('/')
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Logout error:', error)
    }
  }
}

onMounted(() => {
  if (!userStore.isLoggedIn || userStore.userKind !== UserKind.Community) {
    ElMessage.warning('请先登录')
    router.push('/login/community')
    return
  }
  
  // 获取用户信息
  fetchUserInfo()
})
</script>

<style scoped>
.dashboard-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  padding: 40px 20px;
}

.dashboard-card {
  width: 100%;
  max-width: 900px;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  padding: 40px;
}

.dashboard-header {
  text-align: center;
}

.dashboard-header h1 {
  font-size: 32px;
  color: #2d3748;
  margin: 15px 0 10px;
}

.subtitle {
  color: #718096;
  font-size: 16px;
}

.user-info-section,
.features-section,
.token-section {
  margin: 30px 0;
}

.user-info-section h2,
.features-section h2,
.token-section h2 {
  font-size: 24px;
  color: #2d3748;
  margin-bottom: 20px;
}

.feature-card {
  text-align: center;
  padding: 20px;
  transition: transform 0.3s;
  cursor: pointer;
}

.feature-card:hover {
  transform: translateY(-5px);
}

.feature-card h3 {
  font-size: 18px;
  margin: 15px 0 10px;
  color: #2d3748;
}

.feature-card p {
  font-size: 14px;
  color: #718096;
}

.token-display {
  margin-bottom: 15px;
}

.token-display label {
  display: block;
  font-weight: bold;
  color: #4a5568;
  margin-bottom: 8px;
}

.actions-section {
  margin-top: 30px;
}

.passkey-section h2 {
  font-size: 24px;
  color: #2d3748;
  margin-bottom: 20px;
}

.passkey-control {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 15px;
  background: #f7fafc;
  border-radius: 8px;
  margin-bottom: 10px;
}

.passkey-label {
  font-size: 16px;
  color: #2d3748;
  font-weight: 500;
}

.passkey-description {
  font-size: 14px;
  color: #718096;
  margin: 0;
}

.passkey-actions {
  display: flex;
  gap: 10px;
  margin-top: 15px;
}

.mfa-section h2 {
  font-size: 24px;
  color: #2d3748;
  margin-bottom: 20px;
}

.mfa-control {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 15px;
  background: #f7fafc;
  border-radius: 8px;
  margin-bottom: 10px;
}

.mfa-label {
  font-size: 16px;
  color: #2d3748;
  font-weight: 500;
}

.mfa-description {
  font-size: 14px;
  color: #718096;
  margin: 0;
}
</style>
