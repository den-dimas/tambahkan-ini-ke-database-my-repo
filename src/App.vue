<script setup lang="ts">
import { useAuthStore } from './stores/auth'

const authStore = useAuthStore()

function handleLogout() {
  authStore.logout()
  // Refresh page or router push
  window.location.reload()
}
</script>

<template>
  <div class="min-h-screen bg-void-charcoal text-analog-cream font-sans selection:bg-hyper-lime selection:text-black">

    <!-- Main Content Area -->
    <main class="relative z-10 w-full pb-32"> <!-- Padding bottom for floating dock -->
      <router-view></router-view>
    </main>

    <!-- Floating Dock Navigation (Bottom) -->
    <nav class="fixed bottom-8 left-1/2 -translate-x-1/2 z-50">
      <div
        class="flex items-center gap-2 p-2 bg-[rgba(26,26,29,0.8)] backdrop-blur-xl border-2 border-analog-cream rounded-full shadow-[8px_8px_0px_#000]">

        <router-link to="/" class="p-3 rounded-full transition-all hover:bg-analog-cream hover:text-black group"
          active-class="bg-hyper-lime text-black font-bold">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
        </router-link>

        <router-link to="/explore" class="p-3 rounded-full transition-all hover:bg-analog-cream hover:text-black group"
          active-class="bg-hyper-lime text-black font-bold">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </router-link>

        <router-link to="/search" class="p-3 rounded-full transition-all hover:bg-analog-cream hover:text-black group"
          active-class="bg-hyper-lime text-black font-bold">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </router-link>

        <router-link v-if="authStore.isAuthenticated" to="/profile"
          class="p-3 rounded-full transition-all hover:bg-analog-cream hover:text-black group"
          active-class="bg-hyper-lime text-black font-bold">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
        </router-link>

        <!-- Dynamic Login/Logout Button -->
        <router-link v-if="!authStore.isAuthenticated" to="/login"
          class="p-3 rounded-full transition-all hover:bg-analog-cream hover:text-black group"
          active-class="bg-hyper-lime text-black font-bold">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1" />
          </svg>
        </router-link>
        <button v-else @click="handleLogout"
          class="ml-2 p-2 rounded-full border border-radical-pink text-radical-pink hover:bg-radical-pink hover:text-black transition-colors">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
          </svg>
        </button>
      </div>
    </nav>

  </div>
</template>

<style>
/* Global noise texture overlay */
body::before {
  content: "";
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 5;
  opacity: 0.05;
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E");
}
</style>
