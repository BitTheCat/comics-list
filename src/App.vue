<template>
  <div id="app" style="font-family: Arial, Helvetica, sans-serif; padding: 16px;">
    <h1>comics-list (Tauri + Vue)</h1>
    <div style="display:flex; gap:24px;">
      <div style="width:300px;">
        <h2>Series</h2>
        <SeriesList @details="(v: string) => seriesId = v"/>
      </div>
      <div style="flex:1;">
        <h2>Entries</h2>
        <EntriesList v-model:seriesId="seriesId" />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import SeriesList from './pages/SeriesList.vue';
import EntriesList from './pages/EntriesList.vue';
import { api } from './api';

const seriesId = ref<string|number|null>(null);

</script>

