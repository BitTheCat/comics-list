<template>
  <div>
      <div class="flex gap-4">
          <button class="btn  btn-sm" @click="showModal = true">Add Series</button>
      </div>
      <Table 
        :items="series.items" 
        :fields="fields" 
        :pagination="series.pagination"
        @previous="loadRequestPage"
        @next="loadRequestPage">
          <template #actions="{ item }">
          <div class="flex gap-2">
                <EditButton @click="openEdit(item)"/>
                <ViewButton @click="viewDetails(item.id)" />
                <DeleteButton :disabled="item.entries_count > 0" @click="deleteSeries(item.id)" />
          </div>
          </template>
      </Table>
  </div>
  <SeriesModal v-model="showModal" :series="editItem" @created="onCreated"/>
</template>

<script setup lang="ts" >
import { onMounted, ref } from 'vue';
import Table from '../components/Table.vue';
import SeriesModal from '../components/series/SeriesModal.vue';
import { api } from '../api';
import { useRouter } from 'vue-router';
import EditButton from '../components/EditButton.vue';
import ViewButton from '../components/ViewButton.vue';
import DeleteButton from '../components/DeleteButton.vue';
import type { Series } from '../types';
import type { PaginatedResponse, SeriesResource } from '../mocks/api';


const router = useRouter();

const showModal = ref(false);
const editItem = ref<Series | null>(null);

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
    key: 'start_year',
    label: 'Year',
  },
  {
    key: 'entries_count',
    label: 'Numbers',
  },
  {
    key: 'actions',
    label: '',
    tdClass: 'flex justify-end',
  },
]);

const series = ref<PaginatedResponse<SeriesResource>>({
  items: [],
  pagination: {
    page: 1,
    page_size: 10,
    total_items: 0,
    total_pages: 1,
  },
});

const loadSeries = async () => {
  try {
    const res = await api.listSeriesPaginated();
    series.value = res as PaginatedResponse<SeriesResource>;
  } catch (e) {
    console.error(e);
  }
};

const loadRequestPage = async (isNext: boolean) => {
  try {
    const pageValue = isNext ? series.value.pagination.page + 1 : series.value.pagination.page - 1;
    const res = await api.listSeriesPaginated({ page: pageValue });
    series.value = res as PaginatedResponse<SeriesResource>;
  } catch (e) {
    console.error(e);
  }
};

onMounted(() => {
  loadSeries();
});

const openEdit = (series: any) => {
  editItem.value = series;
  showModal.value = true;
};

const deleteSeries = async (id: string | number) => {
  try {
    await api.deleteSeries({ id });
    await refreshSeries();
  } catch (e) {
    console.error(e);
  }
};

const refreshSeries = loadSeries;

const onCreated = async () => {
  // refresh local list
  await refreshSeries();
};

const viewDetails = (id: string | number) => {
  router.push({ name: 'series-detail', params: { id } });
};

</script>

