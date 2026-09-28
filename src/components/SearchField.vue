<template>
    <div class="my-2 flex gap-2">
        <input
            v-model="localSearchQuery"
            type="text"
            :placeholder="t('entries_search_placeholder')"
            class="input w-full"
            @keyup.enter="emit('search')"
        />
        <button class="btn btn-primary" type="button" @click="emit('search')">
            {{ t("search") }}
        </button>
    </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "../i18n/useI18n";

const props = defineProps<{
    modelValue: string | null;
}>();

const emit = defineEmits<{
    (event: "update:modelValue", value: string | null): void;
    (event: "search"): void;
}>();

const { t } = useI18n();

const localSearchQuery = computed({
    get: () => props.modelValue,
    set: (value: string) => emit("update:modelValue", value || null),
});

</script>
