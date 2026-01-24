<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { API_BASE_URL } from '../config/api'

interface EditProposal {
    id: string;
    person_id: string;
    proposer_id: string;
    new_description: string;
    new_age: number;
    new_image_url: string;
    status: string;
    created_at: string;
}

const props = defineProps({
    userId: String
})

const edits = ref<EditProposal[]>([])
const loading = ref(true)

async function fetchEdits() {
    const token = localStorage.getItem('token')
    if (!token) return

    loading.value = true
    try {
        const response = await fetch(`${API_BASE_URL}/people/edits`, {
            headers: { 'Authorization': `Bearer ${token}` }
        })
        const data = await response.json()
        if (data.success) {
            edits.value = data.data
        }
    } finally {
        loading.value = false
    }
}

async function vote(editId: string, approve: boolean) {
    const token = localStorage.getItem('token')
    if (!token) return

    try {
        const response = await fetch(`${API_BASE_URL}/people/edits/${editId}/vote`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify({ approve })
        })
        const data = await response.json()
        if (data.success) {
            fetchEdits()
        }
    } catch (err) {
        console.error('Failed to vote:', err)
    }
}

onMounted(fetchEdits)
</script>

<template>
    <div class="space-y-6">
        <div class="pb-2 border-b border-white/10">
            <h3 class="text-xl font-bold text-white">Community Approval Center</h3>
            <p class="text-sm text-gray-400">Review and vote on proposed changes to the shared Wikipedia.</p>
        </div>

        <div v-if="loading" class="flex justify-center py-12">
            <div class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-emerald-500"></div>
        </div>

        <div v-else-if="edits.length === 0"
            class="text-center py-12 text-gray-500 border-2 border-dashed border-white/5 rounded-2xl">
            No pending proposals. You're all caught up!
        </div>

        <div v-else class="space-y-4">
            <div v-for="edit in edits" :key="edit.id"
                class="p-6 rounded-2xl bg-white/5 border border-white/10 hover:border-white/20 transition-all">
                <div class="flex flex-col md:flex-row gap-6">
                    <div class="w-full md:w-32 h-32 bg-emerald-900/10 rounded-xl overflow-hidden shrink-0">
                        <img v-if="edit.new_image_url" :src="edit.new_image_url" class="w-full h-full object-cover" />
                        <div v-else
                            class="w-full h-full flex items-center justify-center font-bold text-4xl text-emerald-500/20">
                            ?</div>
                    </div>

                    <div class="flex-1 space-y-4">
                        <div class="flex justify-between items-start">
                            <div>
                                <h4 class="text-lg font-bold text-white">Proposal for Person #{{
                                    edit.person_id.split('-')[0] }}</h4>
                                <p class="text-xs text-gray-500">By User #{{ edit.proposer_id.split('-')[0] }} • {{ new
                                    Date(edit.created_at).toLocaleDateString() }}</p>
                            </div>
                            <span
                                class="px-2 py-1 rounded bg-yellow-500/10 text-yellow-500 text-[10px] font-bold uppercase tracking-wider">Pending
                                Consensus</span>
                        </div>

                        <div class="grid gap-4 bg-black/20 p-4 rounded-xl text-sm">
                            <div v-if="edit.new_description">
                                <span class="text-gray-500 block mb-1">New Description</span>
                                <p class="text-gray-300">{{ edit.new_description }}</p>
                            </div>
                            <div v-if="edit.new_age">
                                <span class="text-gray-500 block mb-1">New Age</span>
                                <p class="text-white">{{ edit.new_age }}</p>
                            </div>
                        </div>

                        <div class="flex gap-3">
                            <button @click="vote(edit.id, true)"
                                class="flex-1 py-2 px-4 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold transition-all flex items-center justify-center gap-2">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20"
                                    fill="currentColor">
                                    <path fill-rule="evenodd"
                                        d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                        clip-rule="evenodd" />
                                </svg>
                                Approve
                            </button>
                            <button @click="vote(edit.id, false)"
                                class="flex-1 py-2 px-4 rounded-xl bg-red-600/10 border border-red-600/20 hover:bg-red-600/20 text-red-500 font-bold transition-all flex items-center justify-center gap-2">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20"
                                    fill="currentColor">
                                    <path fill-rule="evenodd"
                                        d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                                        clip-rule="evenodd" />
                                </svg>
                                Reject
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
