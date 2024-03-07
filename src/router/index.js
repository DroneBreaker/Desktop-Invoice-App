import { createRouter, createWebHistory } from 'vue-router'
import Intro from '../views/Intro.vue'
import Login from '../views/Login.vue'

const routes = [
  {
    path: '',
    name: 'intro',
    component: Intro
  },

  {
    path: '/login',
    name: 'login',
    component: Login
  },

  {
    path: '/register',
    name: 'register',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "register" */ '../views/Register.vue')
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
