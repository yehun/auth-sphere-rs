<template>
  <div class="dashboard-container">
    <el-card class="dashboard-card">
      <div class="dashboard-header">
        <h1>欢迎，{{ userStore.userInfo?.nickname }}</h1>
        <p class="subtitle">您已成功登录到 {{ subjectName }} 系统</p>
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
            <el-tag :type="userTypeColor">{{ subjectName }}</el-tag>
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
            :rows="3"
          />
        </div>
      </div>

      <el-divider />

      <div class="mfa-section" v-if="showMfa">
        <h2>多因素认证 (MFA)</h2>
        <el-alert 
          title="MFA 功能开发中" 
          type="info" 
          :closable="false"
        />
      </div>

      <el-divider />

      <div class="actions-section">
        <el-button type="danger" size="large" @click="handleLogout" style="width: 100%">
          <el-icon><SwitchButton /></el-icon>
          退出登录
        </el-button>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useUserStore } from '@/stores/user'
import { UserKind } from '@/types'

const router = useRouter()
const userStore = useUserStore()

const subjectName = computed(() => {
  const names: Record<UserKind, string> = {
    [UserKind.Member]: '会员',
    [UserKind.Community]: '社区运营',
    [UserKind.Platform]: '平台运营'
  }
  return names[userStore.userKind || UserKind.Member] || '用户'
})

const userTypeColor = computed(() => {
  const colors: Record<UserKind, string> = {
    [UserKind.Member]: 'success',
    [UserKind.Community]: 'warning',
    [UserKind.Platform]: 'danger'
  }
  return colors[userStore.userKind || UserKind.Member] || ''
})

const showMfa = computed(() => {
  // MFA 功能可选，可以根据需要显示
  return false
})

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
  // 如果没有登录，跳转到首页
  if (!userStore.isLoggedIn) {
    ElMessage.warning('请先登录')
    router.push('/')
  }
})
</script>

<style scoped>
.dashboard-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 40px 20px;
}

.dashboard-card {
  width: 100%;
  max-width: 800px;
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
  margin-bottom: 10px;
}

.subtitle {
  color: #718096;
  font-size: 16px;
}

.user-info-section,
.token-section,
.mfa-section {
  margin: 30px 0;
}

.user-info-section h2,
.token-section h2,
.mfa-section h2 {
  font-size: 24px;
  color: #2d3748;
  margin-bottom: 20px;
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
</style>
