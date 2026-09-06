<template>
    <button title="Delete" class="text-red-500 disabled:opacity-50">
        <Trash :size="16" />
    </button>
</template>

<script setup lang="ts">
import { Trash } from '@lucide/vue';
</script>
