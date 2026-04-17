/**
 * 登录视图 - 使用服务层架构（优化版示例）
 * 
 * 这个文件展示了如何使用新的架构重构 LoginView
 * 实际使用时可以将此代码合并到现有的 LoginView.vue 中
 */

import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import type { FormInstance } from 'element-plus'
import { useUserStore } from '@/stores/user'
import { authService } from '@/services/auth'
import { UserKind, DeviceType } from '@/types'

export function useLoginView() {
  const router = useRouter()
  const route = useRoute()
  const userStore = useUserStore()

  // 状态
  const loginMethod = ref('password')
  const loading = ref(false)
  const show2faDialog = ref(false)
  const tempToken = ref('')
  const faCode = ref('')

  // 计算属性
  const userKind = computed(() => route.params.userKind as UserKind)

  const subjectName = computed(() => {
    const names: Record<UserKind, string> = {
      [UserKind.Member]: '会员',
      [UserKind.Community]: '社区运营',
      [UserKind.Platform]: '平台运营'
    }
    return names[userKind.value] || '用户'
  })

  // 表单引用
  const passwordFormRef = ref<FormInstance>()
  const passwordForm = ref({
    username: '',
    password: '',
    device: DeviceType.Web
  })

  /**
   * 密码登录 - 使用服务层
   */
  async function handlePasswordLogin() {
    if (!passwordFormRef.value) return
    
    await passwordFormRef.value.validate(async (valid) => {
      if (!valid) return
      
      loading.value = true
      try {
        const result = await authService.loginWithPassword(
          userKind.value,
          passwordForm.value.username,
          passwordForm.value.password
        )

        if (!result.success) {
          ElMessage.error(result.message)
          return
        }

        // 检查是否需要 2FA
        if (result.requires2FA && result.tempToken) {
          tempToken.value = result.tempToken
          show2faDialog.value = true
          ElMessage.info('请输入 2FA 验证码')
        }
        // 如果不需要 2FA，authService 已经处理了跳转
      } finally {
        loading.value = false
      }
    })
  }

  /**
   * 提交 2FA 验证码 - 使用服务层
   */
  async function handleSubmit2FA() {
    if (!faCode.value || faCode.value.length !== 6) {
      ElMessage.error('请输入6位验证码')
      return
    }

    loading.value = true
    try {
      const result = await authService.verify2FA(
        userKind.value,
        tempToken.value,
        faCode.value
      )

      if (!result.success) {
        ElMessage.error(result.message)
        return
      }

      // 关闭对话框
      show2faDialog.value = false
      faCode.value = ''
      tempToken.value = ''
      // authService 已经处理了跳转
    } finally {
      loading.value = false
    }
  }

  /**
   * Passkey 登录 - 使用服务层
   */
  async function handlePasskeyLogin(username: string) {
    if (!username) {
      ElMessage.error('请输入用户名')
      return
    }

    loading.value = true
    try {
      const result = await authService.loginWithPasskey(
        userKind.value,
        username
      )

      if (!result.success) {
        ElMessage.error(result.message)
      }
      // 如果成功，authService 已经处理了跳转
    } finally {
      loading.value = false
    }
  }

  /**
   * 返回首页
   */
  function goHome() {
    authService.goHome()
  }

  /**
   * 初始化
   */
  function init() {
    onMounted(() => {
      // 验证 userKind 是否有效
      if (!Object.values(UserKind).includes(userKind.value)) {
        ElMessage.error('无效的用户类型')
        router.push('/')
        return
      }
      
      // 如果已经登录，直接跳转到对应的 Dashboard
      if (userStore.isLoggedIn && userStore.userKind === userKind.value) {
        const paths: Record<UserKind, string> = {
          [UserKind.Member]: '/dashboard/member',
          [UserKind.Community]: '/dashboard/community',
          [UserKind.Platform]: '/dashboard/platform'
        }
        router.push(paths[userKind.value])
      }
    })
  }

  return {
    // 状态
    loginMethod,
    loading,
    show2faDialog,
    tempToken,
    faCode,
    userKind,
    subjectName,
    
    // 表单
    passwordFormRef,
    passwordForm,
    
    // 方法
    handlePasswordLogin,
    handleSubmit2FA,
    handlePasskeyLogin,
    goHome,
    init
  }
}
