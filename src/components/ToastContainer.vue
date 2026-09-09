<template>
  <div class="toast toast-top toast-end z-50">
    <div
      v-for="toast in toasts"
      :key="toast.id"
      class="alert shadow-lg"
      :class="toast.type === 'success' ? 'alert-success' : 'alert-error'"
    >
      <span>{{ toast.message }}</span>
      <button class="btn btn-ghost btn-xs" type="button" @click="removeToast(toast.id)">✕</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useToast } from '../composables/useToast';

const { toasts, removeToast } = useToast();
</script>
