<template>
    <div class="flex gap-2">
        <RouterLink class="btn btn-sm" to="/series">Back to series</RouterLink>
        <button class="btn btn-sm" @click="showModal = true">Add number</button>
    </div>
    <div v-if="!props.id">
        <p>Select a work/series to see entries. (Placeholder)</p>
    </div>

    <Table v-else :items="entries" :fields="fields">
        <template #pub_month="{ data }">
            {{ formatMonth(data) }}
        </template>
        <template #actions="{ item }">
            <div class="flex gap-2">
                <EditButton />
                <DeleteButton @click="deleteElement(item.id)"/>
            </div>
        </template>
    </Table>
    <CreateEntryModal v-if="props.id" v-model="showModal" :series_id="props.id" @created="onCreated" />
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import { api } from "../api";
import Table from "../components/Table.vue";
import CreateEntryModal from "../components/CreateEntryModal.vue";
import EditButton from "../components/EditButton.vue";
import DeleteButton from "../components/DeleteButton.vue";


// No script logic yet; kept as <script setup> for consistency

const props = defineProps({
    id: {
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

const monthFormatter = new Intl.DateTimeFormat("it-IT", { month: "long" });

const formatMonth = (month: number | null): string => {
    if (month == null || month < 1 || month > 12) return "—";
    const formattedMonth = monthFormatter.format(new Date(2000, month - 1, 1));
    return formattedMonth.charAt(0).toUpperCase() + formattedMonth.slice(1);
};

const fetchEntries = async () => {
    const res = await api.getEntries(props.id);
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
    await api.deleteEntry(id);
    refreshEntries();
};

watch(() => props.id, fetchEntries, { immediate: true });
</script>
