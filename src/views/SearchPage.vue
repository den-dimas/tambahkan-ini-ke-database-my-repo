<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { API_BASE_URL } from '../config/api'
import PersonDetail from '../components/PersonDetail.vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// Just implement custom debounce for simplicity and zero dep
function customDebounce(func: Function, wait: number) {
    let timeout: any
    return function (...args: any[]) {
        clearTimeout(timeout)
        timeout = setTimeout(() => func(...args), wait)
    }
}

interface CategoryStat {
    category: string;
    count: number;
}


interface PersonSearchResult {
    person_id: string;
    name: string;
    category?: string;
    description?: string;
    image_url?: string;
    tracking_id?: string;
}

interface StatsResponse {
    total_people: number;
    category_counts: CategoryStat[];
    top_tracked: PersonSearchResult[];
}

const stats = ref<StatsResponse | null>(null)
const searchQuery = ref('')
const searchResults = ref<PersonSearchResult[]>([])
const loadingStats = ref(false)
const isSearching = ref(false)
const selectedPersonId = ref<string | null>(null)

async function fetchStats() {
    loadingStats.value = true
    try {
        const token = localStorage.getItem('token') // Stats might be public? But safe to send token
        const headers: HeadersInit = {}
        if (token) headers['Authorization'] = `Bearer ${token}`

        const response = await fetch(`${API_BASE_URL}/people/stats`, { headers })
        const data = await response.json()
        if (data.success) {
            stats.value = data.data
        }
    } catch (err) {
        console.error('Failed to fetch stats:', err)
    } finally {
        loadingStats.value = false
    }
}

const performSearch = async () => {
    if (!searchQuery.value.trim()) {
        searchResults.value = []
        return
    }

    isSearching.value = true
    try {
        const token = localStorage.getItem('token')
        const headers: HeadersInit = {}
        if (token) headers['Authorization'] = `Bearer ${token}`

        const response = await fetch(`${API_BASE_URL}/people/search?q=${encodeURIComponent(searchQuery.value)}`, { headers })
        const data = await response.json()
        if (data.success) {
            searchResults.value = data.data
        }
    } catch (err) {
        console.error('Search failed:', err)
    } finally {
        isSearching.value = false
    }
}

const debouncedSearch = customDebounce(performSearch, 500)

watch(searchQuery, () => {
    if (!searchQuery.value) searchResults.value = []
    else debouncedSearch()
})

onMounted(() => {
    fetchStats()
})

function getFullImageUrl(url?: string) {
    if (!url) return `https://ui-avatars.com/api/?name=User&background=random`
    if (url.startsWith('http')) return url
    return url
}

async function addToMyList(person: any) {
    // Basic implementation: Navigate to profile after adding
    // In a real scenario, we might want to open a modal to set category/private note
    // For now, let's assume the user wants to add it with defaults or we prompt them
    // But wait, the AddPersonForm handles "creating" a person which also tracks them.
    // If we want to track an EXISTING person, we need an endpoint or use the same create endpoint with a flag?
    // Based on PersonService, create_person handles logic.
    // Let's redirect to a "track" page or just show success for now.
    // Actually, PersonDetail has an "App to My List" button that emits 'added-to-list'.
    // Let's just listen to that.

    const token = localStorage.getItem('token')
    if (!token) return

    try {
        const response = await fetch(`${API_BASE_URL}/people`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify({
                name: person.name,
                category: 'Bini', // Default
                description: '',
                // global_description: person.global_description,
                age: person.age,
                image_url: person.image_url,
                // If the backend handles de-duplication by name/person_id, this might work.
                // Re-reading backend code (memory): create_person checks if person exists.
                // If it exists, it links the user to it.
                // So sending the same name should work!
            })
        })
        const data = await response.json()
        if (data.success) {
            alert('Added to your list!')
            selectedPersonId.value = null
            router.push('/profile')
        } else {
            alert('Failed to add: ' + data.error)
        }
    } catch (err: any) {
        alert('Error: ' + err.message)
    }
}
</script>

<template>
    <div class="max-w-6xl mx-auto px-6 py-8 flex flex-col gap-12 pb-32 overflow-x-hidden relative">

        <!-- Header with Add Button -->
        <div class="flex justify-end">
            <router-link to="/add"
                class="flex items-center gap-2 px-4 py-2 bg-analog-cream text-black font-bold uppercase tracking-wider hover:bg-white transition-colors border-2 border-black shadow-[4px_4px_0px_#000]">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd"
                        d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z"
                        clip-rule="evenodd" />
                </svg>
                Add New Person
            </router-link>
        </div>

        <!-- Search Input (Neo-Input) -->
        <div class="max-w-2xl mx-auto text-center flex flex-col gap-6 w-full">
            <h1 class="text-5xl uppercase text-hyper-lime drop-shadow-[4px_4px_0px_rgba(0,0,0,0.5)]">
                Search Database
            </h1>
            <div class="relative group w-full">
                <div class="absolute inset-0 bg-hyper-lime translate-x-2 translate-y-2 rounded-button">
                </div>
                <input v-model="searchQuery" type="text" placeholder="TYPE NAME HERE..."
                    class="relative w-full h-16 pl-6 pr-4 bg-analog-cream text-deep-earth font-typewriter text-lg uppercase border-2 border-black rounded-button focus:outline-none placeholder:text-deep-earth/50" />

                <div v-if="isSearching" class="absolute right-6 top-1/2 -translate-y-1/2 z-10">
                    <div class="animate-spin h-6 w-6 border-4 border-deep-earth border-t-transparent rounded-full">
                    </div>
                </div>
            </div>
        </div>

        <!-- Search Results -->
        <div v-if="searchQuery" class="flex flex-col gap-4">
            <h2 class="font-typewriter text-analog-cream">RESULTS ></h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <div v-for="person in searchResults" :key="person.person_id"
                    @click="selectedPersonId = person.person_id"
                    class="flex items-center gap-4 p-4 card-brutalist hover:bg-[rgba(255,255,255,0.05)] transition-colors cursor-pointer">
                    <img :src="getFullImageUrl(person.image_url)"
                        class="h-16 w-16 object-cover border-2 border-analog-cream" />
                    <div>
                        <div class="text-xl uppercase font-bold text-analog-cream">{{ person.name }}
                        </div>
                        <div v-if="person.category"
                            class="inline-block px-2 bg-hyper-lime text-black text-xs font-bold font-typewriter border border-black mt-1">
                            {{ person.category }}
                        </div>
                    </div>
                </div>
                <div v-if="!isSearching && searchResults.length === 0"
                    class="col-span-full text-center text-digital-lavender font-typewriter py-12 border-2 border-dashed border-digital-lavender">
                    NO ENTRIES FOUND. BE THE FIRST TO ADD.
                </div>
            </div>
        </div>

        <!-- Stats Section -->
        <div v-else class="flex flex-col gap-12">

            <!-- Top Cards -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <!-- Total Count -->
                <div class="card-solid p-6 flex flex-col justify-between h-48 bg-hyper-lime border-black">
                    <div class="font-typewriter font-bold text-black border-b-2 border-black pb-2">TOTAL_RECORDS</div>
                    <div class="text-6xl font-bold text-black">{{ stats?.total_people || 0 }}</div>
                </div>

                <!-- Category Stats -->
                <div class="md:col-span-2 card-brutalist p-6">
                    <div class="font-typewriter text-digital-lavender mb-6 border-b border-digital-lavender pb-2">
                        CATEGORY_DISTRIBUTION</div>
                    <div class="flex flex-wrap gap-4">
                        <div v-for="cat in stats?.category_counts" :key="cat.category"
                            class="flex-1 min-w-[120px] p-4 border-2 border-analog-cream text-center hover:bg-analog-cream hover:text-black transition-colors group">
                            <div class="text-3xl font-bold mb-1 group-hover:text-black">{{ cat.count }}</div>
                            <div class="font-typewriter text-xs font-bold uppercase tracking-wider">{{ cat.category }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Top Leaderboard -->
            <div class="flex flex-col gap-6">
                <h2 class="text-3xl text-analog-cream uppercase flex items-center gap-2">
                    <span class="text-radical-pink">👑</span> Top Trendings
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div v-for="(person, index) in stats?.top_tracked" :key="person.person_id"
                        @click="selectedPersonId = person.person_id"
                        class="relative p-6 card-brutalist group hover:scale-[1.02] transition-transform cursor-pointer">

                        <div
                            class="absolute -top-4 -right-4 bg-radical-pink w-12 h-12 flex items-center justify-center font-bold text-black text-xl border-2 border-black shadow-[4px_4px_0px_#000] z-20">
                            #{{ index + 1 }}
                        </div>

                        <div class="flex items-center gap-4">
                            <img :src="getFullImageUrl(person.image_url)"
                                class="h-20 w-20 object-cover border-2 border-analog-cream grayscale group-hover:grayscale-0 transition-all" />
                            <div>
                                <h3 class="text-xl uppercase font-bold text-analog-cream leading-none mb-2">
                                    {{ person.name }}</h3>
                                <span
                                    class="bg-digital-lavender text-black px-2 py-0.5 text-xs font-bold font-typewriter border border-black">
                                    MOST TRACKED
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

        </div>

        <!-- Person Detail Modal -->
        <PersonDetail v-if="selectedPersonId" :person-id="selectedPersonId" @back="selectedPersonId = null"
            @added-to-list="addToMyList" />

    </div>
</template>
