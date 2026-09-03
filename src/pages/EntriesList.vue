<template>
  <div v-if="!props.seriesId">
    <p>Select a work/series to see entries. (Placeholder)</p>
  </div>

  <Table v-else v-model:items="entries" v-model:fields="fields"/>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue';
import { api } from '../api';
import Table from '../components/Table.vue';

// No script logic yet; kept as <script setup> for consistency

const props = defineProps({
  seriesId: {
    type: String,
    default: null,
  },
});

const fields = ref<any[]>([
  { key: 'issue_number', label: 'Issue Number' },
  { key: 'title', label: 'Title' },
  { key: 'pub_year', label: 'Publication Year' },
  { key: 'pub_month', label: 'Publication Month' },
]);

const entries = ref<any[]>([]);

const fetchEntries = async () => {
  const res = await api.getEntries(props.seriesId);
  entries.value = res;
};

watch(() => props.seriesId, fetchEntries);

</script>

