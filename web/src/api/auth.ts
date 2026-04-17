import request from '@/utils/request'
import type {
  LoginResponse,
  TwoFAResponse,
  OtpSendResponse,
  RegisterRequest,
  PasswordLoginRequest,
  OtpLoginRequest,
  UserInfo
} from '@/types'
import { UserKind } from '@/types'

/**
 * 密码登录
 */
export function passwordLogin(
  userKind: UserKind,
  data: PasswordLoginRequest
): Promise<{ code: number; message: string; data: LoginResponse | TwoFAResponse }> {
  return request({
    url: `/${userKind}/login`,
    method: 'post',
    data
  }).then((res) => res.data)
}

/**
 * 2FA 验证
 */
export function verify2FA(
  userKind: UserKind,
  data: { token: string; code: string }
): Promise<{ code: number; message: string; data: LoginResponse }> {
  return request({
    url: `/${userKind}/login/2fa`,
    method: 'post',
    data
  }).then((res) => res.data)
}

/**
 * OTP 登录
 */
export function otpLogin(
  userKind: UserKind,
  data: OtpLoginRequest
): Promise<{ code: number; message: string; data: LoginResponse }> {
  return request({
    url: `/${userKind}/otp/login`,
    method: 'post',
    data
  }).then((res) => res.data)
}

/**
 * 发送 OTP 验证码
 */
export function sendOtp(
  userKind: UserKind,
  data: { email?: string; phone?: string }
): Promise<{ code: number; message: string; data: OtpSendResponse }> {
  return request({
    url: `/${userKind}/otp/send`,
    method: 'post',
    data
  }).then((res) => res.data)
}

/**
 * 注册
 */
export function register(
  data: RegisterRequest
): Promise<{ code: number; message: string; data: any }> {
  return request({
    url: `/${data.user_type}/register`,
    method: 'post',
    data
  }).then((res) => res.data)
}

/**
 * 登出
 */
export function logout(userKind: UserKind): Promise<{ code: number; message: string; data: any }> {
  return request({
    url: `/${userKind}/logout`,
    method: 'post'
  }).then((res) => res.data)
}

/**
 * 获取当前用户信息
 */
export function getCurrentUser(
  userKind: UserKind
): Promise<{ code: number; message: string; data: UserInfo }> {
  return request({
    url: `/${userKind}/info`,
    method: 'get'
  }).then((res) => res.data)
}

// export function getMemberInfo(): Promise<{ code: number; message: string; data: UserInfo }> {
//   return request({
//     url: '/member/info',
//     method: 'get'
//   }).then((res) => res.data)
// }

/**
 * 刷新 Token
 */
export function refreshToken(
  userKind: UserKind,
  refresh_token: string
): Promise<{ code: number; message: string; data: LoginResponse }> {
  return request({
    url: `/${userKind}/refresh`,
    method: 'post',
    data: { refresh_token }
  }).then((res) => res.data)
}

/**
 * 生成 MFA 二维码
 */
export function generateMfaQRCode(): Promise<{
  code: number;
  message: string;
  data: { qr_code: string; secret: string, uri: string }
}> {
  return request({
    url: '/member/mfa/generate',
    method: 'post'
  }).then((res) => res.data)
}

/**
 * 激活 MFA
 */
export function activateMfa(otp_code: string): Promise<{ code: number; message: string; data: any }> {
  return request({
    url: '/member/mfa/active',
    method: 'post',
    data: { otp_code }
  }).then((res) => res.data)
}

/**
 * 停用 MFA
 */
export function deactivateMfa(): Promise<{ code: number; message: string; data: any }> {
  return request({
    url: '/member/mfa/deactive',
    method: 'post'
  }).then((res) => res.data)
}

/**
 * Passkey 注册 - 开始
 */
export function passkeyRegisterBegin(
  userKind: UserKind,
  data: { username: string }
): Promise<any> {
  // @ts-ignore - 自定义配置项，用于跳过响应 code 检查
  return request({
    url: `/${userKind}/passkey/register/begin`,
    method: 'post',
    data,
    skipCodeCheck: true
  }).then((res) => res.data)
}

/**
 * Passkey 注册 - 完成
 */
export function passkeyRegisterComplete(
  userKind: UserKind,
  data: { username: string; credential: any }
): Promise<{ code: number; message: string; data: any }> {
  return request({
    url: `/${userKind}/passkey/register/complete`,
    method: 'post',
    data
  }).then((res) => res.data)
}

/**
 * Passkey 登录 - 开始
 */
export function passkeyLoginBegin(
  userKind: UserKind,
  data: { username: string }
): Promise<any> {
  // @ts-ignore - 自定义配置项，用于跳过响应 code 检查
  return request({
    url: `/${userKind}/passkey/login/begin`,
    method: 'post',
    data,
    skipCodeCheck: true
  }).then((res) => res.data)
}

/**
 * Passkey 登录 - 完成
 */
export function passkeyLoginComplete(
  userKind: UserKind,
  data: { username: string; credential: any }
): Promise<{ code: number; message: string; data: any }> {
  return request({
    url: `/${userKind}/passkey/login/complete`,
    method: 'post',
    data
  }).then((res) => res.data)
}

/**
 * 关闭 Passkey
 */
export function deactivatePasskey(userKind: UserKind): Promise<{ code: number; message: string }> {
  return request({
    url: `/${userKind}/passkey/deactive`,
    method: 'post'
  }).then((res) => res.data)
}
