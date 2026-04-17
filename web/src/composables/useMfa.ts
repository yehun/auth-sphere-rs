import { ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { UserKind } from '@/types'
import { generateMfaQRCode, activateMfa, deactivateMfa } from '@/api/auth'

/**
 * MFA 管理 Composable
 */
export function useMfa(userKind: UserKind, onStatusChange?: (enabled: boolean) => void) {
  const mfaLoading = ref(false)
  const mfaEnabled = ref(false)

  /**
   * MFA 开关变化处理
   */
  async function handleMfaChange(enabled: boolean) {
    if (enabled) {
      await handleEnableMfa()
    } else {
      await handleDisableMfa()
    }
  }

  /**
   * 开启 MFA
   */
  async function handleEnableMfa() {
    try {
      mfaLoading.value = true
      
      const response = await generateMfaQRCode(userKind)
      
      if (response.code !== 200 && response.code !== 0) {
        ElMessage.error(response.message || '生成 MFA 二维码失败')
        mfaEnabled.value = false
        return
      }
      
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
      
      // TODO: 应该让用户输入 OTP 验证码来确认
      await activateMfa(userKind, '')
      
      mfaEnabled.value = true
      ElMessage.success('MFA 已开启')
      
      // 通知状态变化
      if (onStatusChange) {
        onStatusChange(true)
      }
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

  /**
   * 关闭 MFA
   */
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
      const response = await deactivateMfa(userKind)
      
      if (response.code !== 200 && response.code !== 0) {
        ElMessage.error(response.message || '关闭 MFA 失败')
        mfaEnabled.value = true
        return
      }
      
      mfaEnabled.value = false
      ElMessage.success('MFA 已关闭')
      
      // 通知状态变化
      if (onStatusChange) {
        onStatusChange(false)
      }
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

  return {
    mfaLoading,
    mfaEnabled,
    handleMfaChange,
    handleEnableMfa,
    handleDisableMfa
  }
}
