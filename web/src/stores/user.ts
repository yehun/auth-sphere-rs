import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { UserInfo, LoginResponse } from '@/types'
import { UserKind } from '@/types'
import { logout as apiLogout } from '@/api/auth'

export const useUserStore = defineStore('user', () => {
  const userInfo = ref<UserInfo | null>(null)
  const token = ref<string>('')
  const userKind = ref<UserKind | null>(null)

  const isLoggedIn = computed(() => !!token.value && !!userInfo.value)

  // 设置登录信息
  function setLoginInfo(data: LoginResponse, kind: UserKind) {
    console.log('[UserStore] setLoginInfo called with:')
    console.log('  data:', data)
    console.log('  data.token:', data.token)
    console.log('  data.user_info:', data.user_info)
    console.log('  kind:', kind)
    
    token.value = data.token
    userInfo.value = data.user_info
    userKind.value = kind
    
    // 保存到 localStorage
    localStorage.setItem('token', token.value)
    localStorage.setItem('user_kind', kind)
    localStorage.setItem('user_info', JSON.stringify(userInfo.value))
    
    console.log('[UserStore] Login info saved:')
    console.log('  token:', token.value ? token.value.substring(0, 20) + '...' : 'empty')
    console.log('  user_kind:', kind)
    console.log('  user_info:', userInfo.value)
    console.log('  isLoggedIn:', !!token.value && !!userInfo.value)
  }

  // 从 localStorage 恢复登录状态
  function restoreFromStorage() {
    const tok = localStorage.getItem('token')
    const kind = localStorage.getItem('user_kind')
    const info = localStorage.getItem('user_info')
    
    console.log('[UserStore] Restoring from storage:')
    console.log('  token:', tok ? 'exists' : 'missing')
    console.log('  user_kind:', kind)
    console.log('  user_info:', info)
    
    if (tok && kind && info && info !== 'undefined' && info !== 'null') {
      try {
        token.value = tok
        userKind.value = kind as UserKind
        userInfo.value = JSON.parse(info)
        console.log('[UserStore] Restore successful, isLoggedIn:', !!token.value && !!userInfo.value)
        return true
      } catch (error) {
        console.error('[UserStore] Failed to parse user_info:', error)
        // 清除无效数据
        clearStorage()
        return false
      }
    }
    console.log('[UserStore] Restore failed - missing or invalid data')
    return false
  }

  // 登出
  async function logout() {
    try {
      if (userKind.value) {
        await apiLogout(userKind.value)
      }
    } catch (error) {
      console.error('Logout error:', error)
    } finally {
      // 清除本地存储
      clearStorage()
    }
  }

  // 清除存储
  function clearStorage() {
    token.value = ''
    userInfo.value = null
    userKind.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user_kind')
    localStorage.removeItem('user_info')
  }

  return {
    userInfo,
    token,
    userKind,
    isLoggedIn,
    setLoginInfo,
    restoreFromStorage,
    logout,
    clearStorage
  }
})
