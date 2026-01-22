<script setup lang="ts">
import { ref } from 'vue'

import { API_BASE_URL } from '../config/api'

const name = ref('')
const category = ref('Bini')
const description = ref('')
const loading = ref(false)
const error = ref('')

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

    if (!response.ok) {
      throw new Error(data.error || 'Failed to add person')
    }

    name.value = ''
    description.value = ''
    emit('person-added', data)
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
      <div>
        <label class="block text-sm font-medium text-gray-300 mb-1">Name</label>
        <input v-model="name" type="text" required
          class="w-full px-4 py-2.5 rounded-xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all text-sm"
          placeholder="Person name" />
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
