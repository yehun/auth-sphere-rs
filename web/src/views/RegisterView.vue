<template>
  <div class="register-container">
    <el-card class="register-card">
      <div class="register-header">
        <el-button text @click="goHome" class="back-btn">
          <el-icon><ArrowLeft /></el-icon>
          返回首页
        </el-button>
        <h1>用户注册</h1>
        <p class="subtitle">创建您的账号</p>
      </div>

      <el-form :model="registerForm" :rules="registerRules" ref="formRef" class="register-form">
        <el-form-item prop="user_type">
          <el-select v-model="registerForm.user_type" placeholder="选择用户类型" size="large" style="width: 100%">
            <el-option label="会员" value="member" />
            <el-option label="社区运营" value="community" />
            <el-option label="平台运营" value="platform" />
          </el-select>
        </el-form-item>

        <el-form-item prop="nickname">
          <el-input 
            v-model="registerForm.nickname" 
            placeholder="昵称（1-5位）"
            prefix-icon="User"
            size="large"
            minlength="3"
            maxlength="20"
            show-word-limit
          />
        </el-form-item>

        <el-form-item prop="username">
          <el-input 
            v-model="registerForm.username" 
            placeholder="用户名（5-20位）"
            prefix-icon="User"
            size="large"
            minlength="5"
            maxlength="20"
            show-word-limit
          />
        </el-form-item>
        
        <el-form-item prop="password">
          <el-input 
            v-model="registerForm.password" 
            type="password"
            placeholder="密码（6-32位）"
            prefix-icon="Lock"
            size="large"
            show-password
            minlength="6"
            maxlength="32"
          />
        </el-form-item>

        <el-form-item prop="email">
          <el-input 
            v-model="registerForm.email" 
            placeholder="邮箱（可选）"
            prefix-icon="Message"
            size="large"
          />
        </el-form-item>

        <el-form-item prop="phone">
          <el-input 
            v-model="registerForm.phone" 
            placeholder="手机号（可选）"
            prefix-icon="Phone"
            size="large"
          />
        </el-form-item>

        <el-form-item>
          <el-button 
            type="primary" 
            size="large" 
            :loading="loading"
            @click="handleRegister"
            style="width: 100%"
          >
            注册
          </el-button>
        </el-form-item>
      </el-form>

      <div class="register-footer">
        <p>已有账号？ <router-link to="/">立即登录</router-link></p>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { register } from '@/api/auth'

const router = useRouter()
const formRef = ref<FormInstance>()
const loading = ref(false)

const registerForm = ref({
  nickname: '',
  username: '',
  password: '',
  user_type: 'member',
  email: '',
  phone: ''
})

const registerRules: FormRules = {
  user_type: [{ required: true, message: '请选择用户类型', trigger: 'change' }],
  nickname: [
    { required: true, message: '请输入昵称', trigger: 'blur' },
    { min: 1, max: 5, message: '昵称长度在 1 到 5 个字符', trigger: 'blur' }
  ],
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 5, max: 20, message: '用户名长度在 5 到 20 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 32, message: '密码长度在 6 到 32 个字符', trigger: 'blur' }
  ],
  email: [
    { type: 'email', message: '请输入正确的邮箱地址', trigger: 'blur' }
  ],
  phone: [
    { pattern: /^1[3-9]\d{9}$/, message: '请输入正确的手机号', trigger: 'blur' }
  ]
}

async function handleRegister() {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (!valid) return
    
    loading.value = true
    try {
      await register({
        nickname: registerForm.value.nickname,
        username: registerForm.value.username,
        password: registerForm.value.password,
        user_type: registerForm.value.user_type,
        email: registerForm.value.email || undefined,
        phone: registerForm.value.phone || undefined
      })
      
      ElMessage.success('注册成功，请登录')
      router.push('/')
    } catch (error) {
      console.error('Register error:', error)
    } finally {
      loading.value = false
    }
  })
}

function goHome() {
  router.push('/')
}
</script>

<style scoped>
.register-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 20px;
}

.register-card {
  width: 100%;
  max-width: 500px;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.register-header {
  text-align: center;
  margin-bottom: 30px;
  position: relative;
}

.back-btn {
  position: absolute;
  top: 20px;
  left: 20px;
  font-size: 16px;
}

.register-header h1 {
  font-size: 32px;
  color: #2d3748;
  margin: 10px 0;
}

.subtitle {
  color: #718096;
  font-size: 14px;
}

.register-form {
  padding: 20px 0;
}

.register-footer {
  text-align: center;
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid #e2e8f0;
}

.register-footer a {
  color: #409EFF;
  text-decoration: none;
}

.register-footer a:hover {
  text-decoration: underline;
}
</style>
