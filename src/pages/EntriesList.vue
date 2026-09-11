<template>
    <div class="flex gap-2">
        <RouterLink class="btn btn-sm" to="/series">{{ t('entries_back_to_series') }}</RouterLink>
        <button class="btn btn-sm" :disabled="!seriesId" @click="openCreate">{{ t('entries_add_number') }}</button>
    </div>

    <div v-if="!props.id">
        <p>{{ t('entries_select_series_placeholder') }}</p>
    </div>

    <div v-else class="grid items-start gap-4 md:grid-cols-[minmax(0,3fr)_minmax(16rem,2fr)]">
        <section class="min-w-0">
            <Table
                :items="entries.items"
                :fields="fields"
                :pagination="entries.pagination"
                selectable
                :selected-id="selectedEntry?.id"
                @row-click="selectEntry"
                @previous="loadRequestPage"
                @next="loadRequestPage"
            >
                <template #year="{ item }">
                    {{ formatPublication(item as Entry) }}
                </template>

                <template #actions="{ item }">
                    <div class="flex justify-end gap-2 md:hidden" @click.stop>
                        <ViewButton
                            v-if="item.cover_path"
                            @click="openCoverModal(item as Entry)"
                        />
                        <EditButton @click="openEdit(item as Entry)" />
                        <DeleteButton @delete="deleteElement(item.id)" />
                    </div>
                </template>
            </Table>
        </section>

        <EntryDetailsPanel
            :entry="selectedEntry"
            :publication="selectedEntry ? formatPublication(selectedEntry) : '—'"
            @view="openCoverModal"
            @edit="openEdit"
            @delete="deleteEntry"
        />
    </div>

    <EntryModal
        v-if="seriesId"
        v-model="showModal"
        :entry="editItem"
        :series_id="seriesId"
        @created="onCreated"
    />
    <Modal
        v-model="showCoverModal"
        :title="t('entries_cover_title', { issue: coverItem?.issue_number ?? '' })"
        :show-submit="false"
        :cancel-label="t('common_close')"
    >
        <div class="flex justify-center">
            <img
                v-if="coverItem?.cover_path"
                :src="coverItem.cover_path"
                alt="Cover"
                class="max-h-[70vh] max-w-full object-contain"
            />
        </div>
    </Modal>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from "vue";
import { api } from "../api";
import type { PaginatedResponse } from "../mocks/api";
import type { Entry } from "../types";
import DeleteButton from "../components/DeleteButton.vue";
import EditButton from "../components/EditButton.vue";
import EntryModal from "../components/entries/EntryModal.vue";
import EntryDetailsPanel from "../components/entries/EntryDetailsPanel.vue";
import Modal from "../components/Modal.vue";
import Table from "../components/Table.vue";
import ViewButton from "../components/ViewButton.vue";
import { useToast } from "../composables/useToast";
import { useI18n } from '../i18n/useI18n';

const props = withDefaults(defineProps<{
    id?: string | null;
}>(), {
    id: null,
});

const emit = defineEmits<{
    (e: "created"): void;
}>();

const seriesId = computed(() => Number(props.id));
const showModal = ref(false);
const editItem = ref<Entry | null>(null);
const selectedEntry = ref<Entry | null>(null);
const showCoverModal = ref(false);
const coverItem = ref<Entry | null>(null);
const { showToast } = useToast();
const { t, locale } = useI18n();

const fields = computed(() => [
    { key: "issue_number", label: "#", thClass: "w-1/12" },
    { key: "title", label: t('entries_field_title') },
    { key: "year", label: t('entries_field_publication') },
    { key: "condition", label: t('entries_field_condition'), tdClass: "w-1/12" },
    {
        key: "actions",
        label: "",
        thClass: "md:hidden",
        tdClass: "md:hidden",
    },
]);

const entries = ref<PaginatedResponse<Entry>>({
    items: [],
    pagination: {
        page: 1,
        page_size: 10,
        total_items: 0,
        total_pages: 1,
    },
});

const monthFormatter = computed(() =>
    new Intl.DateTimeFormat(locale.value === 'it' ? 'it-IT' : 'en-US', { month: "long" }),
);

const formatMonth = (month: number | null): string | null => {
    if (month == null || month < 1 || month > 12) return null;
    const formattedMonth = monthFormatter.value.format(new Date(2000, month - 1, 1));
    return formattedMonth.charAt(0).toUpperCase() + formattedMonth.slice(1);
};

const formatPublication = (entry: Entry): string => {
    const month = formatMonth(entry.pub_month);
    return [month, entry.pub_year].filter(Boolean).join(" ") || "—";
};

const updateEntries = (response: PaginatedResponse<Entry>) => {
    entries.value = response;
    if (!selectedEntry.value?.id) return;

    selectedEntry.value = response.items.find(
        (entry) => entry.id === selectedEntry.value?.id,
    ) ?? null;
};

const fetchEntries = async () => {
    if (!props.id) return;
    const response = await api.listEntriesPaginated(seriesId.value);
    updateEntries(response as PaginatedResponse<Entry>);
};

const loadRequestPage = async (isNext: boolean) => {
    try {
        const page = isNext
            ? entries.value.pagination.page + 1
            : entries.value.pagination.page - 1;
        const response = await api.listEntriesPaginated(seriesId.value, { page });
        updateEntries(response as PaginatedResponse<Entry>);
    } catch (error) {
        console.error(error);
    }
};

const selectEntry = (item: Record<string, any>) => {
    selectedEntry.value = item as Entry;
};

const openCreate = () => {
    editItem.value = null;
    showModal.value = true;
};

const openEdit = (item: Entry) => {
    editItem.value = item;
    showModal.value = true;
};

const onCreated = async () => {
    await fetchEntries();
    emit("created");
};

const deleteElement = async (id: string) => {
    try {
        await api.deleteEntry(id);
        if (selectedEntry.value?.id === id) selectedEntry.value = null;
        await fetchEntries();
        showToast(t('entry_toast_delete_success'), "success");
    } catch (error) {
        console.error(error);
        showToast(t('entry_toast_delete_error'), "error");
    }
};

const deleteEntry = async (entry: Entry) => {
    if (entry.id) await deleteElement(entry.id);
};

const openCoverModal = (item: Entry) => {
    coverItem.value = item;
    showCoverModal.value = true;
};

watch(() => props.id, () => {
    selectedEntry.value = null;
    coverItem.value = null;
    void fetchEntries();
}, { immediate: true });
</script>
