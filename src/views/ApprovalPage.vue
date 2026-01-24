<script setup lang="ts">
import ApprovalCenter from '../components/ApprovalCenter.vue'
</script>

<template>
    <div class="max-w-4xl mx-auto px-4 py-8">
        <div class="flex items-center gap-4 mb-8">
            <router-link to="/profile"
                class="p-2 rounded-full hover:bg-white/5 text-gray-400 hover:text-white transition-all">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
            </router-link>
            <h1 class="text-3xl font-bold text-white">Approvals</h1>
        </div>

        <ApprovalCenter />
    </div>
</template>
