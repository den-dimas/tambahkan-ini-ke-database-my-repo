<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { API_BASE_URL } from '../config/api'

const currentCategory = ref('KISAH')
const categories = ['KISAH', 'BINI', 'JANDA']
let currentIndex = 0
let typeInterval: any = null

const stats = ref<{
  total_people: number;
  total_edits: number;
  pending_edits: number;
} | null>(null)

async function fetchStats() {
  try {
    const token = localStorage.getItem('token')
    const headers: HeadersInit = {}
    if (token) headers['Authorization'] = `Bearer ${token}`

    const response = await fetch(`${API_BASE_URL}/stats`, { headers })
    const data = await response.json()
    if (data.success) {
      stats.value = data.data
    }
  } catch (err) {
    console.error('Failed to fetch stats:', err)
  }
}

onMounted(() => {
  typeInterval = setInterval(() => {
    currentIndex = (currentIndex + 1) % categories.length
    currentCategory.value = categories[currentIndex] ?? 'KISAH'
  }, 2000)

  fetchStats()
})

onUnmounted(() => {
  if (typeInterval) clearInterval(typeInterval)
})
</script>

<template>
  <div class="min-h-[85vh] flex flex-col items-center justify-center text-center px-4 relative overflow-hidden">

    <!-- Hero Section -->
    <div class="relative z-10 max-w-5xl mx-auto flex flex-col gap-12">
      <div class="flex flex-col gap-6">
        <h1 class="text-5xl md:text-7xl lg:text-9xl font-extrabold tracking-tighter uppercase leading-[0.9]">
          <span class="block text-analog-cream mb-2">Tambahkan Ini</span>
          <span class="block text-analog-cream">Ke Database My</span>

          <div class="mt-8 flex items-center justify-center">
            <span
              class="relative inline-block px-4 py-2 bg-hyper-lime text-black -rotate-2 border-2 border-white shadow-[8px_8px_0px_#000]">
              <span class="font-typewriter text-4xl md:text-6xl font-bold">{{ currentCategory }}</span>
            </span>
          </div>
        </h1>

        <p
          class="font-typewriter text-digital-lavender text-sm md:text-lg max-w-2xl mx-auto leading-relaxed tracking-wide uppercase mt-8">
          The crowdsourced database for your relationships. <br />
          <span class="bg-radical-pink text-black px-1 font-bold">NO ALGORITHMS.</span>
          <span class="bg-analog-cream text-black px-1 font-bold ml-1">JUST LORE.</span>
        </p>
      </div>

      <div class="flex flex-col sm:flex-row items-center justify-center gap-6 pt-8">
        <router-link to="/explore" class="btn-primary text-xl">
          Start Exploring
        </router-link>

        <router-link to="/search" class="btn-secondary text-xl">
          Search Database
        </router-link>
      </div>
    </div>

    <!-- Stats Ticker (Bottom) -->
    <div class="absolute bottom-10 left-0 w-full overflow-hidden border-y-2 border-analog-cream bg-void-charcoal py-3">
      <div
        class="whitespace-nowrap animate-marquee font-typewriter text-hyper-lime text-sm font-bold uppercase tracking-widest flex gap-8">
        <template v-if="stats">
          <span>TOTAL PEOPLE: {{ stats.total_people }}</span> •
          <span>TOTAL EDITS: {{ stats.total_edits }}</span> •
          <span>PENDING APPROVALS: {{ stats.pending_edits }}</span> •
          <span>RAW RESONANCE</span> •
          <span>EST 2026</span> •
          <span>NO GHOSTING</span> •
          <span>TOTAL PEOPLE: {{ stats.total_people }}</span> •
          <span>TOTAL EDITS: {{ stats.total_edits }}</span> •
          <span>PENDING APPROVALS: {{ stats.pending_edits }}</span>
        </template>
        <template v-else>
          RAW RESONANCE • EST 2026 • NO GHOSTING • CALM UI • TYPEWRITINGS • RAW RESONANCE • EST 2026 • NO GHOSTING •
          CALM UI • TYPEWRITINGS
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
.animate-marquee {
  animation: marquee 20s linear infinite;
}

@keyframes marquee {
  0% {
    transform: translateX(0);
  }

  100% {
    transform: translateX(-50%);
  }
}
</style>
