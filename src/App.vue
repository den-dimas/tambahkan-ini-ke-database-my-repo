<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AuthForm from './components/AuthForm.vue'
import PersonList from './components/PersonList.vue'
import PersonSearch from './components/PersonSearch.vue'
import AddPersonForm from './components/AddPersonForm.vue'
import PersonDetail from './components/PersonDetail.vue'
import ApprovalCenter from './components/ApprovalCenter.vue'

interface User {
  username: string;
}

const isAuthenticated = ref(false)
const user = ref<User | null>(null)
const refreshTrigger = ref(0)
const currentView = ref('dashboard') // 'dashboard', 'detail', 'approvals'
const selectedPersonId = ref<string | null>(null)

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
  currentView.value = 'dashboard'
}

function handlePersonAdded() {
  refreshTrigger.value += 1
}

function openPersonDetail(personId: string) {
  selectedPersonId.value = personId
  currentView.value = 'detail'
}

function goToDashboard() {
  currentView.value = 'dashboard'
  selectedPersonId.value = null
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
        <div class="flex items-center gap-2 cursor-pointer" @click="goToDashboard">
          <div
            class="h-8 w-8 bg-emerald-500 rounded-lg flex items-center justify-center font-bold text-white shadow-lg shadow-emerald-500/20 shrink-0">
            W</div>
          <span
            class="text-lg md:text-xl font-bold bg-clip-text text-transparent bg-linear-to-r from-white to-gray-400 truncate">PeopleWiki</span>
        </div>

        <div v-if="isAuthenticated" class="flex items-center gap-4 md:gap-8">
          <div class="hidden md:flex items-center gap-6">
            <button @click="currentView = 'dashboard'"
              :class="currentView === 'dashboard' ? 'text-white' : 'text-gray-400'"
              class="text-sm font-bold hover:text-white transition-colors">My Database</button>
            <button @click="currentView = 'approvals'"
              :class="currentView === 'approvals' ? 'text-white' : 'text-gray-400'"
              class="text-sm font-bold hover:text-white transition-colors flex items-center gap-2">
              Approval Center
              <span class="w-2 h-2 bg-emerald-500 rounded-full animate-pulse"></span>
            </button>
          </div>

          <div class="flex items-center gap-2 md:gap-4">
            <span class="text-xs md:text-sm font-medium text-gray-400 hidden sm:inline">
              <span class="text-white">{{ user?.username }}</span>
            </span>
            <button @click="handleLogout"
              class="text-xs md:text-sm px-3 md:px-4 py-1.5 md:py-2 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 transition-all font-medium">
              Logout
            </button>
          </div>
        </div>
      </div>
    </nav>

    <main class="relative z-10 w-full px-4 md:px-8 py-8 md:py-12">
      <!-- Auth State -->
      <div v-if="!isAuthenticated" class="flex flex-col items-center justify-center min-h-[70vh] px-4">
        <AuthForm @auth-success="handleAuthSuccess" />
      </div>

      <!-- Detail View (Full Screen Overlay Style) -->
      <div v-else-if="currentView === 'detail' && selectedPersonId">
        <PersonDetail :person-id="selectedPersonId" @back="goToDashboard" />
      </div>

      <!-- Dashboard View -->
      <div v-else-if="currentView === 'dashboard'"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 gap-8">
        <!-- Sidebar / Tools -->
        <div class="md:col-span-1 space-y-8">
          <div class="space-y-4">
            <h2 class="text-lg font-bold text-white flex items-center gap-2">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-emerald-500" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
              Wikipedia Search
            </h2>
            <PersonSearch @person-selected="openPersonDetail" />
          </div>

          <AddPersonForm @person-added="handlePersonAdded" />
        </div>

        <!-- Main Content -->
        <div class="md:col-span-1 lg:col-span-2 2xl:col-span-3 3xl:col-span-4 space-y-6">
          <PersonList :refresh-trigger="refreshTrigger" @person-selected="openPersonDetail" />
        </div>
      </div>

      <!-- Approvals View -->
      <div v-else-if="currentView === 'approvals'" class="max-w-4xl mx-auto">
        <ApprovalCenter />
      </div>

    </main>

    <!-- Footer -->
    <footer v-if="currentView !== 'detail'"
      class="relative z-10 border-t border-white/5 py-8 mt-12 bg-[#020617]/50 backdrop-blur-sm">
      <div class="max-w-7xl mx-auto px-4 text-center text-xs md:text-sm text-gray-600 font-medium">
        &copy; 2026 PeopleWiki • Community Managed Database
      </div>
    </footer>

  </div>
</template>

<style>
/* App specific styles */
</style>
