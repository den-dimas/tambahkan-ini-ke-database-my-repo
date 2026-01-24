<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { API_BASE_URL } from '../config/api'

interface Person {
    id: string; // tracking_id (optional if not in list)
    person_id: string; // global id
    name: string;
    category?: string;
    description?: string; // personal
    global_description?: string;
    age?: number;
    image_url?: string;
    creator_id?: string;
    created_at: string;
}

const props = defineProps({
    personId: {
        type: String,
        required: true
    }
})

const emit = defineEmits(['back', 'edit-proposed', 'added-to-list'])

const person = ref<Person | null>(null)
const loading = ref(true)
const error = ref('')
const isEditing = ref(false)

// Edit form state
const editForm = ref({
    global_description: '',
    age: 0,
    image_url: ''
})

async function fetchPersonDetails() {
    const token = localStorage.getItem('token')
    if (!token) return

    loading.value = true
    try {
        const response = await fetch(`${API_BASE_URL}/people/${props.personId}`, {
            headers: { 'Authorization': `Bearer ${token}` }
        })
        const data = await response.json()
        if (data.success) {
            person.value = data.data
            editForm.value = {
                global_description: data.data.global_description || '',
                age: data.data.age || 0,
                image_url: data.data.image_url || ''
            }
        } else {
            error.value = data.error || 'Failed to fetch details'
        }
    } catch (err: any) {
        error.value = err.message
    } finally {
        loading.value = false
    }
}

onMounted(fetchPersonDetails)

async function submitEdit() {
    const token = localStorage.getItem('token')
    if (!token) return

    try {
        const response = await fetch(`${API_BASE_URL}/people/${props.personId}/edit`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify({
                description: editForm.value.global_description,
                age: editForm.value.age,
                image_url: editForm.value.image_url
            })
        })
        const data = await response.json()
        if (data.success) {
            alert('Edit proposal submitted! Waiting for community approval.')
            isEditing.value = false
            emit('edit-proposed')
        }
    } catch (err: any) {
        alert('Failed to submit edit: ' + err.message)
    }
}

const isOwnList = computed(() => !!person.value?.id)
</script>

<template>
    <div class="fixed inset-0 z-50 bg-[#020617] flex flex-col md:flex-row overflow-hidden">
        <!-- Mobile Back Button -->
        <button @click="$emit('back')"
            class="absolute top-4 left-4 z-50 p-3 bg-black/50 backdrop-blur-md rounded-full text-white md:hidden">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
            </svg>
        </button>

        <!-- Vertical Scrolling Photos (TikTok/Reels Style) -->
        <div
            class="w-full md:w-1/2 h-full overflow-y-scroll snap-y snap-mandatory bg-black scroll-smooth hide-scrollbar">
            <div v-if="person?.image_url"
                class="h-full w-full snap-start relative flex items-center justify-center bg-black">
                <img :src="person.image_url" class="h-full w-full object-contain md:object-cover" />
                <div class="absolute bottom-0 left-0 right-0 p-8 bg-linear-to-t from-black via-black/20 to-transparent">
                    <h1 class="text-4xl font-bold text-white mb-2">{{ person.name }}</h1>
                    <p class="text-xl text-emerald-400 font-semibold">{{ person.age }} years old</p>
                </div>
            </div>
            <!-- Placeholder for more photos -->
            <div v-for="i in 3" :key="i"
                class="h-full w-full snap-start flex items-center justify-center bg-emerald-900/10 text-white/10">
                <span class="text-8xl font-bold">{{ person?.name[0] }}</span>
            </div>
        </div>

        <!-- Details Pane -->
        <div class="w-full md:w-1/2 h-full bg-[#020617] p-8 md:p-12 overflow-y-auto border-l border-white/10">
            <div class="max-w-xl mx-auto space-y-8">
                <div class="flex items-center justify-between">
                    <button @click="$emit('back')"
                        class="hidden md:flex items-center gap-2 text-gray-400 hover:text-white transition-colors">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7 7-7" />
                        </svg>
                        Back to Database
                    </button>

                    <div class="flex gap-4">
                        <button v-if="!isOwnList" @click="$emit('added-to-list', person)"
                            class="px-6 py-2 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold transition-all">
                            Add to My List
                        </button>
                        <button @click="isEditing = !isEditing"
                            class="px-6 py-2 rounded-xl bg-white/5 border border-white/10 hover:bg-white/10 text-white font-bold transition-all">
                            {{ isEditing ? 'Cancel Edit' : 'Edit Wikipedia' }}
                        </button>
                    </div>
                </div>

                <div v-if="!isEditing" class="space-y-8">
                    <section>
                        <h3 class="text-sm font-bold text-emerald-500 uppercase tracking-widest mb-4">Community
                            Description</h3>
                        <p class="text-xl text-gray-300 leading-relaxed font-light">
                            {{ person?.global_description || 'No community description yet. Be the first to add one!' }}
                        </p>
                    </section>

                    <section v-if="isOwnList">
                        <h3 class="text-sm font-bold text-blue-500 uppercase tracking-widest mb-4">My Private Collection
                        </h3>
                        <div class="p-6 rounded-2xl bg-blue-500/5 border border-blue-500/10">
                            <div class="flex items-center gap-2 mb-2">
                                <span
                                    class="px-2 py-0.5 rounded bg-blue-500/20 text-blue-400 text-xs font-bold uppercase">{{
                                        person?.category }}</span>
                            </div>
                            <p class="text-gray-400">{{ person?.description || 'No private notes.' }}</p>
                        </div>
                    </section>

                    <section>
                        <h3 class="text-sm font-bold text-gray-500 uppercase tracking-widest mb-4">Information</h3>
                        <div class="grid grid-cols-2 gap-4">
                            <div class="p-4 rounded-xl bg-white/5 border border-white/5">
                                <div class="text-gray-500 text-xs mb-1">Status</div>
                                <div class="text-white font-medium">Verified Community Page</div>
                            </div>
                            <div class="p-4 rounded-xl bg-white/5 border border-white/5">
                                <div class="text-gray-500 text-xs mb-1">Registered By</div>
                                <div class="text-white font-medium">User #{{ person?.creator_id?.split('-')[0] }}</div>
                            </div>
                        </div>
                    </section>
                </div>

                <div v-else class="space-y-6">
                    <h2 class="text-2xl font-bold text-white">Propose Wikipedia Edit</h2>
                    <p class="text-sm text-gray-400">Your changes will be submitted to the community for approval.
                        Consensus is required before they go live.</p>

                    <div class="space-y-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-400 mb-1">Global Description</label>
                            <textarea v-model="editForm.global_description" rows="6"
                                class="w-full px-4 py-3 rounded-xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all resize-none"></textarea>
                        </div>

                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-400 mb-1">Age</label>
                                <input v-model="editForm.age" type="number"
                                    class="w-full px-4 py-3 rounded-xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all" />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-400 mb-1">Photo URL</label>
                                <input v-model="editForm.image_url" type="text"
                                    class="w-full px-4 py-3 rounded-xl bg-white/5 border border-white/10 text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all" />
                            </div>
                        </div>

                        <button @click="submitEdit"
                            class="w-full py-4 rounded-2xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-lg shadow-xl shadow-emerald-600/20 transition-all">
                            Submit Proposal
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.hide-scrollbar::-webkit-scrollbar {
    display: none;
}

.hide-scrollbar {
    -ms-overflow-style: none;
    scrollbar-width: none;
}
</style>
