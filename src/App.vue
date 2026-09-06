<template>
  <div id="app" class="p-4">
    <header class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-bold">comics-list</h1>
      <div class="breadcrumbs text-sm">
        <!-- <RouterLink to="/series">Series</RouterLink> -->
        <button @click="toggle">
            <Sun v-if="theme === 'dark'" :size="16" />
            <Moon v-else :size="16" />
        </button>
      </div>
    </header>

    <RouterView />
  </div>
</template>

<script lang="ts" setup>
  import { Sun, Moon } from 'lucide-vue-next';
  import { ref } from 'vue';

  const theme = ref('light');
  
  const toggle = () => {
    const html = document.documentElement;
    theme.value = html.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
    html.setAttribute('data-theme', theme.value);
  };

</script>

