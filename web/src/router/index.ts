import {createRouter, createWebHashHistory} from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
// import { ElMessage } from 'element-plus'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/HomeView.vue')
  },
  {
    path: '/login/:userKind',
    name: 'Login',
    component: () => import('@/views/LoginView.vue'),
    props: true
  },
  {
    path: '/register',
    name: 'Register',
    component: () => import('@/views/RegisterView.vue')
  },
  {
    path: '/dashboard/member',
    name: 'MemberDashboard',
    component: () => import('@/views/MemberDashboard.vue'),
    meta: { requiresAuth: true, userKind: 'member' }
  },
  {
    path: '/dashboard/community',
    name: 'CommunityDashboard',
    component: () => import('@/views/CommunityDashboard.vue'),
    meta: { requiresAuth: true, userKind: 'community' }
  },
  {
    path: '/dashboard/platform',
    name: 'PlatformDashboard',
    component: () => import('@/views/PlatformDashboard.vue'),
    meta: { requiresAuth: true, userKind: 'platform' }
  }
]

const router = createRouter({
  // history: createWebHistory(),
  history: createWebHashHistory("/"),
  routes
})

// // 路由守卫
// router.beforeEach((to, _from, next) => {
//   const token = localStorage.getItem('access_token')
//   const userKind = localStorage.getItem('user_kind')
//
//   if (to.meta.requiresAuth && !token) {
//     // 需要登录但未登录，跳转到首页
//     next('/')
//   } else if (to.path === '/' && token) {
//     // 已登录用户访问首页，重定向到对应的 Dashboard
//     if (userKind === 'member') {
//       next('/dashboard/member')
//     } else if (userKind === 'community') {
//       next('/dashboard/community')
//     } else if (userKind === 'platform') {
//       next('/dashboard/platform')
//     } else {
//       next('/')
//     }
//   } else if (to.meta.userKind && userKind !== to.meta.userKind) {
//     // 用户类型不匹配，拒绝访问
//     ElMessage.warning('无权访问该页面')
//     next(false)
//   } else {
//     next()
//   }
// })

export default router
