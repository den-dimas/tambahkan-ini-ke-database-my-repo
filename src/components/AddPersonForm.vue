<script setup lang="ts">
import { ref, watch } from 'vue'

import { API_BASE_URL } from '../config/api'

const name = ref('')
const category = ref('Bini')
const description = ref('')
const loading = ref(false)
const error = ref('')
const suggestions = ref<any[]>([])
const showSuggestions = ref(false)
const loadingSuggestions = ref(false)

async function fetchSuggestions() {
  if (name.value.length < 2) {
    suggestions.value = []
    return
  }

  const token = localStorage.getItem('token')
  if (!token) return

  loadingSuggestions.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/people/search?q=${encodeURIComponent(name.value)}`, {
      headers: { 'Authorization': `Bearer ${token}` }
    })
    const data = await response.json()
    if (data.success) {
      suggestions.value = data.data
    }
  } catch (err) {
    console.error('Failed to fetch suggestions:', err)
  } finally {
    loadingSuggestions.value = false
  }
}

let timeout: any
watch(name, () => {
  clearTimeout(timeout)
  timeout = setTimeout(fetchSuggestions, 300)
  showSuggestions.value = true
})

function selectSuggestion(suggestion: any) {
  name.value = suggestion.name
  showSuggestions.value = false
  if (suggestion.tracking_id) {
    category.value = suggestion.category
    description.value = suggestion.description || ''
  }
}

function handleBlur() {
  setTimeout(() => {
    showSuggestions.value = false
  }, 200)
}

const emit = defineEmits(['person-added'])

const categories = ['Bini', 'Janda', 'Kisah']

async function handleSubmit() {
  const token = localStorage.getItem('token')
  if (!token) return

  error.value = ''
  loading.value = true

  try {
    const response = await fetch(`${API_BASE_URL}/people`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({
        name: name.value,
        category: category.value,
        description: description.value,
      }),
    })

    const data = await response.json()

    if (!data.success) {
      throw new Error(data.error || 'Failed to add person')
    }

    name.value = ''
    description.value = ''
    emit('person-added', data.data)
  } catch (err: any) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="bg-white/5 backdrop-blur-lg border border-white/10 p-6 rounded-2xl shadow-xl">
    <h3 class="text-xl font-bold text-white mb-4">Add New Entry</h3>

    <form @submit.prevent="handleSubmit" class="space-y-4">
      <div class="relative">
        <label class="block text-sm font-medium text-gray-300 mb-1">Name</label>
        <input v-model="name" type="text" required
          @blur="handleBlur"
          @focus="showSuggestions = true"
          class="w-full px-4 py-2.5 rounded-xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all text-sm"
          placeholder="Person name" />

        <!-- Autocomplete Dropdown -->
        <transition enter-active-class="transition duration-100 ease-out" enter-from-class="opacity-0 scale-95"
          enter-to-class="opacity-100 scale-100" leave-active-class="transition duration-75 ease-in"
          leave-from-class="opacity-100 scale-100" leave-to-class="opacity-0 scale-95">
          <div v-if="showSuggestions && suggestions.length > 0"
            class="absolute z-50 w-full mt-1 bg-[#0f172a] border border-white/10 rounded-xl overflow-hidden shadow-2xl max-h-48 overflow-y-auto">
            <div v-for="suggestion in suggestions" :key="suggestion.person_id"
              @click="selectSuggestion(suggestion)"
              class="px-4 py-2 hover:bg-white/5 cursor-pointer flex justify-between items-center border-b border-white/5 last:border-0">
              <div>
                <div class="text-white text-sm font-medium">{{ suggestion.name }}</div>
                <div class="text-[10px] text-gray-500">
                  {{ suggestion.tracking_id ? `In your list (${suggestion.category})` : 'Existing entry' }}
                </div>
              </div>
              <div v-if="suggestion.tracking_id" class="text-emerald-500">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
              </div>
            </div>
          </div>
        </transition>
      </div>


      <div>
        <label class="block text-sm font-medium text-gray-300 mb-1">Category</label>
        <select v-model="category"
          class="w-full px-4 py-2 rounded-xl bg-[#1e293b] border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all appearance-none">
          <option v-for="cat in categories" :key="cat" :value="cat">{{ cat }}</option>
        </select>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-300 mb-1">Description</label>
        <textarea v-model="description" rows="3"
          class="w-full px-4 py-2 rounded-xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all resize-none"
          placeholder="Brief description..."></textarea>
      </div>

      <p v-if="error" class="text-red-400 text-sm animate-pulse">{{ error }}</p>

      <button type="submit" :disabled="loading"
        class="w-full py-2 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold transition-all disabled:opacity-50">
        {{ loading ? 'Adding...' : 'Add Person' }}
      </button>
    </form>
  </div>
</template>
