<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { authService } from '../services/authService'
import { formatError } from '../utils'

const router = useRouter()
const authStore = useAuthStore()

const isLogin = ref(true)
const username = ref('')
const password = ref('')
const error = ref('')
const loading = ref(false)

async function handleSubmit() {
  error.value = ''
  loading.value = true

  // Client-side validation
  if (!isLogin.value) {
    if (username.value.length < 3) {
      error.value = 'Username must be at least 3 characters long'
      loading.value = false
      return
    }
    if (password.value.length < 8) {
      error.value = 'Password must be at least 8 characters long'
      loading.value = false
      return
    }
  }

  try {
    let response
    if (isLogin.value) {
      response = await authService.login({
        username: username.value,
        password: password.value,
      })
    } else {
      response = await authService.register({
        username: username.value,
        password: password.value,
      })
    }

    if (response) {
      // Assuming response.data contains { token, username } directly or wrapped
      // Check api response structure: ApiResponse { success: true, data: { token, username } }
      // The service returns `data` from axios response, which is the whole ApiResponse json
      if (response.success && response.data) {
        authStore.setAuth(response.data)
        router.push('/')
      } else {
        throw new Error(response.error || 'Authentication failed')
      }
    }
  } catch (err: any) {
    error.value = formatError(err)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-[85vh] flex items-center justify-center p-4">
    <div
      class="w-full max-w-md mx-auto p-8 rounded-card bg-void-charcoal/80 border-2 border-analog-cream shadow-hard backdrop-blur-xl flex flex-col items-center">

      <div class="mb-8 p-4 rounded-full bg-hyper-lime/10 text-hyper-lime border-2 border-hyper-lime box-content">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"
            v-if="!isLogin" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
            v-else />
        </svg>
      </div>

      <h2 class="text-3xl font-extrabold text-analog-cream mb-2 text-center tracking-tight font-header">
        {{ isLogin ? 'WELCOME BACK' : 'JOIN US' }}
      </h2>
      <p class="text-analog-cream/60 text-sm mb-8 text-center font-typewriter">
        {{ isLogin ? 'Access your collection' : 'Start tracking people' }}
      </p>

      <form @submit.prevent="handleSubmit" class="w-full flex flex-col gap-5">
        <div class="flex flex-col gap-1.5">
          <label
            class="block text-xs font-bold uppercase tracking-wider text-analog-cream/80 ml-1 font-typewriter">Username</label>
          <input v-model="username" type="text" required
            class="w-full px-4 py-3.5 rounded-xl bg-black/40 border-2 border-analog-cream/20 text-analog-cream focus:outline-none focus:border-hyper-lime focus:ring-0 transition-all text-sm md:text-base placeholder-gray-600 font-bold"
            placeholder="ENTER USERNAME" />
        </div>

        <div class="flex flex-col gap-1.5">
          <label
            class="block text-xs font-bold uppercase tracking-wider text-analog-cream/80 ml-1 font-typewriter">Password</label>
          <input v-model="password" type="password" required
            class="w-full px-4 py-3.5 rounded-xl bg-black/40 border-2 border-analog-cream/20 text-analog-cream focus:outline-none focus:border-hyper-lime focus:ring-0 transition-all text-sm md:text-base placeholder-gray-600 font-bold"
            placeholder="••••••••" />
        </div>

        <p v-if="error"
          class="text-radical-pink text-sm animate-pulse text-center bg-radical-pink/10 py-2 rounded-lg font-bold border border-radical-pink">
          {{ error }}</p>

        <button type="submit" :disabled="loading"
          class="w-full py-4 rounded-xl bg-hyper-lime hover:bg-hyper-lime text-black font-extrabold text-lg tracking-wide transition-all transform hover:translate-x-[2px] hover:translate-y-[2px] active:translate-x-[4px] active:translate-y-[4px] disabled:opacity-50 disabled:cursor-not-allowed border-2 border-hyper-lime shadow-[4px_4px_0px_var(--color-analog-cream)] hover:shadow-[2px_2px_0px_var(--color-analog-cream)] active:shadow-none mt-4 uppercase">
          <span v-if="loading" class="flex items-center justify-center">
            <svg class="animate-spin h-5 w-5 mr-3" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none">
              </circle>
              <path class="opacity-75" fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
              </path>
            </svg>
            PROCESSING...
          </span>
          <span v-else>{{ isLogin ? 'SIGN IN' : 'CREATE ACCOUNT' }}</span>
        </button>
      </form>

      <div class="mt-8 pt-6 border-t-2 border-analog-cream/10 w-full text-center">
        <p class="text-analog-cream/60 text-sm font-typewriter">
          {{ isLogin ? "New here?" : "Have an account?" }}
          <button @click="isLogin = !isLogin"
            class="text-hyper-lime hover:text-white font-bold ml-1 transition-colors uppercase decoration-2 underline-offset-4 hover:underline">
            {{ isLogin ? 'Join Us' : 'Sign In' }}
          </button>
        </p>
      </div>
    </div>
  </div>
</template>
