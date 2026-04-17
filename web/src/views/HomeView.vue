<template>
  <div class="home-container">
    <div class="hero-section">
      <h1 class="title">Auth-Sphere 统一认证平台</h1>
      <p class="subtitle">支持多主体、多凭证的统一身份认证系统</p>
    </div>

    <div class="subjects-grid">
      <el-card 
        v-for="subject in subjects" 
        :key="subject.kind"
        class="subject-card"
        @click="goToLogin(subject.kind)"
      >
        <div class="card-content">
          <el-icon :size="60" :color="subject.color">
            <component :is="subject.icon" />
          </el-icon>
          <h2>{{ subject.name }}</h2>
          <p>{{ subject.description }}</p>
          <el-button type="primary" :style="{ backgroundColor: subject.color, borderColor: subject.color }">
            进入登录
          </el-button>
        </div>
      </el-card>
    </div>

    <div class="features-section">
      <h2>核心特性</h2>
      <div class="features-grid">
        <div class="feature-item" v-for="feature in features" :key="feature.title">
          <el-icon :size="40" color="#409EFF"><component :is="feature.icon" /></el-icon>
          <h3>{{ feature.title }}</h3>
          <p>{{ feature.description }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { UserKind } from '@/types'

const router = useRouter()

const subjects = [
  {
    kind: UserKind.Member,
    name: '会员',
    description: '普通用户，享受平台服务',
    icon: 'User',
    color: '#67C23A'
  },
  {
    kind: UserKind.Community,
    name: '社区运营',
    description: '社区管理人员，维护社区秩序',
    icon: 'UserFilled',
    color: '#E6A23C'
  },
  {
    kind: UserKind.Platform,
    name: '平台运营',
    description: '平台管理员，管理系统全局',
    icon: 'Avatar',
    color: '#F56C6C'
  }
]

const features = [
  {
    title: '多主体支持',
    description: '支持会员、社区运营、平台运营三类主体独立登录',
    icon: 'Connection'
  },
  {
    title: '多维凭证',
    description: '支持密码、OTP验证码、Passkey等多种认证方式',
    icon: 'Key'
  },
  {
    title: '多设备管理',
    description: '支持Web、Android、iOS、Desktop等多设备登录',
    icon: 'Monitor'
  },
  {
    title: 'MFA增强安全',
    description: '可选的多因素认证，进一步提升账户安全性',
    icon: 'Lock'
  }
]

function goToLogin(kind: UserKind) {
  router.push(`/login/${kind}`)
}
</script>

<style scoped>
.home-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 40px 20px;
}

.hero-section {
  text-align: center;
  color: white;
  margin-bottom: 60px;
}

.title {
  font-size: 48px;
  font-weight: bold;
  margin-bottom: 20px;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
}

.subtitle {
  font-size: 20px;
  opacity: 0.9;
}

.subjects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 30px;
  max-width: 1200px;
  margin: 0 auto 60px;
  padding: 0 20px;
}

.subject-card {
  cursor: pointer;
  transition: all 0.3s ease;
  border-radius: 16px;
  overflow: hidden;
}

.subject-card:hover {
  transform: translateY(-10px);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
}

.card-content {
  padding: 40px 30px;
  text-align: center;
}

.card-content h2 {
  font-size: 28px;
  margin: 20px 0 10px;
  color: #2d3748;
}

.card-content p {
  font-size: 16px;
  color: #718096;
  margin-bottom: 20px;
}

.features-section {
  max-width: 1200px;
  margin: 0 auto;
  padding: 60px 20px;
  background: white;
  border-radius: 20px;
}

.features-section h2 {
  text-align: center;
  font-size: 36px;
  margin-bottom: 40px;
  color: #2d3748;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 40px;
}

.feature-item {
  text-align: center;
  padding: 20px;
}

.feature-item h3 {
  font-size: 22px;
  margin: 20px 0 10px;
  color: #2d3748;
}

.feature-item p {
  font-size: 15px;
  color: #718096;
  line-height: 1.6;
}
</style>
