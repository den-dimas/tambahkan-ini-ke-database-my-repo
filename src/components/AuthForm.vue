<script setup lang="ts">
import { ref } from 'vue'
import { API_BASE_URL } from '../config/api'

const isLogin = ref(true)
const username = ref('')
const password = ref('')
const error = ref('')
const loading = ref(false)

const emit = defineEmits(['auth-success'])

async function handleSubmit() {
  error.value = ''
  loading.value = true

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

  const endpoint = isLogin.value ? `${API_BASE_URL}/auth/login` : `${API_BASE_URL}/auth/register`

  try {
    const response = await fetch(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        username: username.value,
        password: password.value,
      }),
    })

    const data = await response.json()

    if (!data.success) {
      throw new Error(data.error || 'Authentication failed')
    }

    emit('auth-success', data.data)
  } catch (err: any) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div
    class="w-full max-w-md mx-auto p-6 sm:p-8 md:p-10 rounded-2xl md:rounded-3xl bg-white/10 backdrop-blur-xl border border-white/20 shadow-2xl flex flex-col items-center transition-all duration-300">
    <h2 class="text-2xl sm:text-3xl font-bold text-white mb-6 md:mb-8 text-center">
      {{ isLogin ? 'Welcome Back' : 'Join Us' }}
    </h2>

    <form @submit.prevent="handleSubmit" class="w-full space-y-5 md:space-y-6">
      <div class="space-y-1.5">
        <label class="block text-sm font-medium text-gray-300 ml-1">Username</label>
        <input v-model="username" type="text" required
          class="w-full px-4 py-3.5 rounded-xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all text-sm md:text-base placeholder-gray-500"
          placeholder="Enter username" />
      </div>

      <div class="space-y-1.5">
        <label class="block text-sm font-medium text-gray-300 ml-1">Password</label>
        <input v-model="password" type="password" required
          class="w-full px-4 py-3.5 rounded-xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all text-sm md:text-base placeholder-gray-500"
          placeholder="••••••••" />
      </div>

      <p v-if="error" class="text-red-400 text-sm animate-pulse text-center">{{ error }}</p>

      <button type="submit" :disabled="loading"
        class="w-full py-3.5 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold transition-all transform hover:scale-[1.01] active:scale-[0.99] disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-emerald-900/40 text-sm md:text-base mt-2">
        <span v-if="loading" class="flex items-center justify-center">
          <svg class="animate-spin h-5 w-5 mr-3" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none">
            </circle>
            <path class="opacity-75" fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
            </path>
          </svg>
          Processing...
        </span>
        <span v-else>{{ isLogin ? 'Sign In' : 'Create Account' }}</span>
      </button>
    </form>

    <p class="mt-8 text-gray-400 text-sm text-center">
      {{ isLogin ? "Don't have an account?" : "Already have an account?" }}
      <button @click="isLogin = !isLogin"
        class="text-emerald-400 hover:text-emerald-300 font-semibold ml-1 transition-colors underline-offset-4 hover:underline">
        {{ isLogin ? 'Sign Up' : 'Sign In' }}
      </button>
    </p>
  </div>

</template>
