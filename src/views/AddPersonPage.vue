<script setup lang="ts">
import AddPersonForm from '../components/AddPersonForm.vue'
import { useRouter } from 'vue-router'

const router = useRouter()

function onPersonAdded() {
    router.push('/profile')
}
</script>

<template>
    <div class="max-w-2xl mx-auto px-4 py-8">
        <div class="flex items-center gap-4 mb-8">
            <router-link to="/search"
                class="p-2 rounded-full hover:bg-white/5 text-gray-400 hover:text-white transition-all">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
            </router-link>
            <h1 class="text-3xl font-bold text-white">Add Entry</h1>
        </div>

        <AddPersonForm @person-added="onPersonAdded" />
    </div>
</template>
