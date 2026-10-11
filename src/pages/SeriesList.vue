<template>
  <div>
      <div class="flex gap-4">
          <button class="btn  btn-sm" @click="showModal = true">{{ t('series_add') }}</button>
      </div>

    <SearchField v-model="searchQuery" @search="fetchSeries(undefined, true)" />
      
      <Table 
        :items="series.items" 
        :fields="fields" 
        :pagination="series.pagination"
        selectable
        @row-click="v => viewDetails(v.id)"
        @previous="fetchSeries"
        @next="fetchSeries">
          <template #actions="{ item }">
          <div class="flex gap-2">
                <EditButton @click="openEdit(item)"/>
                <ViewButton @click="viewDetails(item.id)" />
                <DeleteButton :disabled="item.entries_count > 0" @delete="deleteSeries(item.id)" />
          </div>
          </template>
      </Table>
  </div>
  <SeriesModal v-model="showModal" :series="editItem" @created="onCreated"/>
</template>

<script setup lang="ts" >
import { computed, onMounted, ref } from 'vue';
import Table from '../components/Table.vue';
import SeriesModal from '../components/series/SeriesModal.vue';
import { api } from '../api';
import { useRouter } from 'vue-router';
import EditButton from '../components/EditButton.vue';
import ViewButton from '../components/ViewButton.vue';
import DeleteButton from '../components/DeleteButton.vue';
import type { PaginatedResponse, Series, SeriesResource } from '../types';
import { useToast } from '../composables/useToast';
import { useI18n } from '../i18n/useI18n';
import SearchField from '../components/SearchField.vue';


const router = useRouter();

const showModal = ref(false);
const editItem = ref<Series | null>(null);
const { showToast } = useToast();
const { t } = useI18n();
const searchQuery = ref<string | null>(null);

const fields = computed(() => ([
  {
    key: 'title',
    label: t('series_field_title'),
  },
  {
    key: 'description',
    label: t('series_field_description'),
  },
  {
    key: 'publisher.name',
    label: t('series_field_publisher'),
  },
  {
    key: 'start_year',
    label: t('series_field_year'),
  },
  {
    key: 'entries_count',
    label: t('series_field_numbers'),
  },
  {
    key: 'actions',
    label: '',
    tdClass: 'flex justify-end',
  },
]));

const series = ref<PaginatedResponse<SeriesResource>>({
  items: [],
  pagination: {
    page: 1,
    page_size: 10,
    total_items: 0,
    total_pages: 1,
  },
});

const fetchSeries = async (isNext?: boolean, resetPage?: boolean) => {
  const page = isNext
    ? series.value.pagination.page + 1
    : series.value.pagination.page - 1;

  try {
    const res = await api.listSeriesPaginated({
      page: resetPage ? 1 : page,
      query: searchQuery.value,
    });
    series.value = res as PaginatedResponse<SeriesResource>;
  } catch (e) {
    console.error(e);
  }
};

onMounted(() => {
  fetchSeries();
});

const openEdit = (series: any) => {
  editItem.value = series;
  showModal.value = true;
};

const deleteSeries = async (id: number) => {
  try {
    await api.deleteSeries(id as number);
    await refreshSeries();
    showToast(t('series_toast_delete_success'), 'success');
  } catch (e) {
    console.error(e);
    showToast(t('series_toast_delete_error'), 'error');
  }
};

const refreshSeries = fetchSeries;

const onCreated = async () => {
  // refresh local list
  await refreshSeries();
};

const viewDetails = (id: string | number) => {
  router.push({ name: 'series-detail', params: { id } });
};

</script>

