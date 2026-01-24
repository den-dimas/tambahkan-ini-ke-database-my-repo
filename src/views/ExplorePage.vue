<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { API_BASE_URL } from '../config/api'

interface PersonSearchResult {
  person_id: string;
  name: string;
  tracking_id?: string;
  category?: string;
  description?: string;
  global_description?: string;
  age?: number;
  gender?: string;
  image_url?: string;
}

const people = ref<PersonSearchResult[]>([])
const loading = ref(false)

async function fetchFeed() {
  loading.value = true
  try {
    const token = localStorage.getItem('token')
    const headers: HeadersInit = {}
    if (token) {
      headers['Authorization'] = `Bearer ${token}`
    }

    const response = await fetch(`${API_BASE_URL}/people/feed`, {
      headers
    })
    const data = await response.json()
    if (data.success) {
      people.value = data.data
    }
  } catch (err) {
    console.error('Failed to fetch feed:', err)
  } finally {
    loading.value = false
  }
}

function getFullImageUrl(url?: string) {
  if (!url) return 'https://ui-avatars.com/api/?name=User&background=10b981&color=fff'
  if (url.startsWith('http')) return url
  return url
}

onMounted(() => {
  fetchFeed()
})
</script>

<template>
  <div class="h-screen w-full overflow-y-scroll snap-y snap-mandatory bg-void-charcoal">

    <!-- Loading State -->
    <div v-if="loading && people.length === 0" class="h-full flex flex-col items-center justify-center gap-4">
      <div class="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-hyper-lime"></div>
      <p class="font-typewriter text-digital-lavender animate-pulse">LOADING LORE DATABASE...</p>
    </div>

    <div v-for="person in people" :key="person.person_id"
      class="h-full w-full snap-start relative flex items-center justify-center p-4 pb-20">
      <!-- Added padding bottom for nav -->

      <!-- Background Blur (Atmosphere) -->
      <div class="absolute inset-0 z-0 overflow-hidden">
        <img :src="getFullImageUrl(person.image_url)"
          class="w-full h-full object-cover blur-[100px] opacity-20 scale-150" alt="bg" />
      </div>

      <!-- Neo-Brutalist Card -->
      <div class="relative z-10 w-full max-w-md h-[75vh] card-brutalist flex flex-col overflow-hidden">

        <!-- Image Section -->
        <div class="relative h-[65%] w-full border-b-2 border-analog-cream overflow-hidden bg-black">
          <img :src="getFullImageUrl(person.image_url)" class="w-full h-full object-cover" :alt="person.name" />

          <!-- Floating Tags -->
          <div class="absolute top-4 left-4 flex flex-wrap gap-2">
            <span v-if="person.category"
              class="px-3 py-1 bg-hyper-lime text-black border-2 border-black font-bold font-typewriter uppercase text-sm shadow-[4px_4px_0px_rgba(0,0,0,0.5)]">
              {{ person.category }}
            </span>
            <span v-if="person.age"
              class="px-3 py-1 bg-analog-cream text-black border-2 border-black font-bold font-typewriter uppercase text-sm shadow-[4px_4px_0px_rgba(0,0,0,0.5)]">
              {{ person.age }} Y.O
            </span>
          </div>
        </div>

        <!-- Content Stats (Paper Style) -->
        <div class="flex-1 bg-analog-cream p-6 flex flex-col justify-between relative">
          <!-- Paper Texture Overlay -->
          <div class="absolute inset-0 opacity-10 pointer-events-none"
            style="background-image: url('data:image/svg+xml,%3Csvg width=\'20\' height=\'20\' viewBox=\'0 0 20 20\' xmlns=\'http://www.w3.org/2000/svg\'%3E%3Cg fill=\'%23000000\' fill-opacity=\'1\' fill-rule=\'evenodd\'%3E%3Ccircle cx=\'3\' cy=\'3\' r=\'1\'/%3E%3Ccircle cx=\'13\' cy=\'13\' r=\'1\'/%3E%3C/g%3E%3C/svg%3E');">
          </div>

          <div>
            <h2 class="text-4xl text-deep-earth mb-2 uppercase leading-[0.9] text-wrap warp-break-word">
              {{ person.name }}
            </h2>

            <div class="w-16 h-1 bg-radical-pink mb-4"></div>

            <p class="font-typewriter text-sm text-deep-earth leading-relaxed line-clamp-4">
              {{ person.description || person.global_description || 'NO ADDITIONAL LORE FOUND.' }}
            </p>
          </div>

          <!-- Action Bar -->
          <div class="flex justify-between items-center pt-4 border-t-2 border-[rgba(44,37,32,0.1)]">
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded-full bg-green-500 animate-pulse"></div>
              <span class="font-typewriter text-xs font-bold text-deep-earth">ONLINE NOW</span>
            </div>
            <button
              class="bg-void-charcoal text-hyper-lime px-4 py-2 font-bold uppercase text-sm border-2 border-transparent hover:border-black transition-all">
              View Full Lore
            </button>
          </div>
        </div>

      </div>

    </div>
  </div>
</template>

<style scoped>
/* Hidden scrollbar */
div::-webkit-scrollbar {
  display: none;
}

div {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
