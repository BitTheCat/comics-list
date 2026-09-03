<template>
  <div>
      <div class="flex gap-4">
          <button class="btn btn-sm" @click="toggle">Toggle Theme</button>
          <button class="btn  btn-sm" @click="showModal = true">Add Series</button>
      </div>
      <Table :items="series" :fields="fields" :key="renderKey">
          <template #title="{ item }">
              <p>{{ item.title }}</p>
              <span class="text-muted">{{item.description ?? 'aaaaaaaaaaaa'}}</span>
          </template>
          <template #actions="{ item }">
              <button class="btn btn-sm" @click="emit('details', item.id)">Details</button>
          </template>
      </Table>
  </div>
  <CreateSeriesModal v-model="showModal" @created="onCreated"/>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import Table from '../components/Table.vue';
import CreateSeriesModal from '../components/CreateSeriesModal.vue';
import { api } from '../api';

const emit = defineEmits<{
  (e: 'created'): void;
  (e: 'details', id: string | number): void;
}>();

const toggle = () => {
  const html = document.documentElement;
  html.setAttribute('data-theme', html.getAttribute('data-theme') === 'dark' ? 'light' : 'dark');
};

const showModal = ref(false);

const fields = ref([
  {
    key: 'title',
    label: 'Title',
  },
  {
    key: 'actions',
    label: '',
  },
]);

const renderKey = ref(1);

const series = ref<any[]>([]);

const loadSeries = async () => {
  try {
    const res = await api.listSeries();
    series.value = res as any[];
    renderKey.value++;
  } catch (e) {
    console.error(e);
  }
};

onMounted(() => {
  loadSeries();
});

const refreshSeries = loadSeries;

const onCreated = async () => {
  // refresh local list
  await refreshSeries();
  // notify parent so it can refresh its own data if needed
  emit('created');
};

</script>

