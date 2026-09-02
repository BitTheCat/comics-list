<template>
  <div id="app" style="font-family: Arial, Helvetica, sans-serif; padding: 16px;">
    <h1>comics-list (Tauri + Vue)</h1>
    <div style="display:flex; gap:24px;">
      <div style="width:300px;">
        <h2>Works</h2>
        <WorksList @created="refreshWorks" :works="works" />
      </div>
      <div style="flex:1;">
        <h2>Entries</h2>
        <EntriesList />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import WorksList from './components/WorksList.vue';
import EntriesList from './components/EntriesList.vue';
import { api } from './api';

export default defineComponent({
  components: { WorksList, EntriesList },
  setup() {
    const works = ref([]);

    const loadWorks = async () => {
      try {
        const res = await api.listSeries();
        works.value = res as any[];
      } catch (e) {
        console.error(e);
      }
    };

    onMounted(() => {
      loadWorks();
    });

    return { works, refreshWorks: loadWorks };
  },
});
</script>

