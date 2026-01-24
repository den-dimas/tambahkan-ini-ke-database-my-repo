<script setup lang="ts">
import { ref, watch } from 'vue'

import { API_BASE_URL } from '../config/api'

const name = ref('')
const category = ref('Bini')
const description = ref('')
const selectedFile = ref<File | null>(null)
const imagePreview = ref<string | null>(null)
const uploadProgress = ref(0)
const loading = ref(false)
const error = ref('')
const suggestions = ref<any[]>([])
const showSuggestions = ref(false)
const loadingSuggestions = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    const file = target.files[0]
    if (!file.type.startsWith('image/')) {
      error.value = 'Please select an image file.'
      return
    }
    if (file.size > 5 * 1024 * 1024) {
      error.value = 'File size must be less than 5MB.'
      return
    }
    selectedFile.value = file
    imagePreview.value = URL.createObjectURL(file)
    error.value = ''
  }
}

async function fetchSuggestions() {
  // ... existing fetchSuggestions code ...
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

const global_description = ref('')
const age = ref(0)

// ... existing handleFileChange ...

async function handleSubmit() {
  const token = localStorage.getItem('token')
  if (!token) return

  error.value = ''
  loading.value = true

  let imageUrl = null

  try {
    if (selectedFile.value) {
      const urlResponse = await fetch(`${API_BASE_URL}/people/upload-url`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({
          filename: selectedFile.value.name,
          content_type: selectedFile.value.type
        })
      })

      const urlData = await urlResponse.json()
      if (!urlData.success) throw new Error(urlData.error)

      const { upload_url, public_url } = urlData.data
      await fetch(upload_url, {
        method: 'PUT',
        headers: { 'Content-Type': selectedFile.value.type },
        body: selectedFile.value
      })
      imageUrl = public_url
    }

    const response = await fetch(`${API_BASE_URL}/people`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({
        name: name.value,
        category: category.value,
        description: description.value, // personal
        global_description: global_description.value,
        age: age.value,
        image_url: imageUrl
      }),
    })

    const data = await response.json()
    if (!data.success) throw new Error(data.error)

    name.value = ''
    description.value = ''
    global_description.value = ''
    age.value = 0
    selectedFile.value = null
    imagePreview.value = null
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
    <h3 class="text-xl font-bold text-white mb-4">Register New Person</h3>

    <form @submit.prevent="handleSubmit" class="space-y-4">
      <!-- ... Image Section ... -->
      <div v-if="imagePreview" class="relative group mx-auto w-24 h-24">
        <img :src="imagePreview" class="w-full h-full rounded-2xl object-cover border-2 border-emerald-500/50" />
        <button @click="imagePreview = null; selectedFile = null"
          class="absolute -top-1 -right-1 bg-red-500 rounded-full p-1 opacity-0 group-hover:opacity-100 transition-all">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 text-white" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      <div v-else @click="fileInput?.click()"
        class="border-2 border-dashed border-white/10 rounded-2xl p-6 flex flex-col items-center gap-2 cursor-pointer hover:bg-white/5 transition-all">
        <div class="p-2 bg-emerald-500/10 rounded-lg text-emerald-500">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
        </div>
        <span class="text-xs text-gray-400">Add Photo</span>
        <input type="file" ref="fileInput" class="hidden" @change="handleFileChange" />
      </div>

      <div class="grid grid-cols-3 gap-3">
        <div class="col-span-2">
          <label class="block text-[10px] font-bold text-gray-500 uppercase mb-1">Full Name</label>
          <input v-model="name" type="text" required
            class="w-full px-4 py-2 rounded-xl bg-white/5 border border-white/10 text-white text-sm focus:ring-2 focus:ring-emerald-500/50 outline-none" />
        </div>
        <div>
          <label class="block text-[10px] font-bold text-gray-500 uppercase mb-1">Age</label>
          <input v-model="age" type="number"
            class="w-full px-4 py-2 rounded-xl bg-white/5 border border-white/10 text-white text-sm focus:ring-2 focus:ring-emerald-500/50 outline-none" />
        </div>
      </div>

      <div>
        <label class="block text-[10px] font-bold text-gray-500 uppercase mb-1">Wikipedia Description (Global)</label>
        <textarea v-model="global_description" rows="3"
          class="w-full px-4 py-2 rounded-xl bg-white/5 border border-white/10 text-white text-sm focus:ring-2 focus:ring-emerald-500/50 outline-none resize-none"
          placeholder="Visible to the whole community..."></textarea>
      </div>

      <div class="pt-4 border-t border-white/5 space-y-4">
        <h4 class="text-[10px] font-bold text-gray-500 uppercase tracking-widest">Personal Collection Data</h4>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-[10px] font-bold text-gray-400 mb-1">Category</label>
            <select v-model="category"
              class="w-full px-4 py-2 rounded-xl bg-[#0f172a] border border-white/10 text-white text-sm outline-none">
              <option v-for="cat in categories" :key="cat" :value="cat">{{ cat }}</option>
            </select>
          </div>
          <div>
            <label class="block text-[10px] font-bold text-gray-400 mb-1">Private Note</label>
            <input v-model="description" type="text"
              class="w-full px-4 py-2 rounded-xl bg-white/5 border border-white/10 text-white text-sm outline-none"
              placeholder="Only for you..." />
          </div>
        </div>
      </div>

      <p v-if="error" class="text-red-400 text-xs text-center">{{ error }}</p>

      <button type="submit" :disabled="loading"
        class="w-full py-3 rounded-2xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold transition-all disabled:opacity-50 shadow-lg shadow-emerald-500/10">
        {{ loading ? 'Saving...' : 'Register Person' }}
      </button>
    </form>
  </div>
</template>
