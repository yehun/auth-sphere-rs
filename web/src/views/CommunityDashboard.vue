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

const subjectName = computed(() => '社区运营')

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
  }
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
</style>
