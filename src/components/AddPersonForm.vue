<script setup lang="ts">
import { ref, watch } from 'vue'

import { API_BASE_URL } from '../config/api'

const name = ref('')
const category = ref('Bini')
const description = ref('')
const selectedFile = ref<File | null>(null)
const imagePreview = ref<string | null>(null)
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
  <div class="card-brutalist p-8 border-2 border-analog-cream bg-black/50 backdrop-blur-md">
    <div class="flex items-center justify-between mb-8 pb-4 border-b border-white/10">
      <h3 class="text-3xl font-extrabold text-analog-cream uppercase tracking-tighter">
        Initiate <span class="text-hyper-lime">New Protocol</span>
      </h3>
      <div class="flex gap-1">
        <div class="w-3 h-3 bg-radical-pink rounded-full animate-pulse"></div>
        <div class="w-3 h-3 bg-hyper-lime rounded-full animate-pulse delay-75"></div>
        <div class="w-3 h-3 bg-analog-cream rounded-full animate-pulse delay-150"></div>
      </div>
    </div>

    <form @submit.prevent="handleSubmit" class="space-y-8">

      <!-- Image Upload Section -->
      <div class="flex justify-center">
        <div v-if="imagePreview"
          class="relative group w-48 h-48 card-brutalist p-2 rotate-2 border-2 border-analog-cream bg-black">
          <img :src="imagePreview" class="w-full h-full object-cover border border-white/20" />
          <button @click="imagePreview = null; selectedFile = null"
            class="absolute -top-4 -right-4 bg-radical-pink text-black border-2 border-black w-10 h-10 flex items-center justify-center font-bold shadow-[4px_4px_0px_#000] hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-none transition-all">
            X
          </button>
        </div>
        <div v-else @click="fileInput?.click()"
          class="w-full max-w-sm h-48 border-2 border-dashed border-white/20 flex flex-col items-center justify-center gap-4 cursor-pointer hover:bg-white/5 hover:border-hyper-lime transition-all group rounded-2xl">
          <div
            class="p-4 bg-white/5 text-digital-lavender rounded-full group-hover:bg-hyper-lime/20 group-hover:text-hyper-lime transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
          </div>
          <span
            class="font-typewriter font-bold uppercase text-digital-lavender group-hover:text-analog-cream tracking-widest text-sm">Upload
            Subject Photo</span>
          <input type="file" ref="fileInput" class="hidden" @change="handleFileChange" />
        </div>
      </div>

      <!-- Core Data Fields -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="md:col-span-2 space-y-2">
          <label class="font-typewriter text-xs font-bold text-digital-lavender uppercase ml-1 block">Subject
            Identity</label>
          <div class="relative group">
            <input v-model="name" type="text" required placeholder="FULL NAME"
              class="relative w-full px-4 py-3 bg-white/5 border border-white/20 text-analog-cream font-bold uppercase focus:outline-none focus:border-hyper-lime focus:ring-1 focus:ring-hyper-lime/50 transition-colors rounded-none placeholder:text-white/20" />
          </div>
        </div>

        <div class="space-y-2">
          <label class="font-typewriter text-xs font-bold text-digital-lavender uppercase ml-1 block">Temporal
            Age</label>
          <div class="relative group">
            <input v-model="age" type="number" placeholder="00"
              class="relative w-full px-4 py-3 bg-white/5 border border-white/20 text-analog-cream font-bold focus:outline-none focus:border-hyper-lime focus:ring-1 focus:ring-hyper-lime/50 transition-colors rounded-none placeholder:text-white/20" />
          </div>
        </div>
      </div>

      <div class="space-y-2">
        <label class="font-typewriter text-xs font-bold text-digital-lavender uppercase ml-1 block">Global Lore
          (Public)</label>
        <div class="relative group">
          <textarea v-model="global_description" rows="3"
            class="relative w-full px-4 py-3 bg-white/5 border border-white/20 text-analog-cream font-typewriter text-sm focus:outline-none focus:border-hyper-lime focus:ring-1 focus:ring-hyper-lime/50 transition-colors resize-none rounded-none placeholder:text-white/20"
            placeholder="ENTER GLOBAL DATABASE DESCRIPTION..."></textarea>
        </div>
      </div>

      <!-- Private Data Section -->
      <div class="pt-6 border-t border-white/10 border-dashed space-y-6">
        <div class="flex items-center gap-2">
          <span
            class="bg-radical-pink text-black px-2 py-1 text-xs font-bold border border-black shadow-[2px_2px_0px_#fff]">PRIVATE</span>
          <h4 class="font-typewriter text-sm font-bold text-analog-cream uppercase">Your Collection Metadata</h4>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-2">
            <label
              class="font-typewriter text-xs font-bold text-digital-lavender uppercase ml-1 block">Classification</label>
            <div class="relative group">
              <select v-model="category"
                class="relative w-full px-4 py-3 bg-black border border-white/20 text-analog-cream font-bold uppercase focus:outline-none focus:border-hyper-lime transition-colors appearance-none cursor-pointer rounded-none">
                <option v-for="cat in categories" :key="cat" :value="cat">{{ cat }}</option>
              </select>
              <div class="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none font-bold text-hyper-lime">V
              </div>
            </div>
          </div>

          <div class="space-y-2">
            <label class="font-typewriter text-xs font-bold text-digital-lavender uppercase ml-1 block">Private
              Notes</label>
            <div class="relative group">
              <input v-model="description" type="text"
                class="relative w-full px-4 py-3 bg-black border border-white/20 text-analog-cream font-typewriter text-sm focus:outline-none focus:border-hyper-lime transition-colors rounded-none placeholder:text-white/20"
                placeholder="ONLY VISIBLE TO YOU..." />
            </div>
          </div>
        </div>
      </div>

      <div v-if="error"
        class="bg-red-500/20 text-red-400 p-4 border border-red-500/50 font-bold text-center uppercase font-typewriter text-sm">
        WARNING: {{ error }}
      </div>

      <button type="submit" :disabled="loading"
        class="w-full py-4 bg-hyper-lime text-black font-black text-xl uppercase border-2 border-transparent hover:border-white shadow-[0px_0px_20px_rgba(139,235,0,0.3)] hover:shadow-[0px_0px_30px_rgba(139,235,0,0.5)] active:scale-[0.99] transition-all disabled:opacity-50 disabled:cursor-not-allowed">
        {{ loading ? 'ENCRYPTING & UPLOADING...' : 'COMMIT TO DATABASE' }}
      </button>
    </form>
  </div>
</template>
