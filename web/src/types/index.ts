// 用户类型枚举
export enum UserKind {
  Member = 'member',
  Community = 'community',
  Platform = 'platform'
}

// 设备类型枚举
export enum DeviceType {
  Web = '1',
  Android = '2',
  Ios = '3',
  Desktop = '4',
  Unknown = '0'
}

// 登录方式枚举
export enum LoginMethod {
  Password = 'password',
  OTP = 'otp',
  Passkey = 'passkey'
}

// 用户信息
export interface UserInfo {
  id: number
  username: string
  nickname: string
  user_type: UserKind
  is_mfa: boolean
}

// 登录响应
export interface LoginResponse {
  token: string
  expires_in: number
  user_info: UserInfo
}

// 2FA 验证响应
export interface TwoFAResponse {
  requires_2fa: boolean
  token: string
}

// OTP 发送响应
export interface OtpSendResponse {
  success: boolean
  message: string
  expire_in: number
}

// 注册请求
export interface RegisterRequest {
  nickname: string
  username: string
  password: string
  user_type: string
  email?: string
  phone?: string
}

// 密码登录请求
export interface PasswordLoginRequest {
  username: string
  password: string
  device?: string
}

// OTP 登录请求
export interface OtpLoginRequest {
  contact: string
  otp_code: string
}

// API 响应
export interface ApiResponse<T = any> {
  code: number
  message: string
  data: T
}
