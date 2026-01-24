<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { API_BASE_URL } from '../config/api'

interface Person {
    id: string; // tracking_id
    person_id: string;
    name: string;
    category: string;
    description: string;
    global_description?: string;
    age?: number;
    image_url: string;
    created_at: string;
}

const activeTab = ref('Kisah')
const tabs = ['Kisah', 'Bini', 'Janda']
const people = ref<Person[]>([])
const loading = ref(false)
const username = ref(localStorage.getItem('username') || 'User')

const filteredPeople = computed(() => {
    return people.value.filter(p => p.category.toLowerCase() === activeTab.value.toLowerCase())
})

async function fetchMyPeople() {
    loading.value = true
    try {
        const token = localStorage.getItem('token')
        if (!token) return

        const response = await fetch(`${API_BASE_URL}/people`, {
            headers: { 'Authorization': `Bearer ${token}` }
        })
        const data = await response.json()
        if (data.success) {
            people.value = data.data
        }
    } catch (err) {
        console.error('Failed to fetch people:', err)
    } finally {
        loading.value = false
    }
}

function getFullImageUrl(url?: string) {
    if (!url) return `https://ui-avatars.com/api/?name=P&background=random`
    if (url.startsWith('http')) return url
    return url
}

onMounted(() => {
    fetchMyPeople()
})
</script>

<template>
    <div class="max-w-4xl mx-auto px-4 py-8 pb-32 flex flex-col gap-8">
        <!-- User Header (Solid Card) -->
        <div class="card-solid p-6 flex items-center gap-6">
            <div
                class="h-24 w-24 rounded-full border-4 border-void-charcoal overflow-hidden bg-radical-pink flex items-center justify-center text-4xl font-bold text-white shadow-[4px_4px_0px_#000]">
                {{ (username && username[0]) ? username[0].toUpperCase() : 'U' }}
            </div>
            <div>
                <h1 class="text-4xl text-deep-earth uppercase leading-none">{{ username }}</h1>
                <p
                    class="font-typewriter text-sm text-deep-earth mt-2 bg-digital-lavender inline-block px-2 border border-black">
                    CONTENT CURATOR</p>
                <div class="flex gap-4 mt-4">
                    <div class="font-typewriter text-xs font-bold text-deep-earth">
                        <span class="text-radical-pink text-lg">{{ people.length }}</span> ENTRIES
                    </div>
                </div>
                <div class="mt-6 flex gap-3">
                    <router-link to="/approvals"
                        class="px-4 py-2 bg-black text-white text-xs font-bold uppercase tracking-widest hover:bg-gray-900 transition-colors">
                        Community Approvals
                    </router-link>
                </div>
            </div>
        </div>

        <!-- Tabs with Brutalist Style -->
        <div class="flex gap-4 overflow-x-auto pb-4">
            <button v-for="tab in tabs" :key="tab" @click="activeTab = tab"
                :class="activeTab === tab ? 'bg-hyper-lime text-black shadow-[4px_4px_0px_#000] translate-x-[-2px] translate-y-[-2px]' : 'bg-transparent text-analog-cream'"
                class="border-2 border-analog-cream px-6 py-3 font-bold uppercase transition-all hover:bg-analog-cream hover:text-black font-typewriter whitespace-nowrap">
                {{ tab }}
            </button>
        </div>

        <!-- Content Grid -->
        <div v-if="loading" class="flex justify-center py-12">
            <div class="animate-spin h-8 w-8 border-4 border-hyper-lime border-t-transparent rounded-full">
            </div>
        </div>

        <div v-else-if="filteredPeople.length === 0"
            class="text-center py-20 border-2 border-dashed border-analog-cream rounded-card">
            <div class="text-6xl mb-4 grayscale opacity-50">📭</div>
            <p class="font-typewriter text-analog-cream text-lg uppercase">Empty File.</p>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="person in filteredPeople" :key="person.id"
                class="card-brutalist p-4 transition-all hover:-translate-y-1 hover:shadow-[10px_10px_0px_#FF0099]">
                <div class="flex gap-4 items-start">
                    <img :src="getFullImageUrl(person.image_url)"
                        class="h-20 w-20 object-cover border-2 border-analog-cream" />
                    <div class="min-w-0 flex-1">
                        <h3 class="text-xl text-analog-cream truncate uppercase">{{ person.name }}</h3>
                        <span
                            class="inline-block px-2 py-0.5 bg-hyper-lime text-black text-xs font-bold font-typewriter border border-black mb-2">
                            {{ person.category }}
                        </span>
                        <p class="font-typewriter text-xs text-digital-lavender line-clamp-2 leading-relaxed">
                            {{ person.description || person.global_description || 'NO ADDITIONAL DATA.' }}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
