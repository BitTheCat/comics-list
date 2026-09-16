<template>
    <div class="overflow-x-auto border rounded-box border-base-content/5 my-2">
        <table class="table table-zebra">
            <thead>
                <tr>
                    <th
                        v-for="value in fields"
                        :key="value.key"
                        :class="value.thClass"
                    >
                        {{ value.label }}
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr v-if="!hasItems">
                    <td :colspan="fields.length" class="text-center text-base-content/60">
                        {{ t('table_empty') }}
                    </td>
                </tr>
                <tr
                    v-for="item in items"
                    :key="item.id"
                    :aria-selected="selectable ? selectedId === item.id : undefined"
                    :class="{
                        'cursor-pointer hover:bg-base-200': selectable,
                        'bg-base-200': selectable && selectedId === item.id,
                    }"
                    :tabindex="selectable ? 0 : undefined"
                    @click="selectRow(item)"
                    @keydown.enter="selectRow(item)"
                >
                    <td v-for="value in fields" :key="value.key" :class="value.tdClass">
                        <slot
                            :name="value.key"
                            :data="getValue(item, value.key)"
                            :item="item"
                        >
                            {{ getValue(item, value.key) }}
                        </slot>
                    </td>
                </tr>
            </tbody>
        </table>
    </div>

    <div v-if="pagination" class="flex justify-between">
        <span class="text-sm">{{ summaryText() }}</span>
        <div class="join">
            <button
                v-if="pagination.page > 1"
                class="join-item btn"
                @click="emit('previous', false)"
            >
                «
            </button>
            <button class="join-item btn">{{ pagination.page }}</button>
            <button
                v-if="pagination.page < pagination.total_pages"
                class="join-item btn"
                @click="emit('next', true)"
            >
                »
            </button>
        </div>
    </div>
</template>
<script lang="ts" setup>
import { computed } from "vue";
import type { PaginationMeta } from '../types';
import { useI18n } from '../i18n/useI18n';

interface Field {
    key: string;
    label: string;
    thClass?: string;
    tdClass?: string;
}

const props = withDefaults(defineProps<{
    items: Record<string, any>[];
    fields: Field[];
    pagination?: PaginationMeta;
    selectable?: boolean;
    selectedId?: string | number | null;
}>(), {
    selectable: false,
    selectedId: null,
});

const emit = defineEmits<{
    (e: "previous", isNext: boolean): void;
    (e: "next", isNext: boolean): void;
    (e: "row-click", item: Record<string, any>): void;
}>();

const selectRow = (item: Record<string, any>) => {
    if (props.selectable) emit("row-click", item);
};

const getValue = (obj: any, path?: string) => {
    if (obj == null || !path) return undefined;
    const parts = path.replace(/\[(\w+)\]/g, ".$1").split(".");
    return parts.reduce(
        (o: any, key: string) => (o == null ? undefined : o[key]),
        obj,
    );
};

const hasItems = computed<boolean>(() => props.items.length > 0);
const { t } = useI18n();

const summaryText = () => {
    if (!props.pagination) return "";
    return t('table_page_of', {
        page: props.pagination.page,
        total: props.pagination.total_pages,
    });
};
</script>
