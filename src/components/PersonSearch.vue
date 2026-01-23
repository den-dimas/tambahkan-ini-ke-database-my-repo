<script setup lang="ts">
import { ref, watch } from 'vue'

import { API_BASE_URL } from '../config/api'

interface Person {
  id: number;
  name: string;
  category: string;
  description: string;
}

const query = ref('')
const results = ref<Person[]>([])
const loading = ref(false)

async function handleSearch() {
  if (query.value.length < 2) {
    results.value = []
    return
  }

  const token = localStorage.getItem('token')
  if (!token) return

  loading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/people/search?q=${encodeURIComponent(query.value)}`, {
      headers: { 'Authorization': `Bearer ${token}` }
    })
    const data = await response.json()
    if (data.success) {
      results.value = data.data
    }
  } catch (err) {
    console.error('Search failed:', err)
  } finally {
    loading.value = false
  }
}

let timeout: any
watch(query, (newQuery) => {
  clearTimeout(timeout)
  timeout = setTimeout(handleSearch, 300)
})
</script>

<template>
  <div class="relative w-full">
    <div class="relative items-center">
      <input v-model="query" type="text"
        class="w-full pl-12 pr-4 py-3 rounded-2xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all placeholder-gray-500"
        placeholder="Quick search by name..." />
      <div class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-500">
        <svg v-if="loading" class="animate-spin h-5 w-5" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"></circle>
          <path class="opacity-75" fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
          </path>
        </svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
          stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </div>
    </div>

    <!-- Predict Results -->
    <transition enter-active-class="transition duration-200 ease-out" enter-from-class="transform scale-95 opacity-0"
      enter-to-class="transform scale-100 opacity-100" leave-active-class="transition duration-150 ease-in"
      leave-from-class="transform scale-100 opacity-100" leave-to-class="transform scale-95 opacity-0">
      <div v-if="results.length > 0 && query.length >= 2"
        class="absolute z-50 w-full mt-2 bg-[#0f172a]/90 backdrop-blur-2xl border border-white/10 rounded-2xl overflow-hidden shadow-2xl">
        <div v-for="person in results" :key="person.id"
          class="px-4 py-3 hover:bg-white/5 cursor-pointer flex justify-between items-center transition-colors border-b border-white/5 last:border-0">
          <div>
            <div class="text-white font-medium">{{ person.name }}</div>
            <div class="text-[10px] text-gray-500">{{ person.category }}</div>
          </div>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-gray-600" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </div>
      </div>
    </transition>
  </div>
</template>
