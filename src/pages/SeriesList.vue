<template>
  <div>
      <div class="flex gap-4">
          <button class="btn btn-sm" @click="toggle">Toggle Theme</button>
          <button class="btn  btn-sm" @click="showModal = true">Add Series</button>
      </div>
      <Table :items="series" :fields="fields">
          <template #actions="{ item }">
              <button class="btn btn-sm" @click="viewDetails(item.id)">Details</button>
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
import { useRouter } from 'vue-router';

const router = useRouter();

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
    key: 'description',
    label: 'Description',
  },
  {
    key: 'publisher.name',
    label: 'Publisher',
  },
  {
    key: '',
    label: '',
  },
  {
    key: 'actions',
    label: '',
  },
]);

const series = ref<any[]>([]);

const loadSeries = async () => {
  try {
    const res = await api.listSeries();
    // assign a fresh array with new object references to ensure Vue reactivity
    // picks up deep changes and child components receive a new prop reference.
    series.value = Array.isArray(res) ? (res as any[]).map((s) => ({ ...s })) : [];
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
};

const viewDetails = (id: string | number) => {
  router.push({ name: 'series-detail', params: { id } });
};

</script>

