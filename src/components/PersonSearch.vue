<script setup lang="ts">
import { ref, watch } from 'vue'

import { API_BASE_URL } from '../config/api'

interface PersonSearchResult {
  person_id: string;
  name: string;
  tracking_id: string | null;
  category: string | null;
  description: string | null;
  image_url: string | null;
  age?: number; // Added based on template usage
}

const emit = defineEmits(['person-selected'])

const query = ref('')
const results = ref<PersonSearchResult[]>([])
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

function getFullImageUrl(url: string | null) {
  if (!url) return ''
  if (url.startsWith('http')) return url
  return url
}

function selectSuggestion(suggestion: PersonSearchResult) {
  emit('person-selected', suggestion.person_id)
  query.value = ''
  results.value = []
}
</script>

<template>
  <div class="relative w-full group">
    <div class="relative">
      <div class="absolute inset-y-0 left-0 pl-3.5 flex items-center pointer-events-none">
        <svg xmlns="http://www.w3.org/2000/svg"
          class="h-4 w-4 text-gray-500 group-focus-within:text-emerald-500 transition-colors" fill="none"
          viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </div>
      <input v-model="query" type="text" placeholder="Find in Wikipedia..."
        class="w-full pl-10 pr-4 py-2.5 rounded-xl bg-white/5 border border-white/10 text-white text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all placeholder:text-gray-600" />
    </div>

    <!-- Results Dropdown -->
    <transition enter-active-class="transition duration-100 ease-out" enter-from-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100" leave-active-class="transition duration-75 ease-in"
      leave-from-class="opacity-100 scale-100" leave-to-class="opacity-0 scale-95">
      <div v-if="results.length > 0"
        class="absolute z-50 w-full mt-2 bg-[#0f172a] border border-white/10 rounded-2xl overflow-hidden shadow-2xl backdrop-blur-xl">
        <div v-for="person in results" :key="person.person_id" @click="selectSuggestion(person)"
          class="px-4 py-3 hover:bg-white/5 cursor-pointer flex items-center gap-3 border-b border-white/5 last:border-0 transition-colors">
          <img v-if="person.image_url" :src="person.image_url"
            class="w-10 h-10 rounded-full object-cover border border-white/10" />
          <div v-else
            class="w-10 h-10 rounded-full bg-emerald-500/10 flex items-center justify-center border border-emerald-500/10">
            <span class="text-xs font-bold text-emerald-500/50">{{ person.name[0] }}</span>
          </div>
          <div>
            <div class="text-white text-sm font-semibold">{{ person.name }}</div>
            <div class="text-[10px] text-gray-500 flex items-center gap-1.5">
              <span v-if="person.tracking_id" class="text-emerald-500">In my list</span>
              <span v-else>Wikipedia Entry</span>
              <span v-if="person.age">• {{ person.age }}y</span>
            </div>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>
