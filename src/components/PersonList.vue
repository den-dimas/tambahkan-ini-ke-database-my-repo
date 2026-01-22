<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'

import { API_BASE_URL } from '../config/api'

interface Person {
  id: number;
  name: string;
  category: string;
  description: string;
  created_at: string;
}

const props = defineProps({
  refreshTrigger: Number
})

const people = ref<Person[]>([])
const loading = ref(false)
const filter = ref('')
const categories = ['Bini', 'Janda', 'Kisah']

async function fetchPeople() {
  const token = localStorage.getItem('token')
  if (!token) return

  loading.value = true
  try {
    const url = filter.value ? `${API_BASE_URL}/people?category=${filter.value}` : `${API_BASE_URL}/people`
    const response = await fetch(url, {
      headers: { 'Authorization': `Bearer ${token}` }
    })
    const data = await response.json()
    if (response.ok) {
      people.value = data
    }
  } catch (err) {
    console.error('Failed to fetch people:', err)
  } finally {
    loading.value = false
  }
}

onMounted(fetchPeople)
watch(() => props.refreshTrigger, fetchPeople)
watch(filter, fetchPeople)

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-2 border-b border-white/10">
      <h3 class="text-xl font-bold text-white">Database Entries</h3>
      <div class="flex flex-wrap gap-2">
        <button @click="filter = ''" :class="!filter ? 'bg-emerald-500 text-white' : 'bg-white/5 text-gray-400'"
          class="px-3 py-1 rounded-full text-[10px] md:text-xs font-semibold transition-all">
          All
        </button>
        <button v-for="cat in categories" :key="cat" @click="filter = cat"
          :class="filter === cat ? 'bg-emerald-500 text-white' : 'bg-white/5 text-gray-400'"
          class="px-3 py-1 rounded-full text-[10px] md:text-xs font-semibold transition-all">
          {{ cat }}
        </button>
      </div>
    </div>

    <div v-if="loading" class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-emerald-500"></div>
    </div>

    <div v-else-if="people.length === 0"
      class="text-center py-12 text-gray-500 border-2 border-dashed border-white/5 rounded-2xl px-4">
      No entries found in this category.
    </div>

    <div v-else class="grid gap-4 sm:grid-cols-2 lg:grid-cols-1 xl:grid-cols-2 2xl:grid-cols-3 3xl:grid-cols-4">
      <div v-for="person in people" :key="person.id"
        class="group p-4 rounded-2xl bg-white/5 border border-white/10 hover:border-emerald-500/50 hover:bg-white/10 transition-all duration-300 transform hover:-translate-y-1">
        <div class="flex justify-between items-start mb-2 gap-2">
          <h4
            class="text-base md:text-lg font-semibold text-white group-hover:text-emerald-400 transition-colors line-clamp-1">
            {{ person.name }}
          </h4>
          <span
            class="px-2 py-0.5 rounded-lg bg-emerald-500/20 text-emerald-400 text-[8px] md:text-[10px] font-bold uppercase tracking-wider shrink-0">
            {{ person.category }}
          </span>
        </div>
        <p class="text-gray-400 text-xs md:text-sm line-clamp-2 mb-3">
          {{ person.description || 'No description provided.' }}
        </p>
        <div class="text-[8px] md:text-[10px] text-gray-500 font-medium">
          Added on {{ formatDate(person.created_at) }}
        </div>
      </div>
    </div>

  </div>
</template>
