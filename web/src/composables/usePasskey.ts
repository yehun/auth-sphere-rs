import { ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { UserKind } from '@/types'
import { passkeyRegisterBegin, passkeyRegisterComplete, deactivatePasskey } from '@/api/auth'
import { startRegistration } from '@simplewebauthn/browser'

/**
 * Passkey 管理 Composable
 */
export function usePasskey(userKind: UserKind, onStatusChange?: (enabled: boolean) => void) {
  const passkeyLoading = ref(false)
  const hasPasskey = ref(false)

  /**
   * 注册 Passkey
   */
  async function handleRegisterPasskey(username: string) {
    if (!username) {
      ElMessage.error('用户名不能为空')
      return
    }
    
    try {
      passkeyLoading.value = true
      
      // 1. 开始 Passkey 注册流程
      const response = await passkeyRegisterBegin(userKind, { username })
      
      console.log('[Passkey] Registration challenge received:', response)
      
      // webauthn-rs 返回的格式是 { publicKey: {...} }
      const options = response.publicKey || response
      
      console.log('[Passkey] Using options:', options)
      
      if (!options || !options.challenge) {
        console.error('[Passkey] Invalid challenge object:', options)
        ElMessage.error('获取挑战失败，请重试')
        return
      }
      
      // 2. 调用浏览器 WebAuthn API 进行注册
      const credential = await startRegistration(options)
      console.log('[Passkey] Registration completed:', credential)
      
      // 3. 完成 Passkey 注册
      const completeResponse = await passkeyRegisterComplete(userKind, {
        username,
        credential
      })
      
      if (completeResponse.code !== 200 && completeResponse.code !== 0) {
        ElMessage.error(completeResponse.message || 'Passkey 注册失败')
        return
      }
      
      hasPasskey.value = true
      ElMessage.success('Passkey 注册成功！现在您可以使用 Passkey 无密码登录了')
      
      // 通知状态变化
      if (onStatusChange) {
        onStatusChange(true)
      }
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

  /**
   * 关闭 Passkey
   */
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
      const response = await deactivatePasskey(userKind)
      
      if (response.code !== 200 && response.code !== 0) {
        ElMessage.error(response.message || '关闭 Passkey 失败')
        return
      }
      
      hasPasskey.value = false
      ElMessage.success('Passkey 已关闭')
      
      // 通知状态变化
      if (onStatusChange) {
        onStatusChange(false)
      }
    } catch (error: any) {
      if (error !== 'cancel') {
        console.error('Deactivate Passkey error:', error)
        ElMessage.error(error.message || '关闭 Passkey 失败')
      }
    } finally {
      passkeyLoading.value = false
    }
  }

  return {
    passkeyLoading,
    hasPasskey,
    handleRegisterPasskey,
    handleDeactivatePasskey
  }
}
