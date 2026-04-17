import axios, { AxiosInstance, InternalAxiosRequestConfig, AxiosResponse } from 'axios'
import { ElMessage } from 'element-plus'
import type { ApiResponse } from '@/types'

const service: AxiosInstance = axios.create({
  baseURL: '/api',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json'
  }
})

// 请求拦截器
service.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // 从 localStorage 获取 token
    const token = localStorage.getItem('token')
    if (token && config.headers) {
      config.headers.Authorization = `Bearer ${token}`
      config.headers['X-AUTHORIZATION'] = token
    }
    
    // 添加 X-DEVICE 请求头
    const device = localStorage.getItem('device_type') || '1' // 默认为 Web
    if (config.headers) {
      config.headers['X-DEVICE'] = device
      console.log('[Request] URL:', config.url, '| Device:', device)
    }
    
    return config
  },
  (error) => {
    console.error('Request error:', error)
    return Promise.reject(error)
  }
)

// 响应拦截器
service.interceptors.response.use(
  (response: AxiosResponse<ApiResponse>) => {
    const res = response.data
    console.log('[Response] URL:', response.config.url, '| Status:', response.status, '| Data:', res)
    
    // 如果配置了 skipCodeCheck，则直接返回响应（用于 WebAuthn 等特殊 API）
    const skipCodeCheck = (response.config as any).skipCodeCheck
    if (!skipCodeCheck) {
      // 如果返回的状态码不是 0，说明接口有问题
      if (res.code !== 0) {
        ElMessage.error(res.message || '请求失败')
        
        // 401: 未授权，需要重新登录
        if (res.code === 401) {
          const userKind = localStorage.getItem('user_kind')
          localStorage.removeItem('token')
          localStorage.removeItem('user_kind')
          localStorage.removeItem('user_info')
          
          // 根据用户类型跳转到对应的登录页面
          if (userKind) {
            window.location.href = `/#/login/${userKind}`
          } else {
            window.location.href = '/#/'
          }
        }
        
        return Promise.reject(new Error(res.message || '请求失败'))
      }
    }
    
    return response
  },
  (error) => {
    console.error('[Response Error]', error)
    
    // 处理 HTTP 401 状态码
    if (error.response && error.response.status === 401) {
      const userKind = localStorage.getItem('user_kind')
      
      // 清除所有认证信息
      localStorage.removeItem('token')
      localStorage.removeItem('user_kind')
      localStorage.removeItem('user_info')
      localStorage.removeItem('device_type')
      
      console.log('[401 Handler] Cleared auth data, user_kind:', userKind)
      
      // 根据用户类型跳转到对应的登录页面
      if (userKind) {
        window.location.href = `/#/login/${userKind}`
      } else {
        window.location.href = '/#/'
      }
      
      return Promise.reject(error)
    }
    
    if (error.response) {
      // 服务器返回了错误状态码
      console.error('Status:', error.response.status)
      console.error('Data:', error.response.data)
      ElMessage.error(`请求失败: ${error.response.status} ${error.response.statusText}`)
    } else if (error.request) {
      // 请求已发出但没有收到响应
      console.error('No response received. Request:', error.request)
      ElMessage.error('网络错误：无法连接到服务器，请确保后端服务已启动')
    } else {
      // 其他错误
      console.error('Error:', error.message)
      ElMessage.error(error.message || '网络错误')
    }
    return Promise.reject(error)
  }
)

export default service
