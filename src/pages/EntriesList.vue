<template>
    <button class="btn btn-sm" @click="showModal = true">Add number</button>
    <div v-if="!props.seriesId">
        <p>Select a work/series to see entries. (Placeholder)</p>
    </div>

    <Table v-else :items="entries" :fields="fields">
        <template #actions="{ item }">
            <button class="btn btn-sm" @click="deleteElement(item.id)">Delete</button>
        </template>
    </Table>
    <CreateEntryModal v-if="seriesId" v-model="showModal" :series_id="props.seriesId" @created="onCreated" />
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import { api } from "../api";
import Table from "../components/Table.vue";
import CreateEntryModal from "../components/CreateEntryModal.vue";
import { deleteEntry } from "../mocks/api_entries";

// No script logic yet; kept as <script setup> for consistency

const props = defineProps({
    seriesId: {
        type: String,
        default: null,
    },
});

const emit = defineEmits<{
  (e: 'created'): void;
}>();

const showModal = ref(false);

const fields = ref<any[]>([
    { key: "issue_number", label: "Issue Number" },
    { key: "title", label: "Title" },
    { key: "pub_year", label: "Publication Year" },
    { key: "pub_month", label: "Publication Month" },
    { key: "actions", label: "" },
]);

const entries = ref<any[]>([]);

const fetchEntries = async () => {
    const res = await api.getEntries(props.seriesId);
    entries.value = res;
};

const refreshEntries = fetchEntries;

const onCreated = async () => {
  // refresh local list
  await refreshEntries();
  // notify parent so it can refresh its own data if needed
  emit('created');
};

const deleteElement = async (id: string) => {
    await deleteEntry(id);
    refreshEntries();
};

watch(() => props.seriesId, fetchEntries);
</script>
