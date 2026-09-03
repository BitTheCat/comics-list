<template>
  <div id="app" style="font-family: Arial, Helvetica, sans-serif; padding: 16px;">
    <h1>comics-list (Tauri + Vue)</h1>
    <div style="display:flex; gap:24px;">
      <div style="width:300px;">
        <h2>Series</h2>
        <SeriesList @details="onDetails" @created="onCreated" />
      </div>
      <div style="flex:1;">
        <h2>Entries</h2>
        <EntriesList v-model:seriesId="seriesId" />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import SeriesList from './pages/SeriesList.vue';
import EntriesList from './pages/EntriesList.vue';

const seriesId = ref<string | number | null>(null);

function onDetails(id: string | number) {
  seriesId.value = id;
}

function onCreated() {
  // placeholder: SeriesList refreshes itself and emits 'created' to notify parent.
  // We don't need to do anything here for now.
}
</script>

