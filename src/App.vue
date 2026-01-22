<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AuthForm from './components/AuthForm.vue'
import PersonList from './components/PersonList.vue'
import PersonSearch from './components/PersonSearch.vue'
import AddPersonForm from './components/AddPersonForm.vue'

interface User {
  username: string;
}

const isAuthenticated = ref(false)
const user = ref<User | null>(null)
const refreshTrigger = ref(0)

onMounted(() => {
  const token = localStorage.getItem('token')
  const username = localStorage.getItem('username')
  if (token && username) {
    isAuthenticated.value = true
    user.value = { username }
  }
})

function handleAuthSuccess(data: any) {
  localStorage.setItem('token', data.token)
  localStorage.setItem('username', data.username)
  isAuthenticated.value = true
  user.value = { username: data.username }
}

function handleLogout() {
  localStorage.removeItem('token')
  localStorage.removeItem('username')
  isAuthenticated.value = false
  user.value = null
}

function handlePersonAdded() {
  refreshTrigger.value += 1
}
</script>

<template>
  <div
    class="min-h-screen bg-[#020617] text-slate-200 font-sans selection:bg-emerald-500/30 selection:text-emerald-200">
    <!-- Gradient Backgrounds -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
      <div class="absolute -top-[10%] -left-[10%] w-[40%] h-[40%] bg-emerald-900/10 blur-[120px] rounded-full"></div>
      <div class="absolute top-[20%] -right-[10%] w-[35%] h-[35%] bg-blue-900/10 blur-[120px] rounded-full"></div>
      <div class="absolute -bottom-[10%] left-[20%] w-[45%] h-[45%] bg-purple-900/10 blur-[120px] rounded-full"></div>
    </div>

    <!-- Navigation -->
    <nav class="sticky top-0 z-40 w-full bg-[#020617]/50 backdrop-blur-md border-b border-white/5">
      <div class="w-full px-4 md:px-8 h-16 flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div
            class="h-8 w-8 bg-emerald-500 rounded-lg flex items-center justify-center font-bold text-white shadow-lg shadow-emerald-500/20 shrink-0">
            DB</div>
          <span
            class="text-lg md:text-xl font-bold bg-clip-text text-transparent bg-linear-to-r from-white to-gray-400 truncate">MyDatabase</span>
        </div>

        <div v-if="isAuthenticated" class="flex items-center gap-2 md:gap-4">
          <span class="text-xs md:text-sm font-medium text-gray-400 hidden sm:inline">
            Welcome, <span class="text-white">{{ user?.username }}</span>
          </span>
          <button @click="handleLogout"
            class="text-xs md:text-sm px-3 md:px-4 py-1.5 md:py-2 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 transition-all font-medium">
            Logout
          </button>
        </div>
      </div>
    </nav>

    <main class="relative z-10 w-full px-4 md:px-8 py-8 md:py-12">
      <!-- Auth State -->
      <div v-if="!isAuthenticated" class="flex flex-col items-center justify-center min-h-[70vh] px-4">
        <AuthForm @auth-success="handleAuthSuccess" />
      </div>

      <!-- Dashboard State -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 gap-8">
        <!-- Sidebar / Tools -->
        <div class="md:col-span-1 space-y-8">
          <div class="space-y-4">
            <h2 class="text-lg font-bold text-white flex items-center gap-2">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-emerald-500" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
              Quick Search
            </h2>
            <PersonSearch />
          </div>

          <AddPersonForm @person-added="handlePersonAdded" />
        </div>

        <!-- Main Content -->
        <div class="md:col-span-1 lg:col-span-2 2xl:col-span-3 3xl:col-span-4 space-y-6">
          <PersonList :refresh-trigger="refreshTrigger" />
        </div>
      </div>

    </main>


    <!-- Footer -->
    <footer class="relative z-10 border-t border-white/5 py-8 mt-12 bg-[#020617]/50 backdrop-blur-sm">
      <div class="max-w-7xl mx-auto px-4 text-center text-xs md:text-sm text-gray-600 font-medium">
        &copy; 2026 Tambahkan Ini Ke Database. Built with Vue 3 & Rust Axum.
      </div>
    </footer>

  </div>
</template>

<style>
/* App specific styles */
</style>
