<template>
    <aside class="sticky top-4 hidden min-h-64 rounded-box border border-base-content/10 bg-base-100 p-5 md:block">
        <div v-if="entry" class="space-y-5">
            <div>
                <div class="flex justify-between gap-3 rounded-box border border-base-content/10 bg-primary p-2 text-primary-content">
                    <h2 class="text-xl font-bold">
                        {{ entry.title || `Issue #${entry.issue_number}` }}
                    </h2>
                    <div class="flex gap-2">
                        <EditButton class="btn btn-sm" @click="emit('edit', entry)" />
                        <DeleteButton
                            v-if="entry.id"
                            class="btn btn-sm"
                            @delete="emit('delete', entry)"
                        />
                    </div>
                </div>
            </div>

            <div class="flex items-start justify-between gap-10 mb-0">
                <dl class="grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-sm text-left">
                    <dt class="font-semibold">{{ t('entry_details_issue') }}</dt>
                    <dd>#{{ entry.issue_number }}</dd>
                    <dt class="font-semibold">{{ t('entry_details_title') }}</dt>
                    <dd>{{ entry.title || "—" }}</dd>
                    <dt class="font-semibold">{{ t('entry_details_publication') }}</dt>
                    <dd>{{ publication }}</dd>
                    <dt class="font-semibold">{{ t('entry_details_cover') }}</dt>
                    <dd>{{ entry.cover_path ? t('entry_details_cover_available') : t('entry_details_cover_not_available') }}</dd>
                    <dt class="font-semibold">{{ t('entry_details_condition') }}</dt>
                    <dd>{{ entry.condition || "—" }}</dd>    
                </dl>
                
    
                <div>
                    <img
                        v-if="entry?.cover_path"
                        :src="entry.cover_path"
                        alt="Cover"
                        class="h-50 object-contain cursor-pointer"
                        @click="emit('view', entry)"
                    />
                </div>
            </div>
            <div v-if="entry.tags.length">
                <p class="mb-2 text-sm font-semibold">{{ t('entry_details_tags') }}</p>
                <div class="flex flex-wrap gap-2">
                    <Badge
                        v-for="tag in entry.tags"
                        :key="tag"
                        :label="tag"
                    />
                </div>
            </div>
        </div>

        <div v-else class="flex min-h-54 items-center justify-center text-center text-base-content/50">
            <p>{{ t('entry_details_empty') }}</p>
        </div>
    </aside>
</template>

<script setup lang="ts">
import type { Entry } from "../../types";
import Badge from "../Badge.vue";
import DeleteButton from "../DeleteButton.vue";
import EditButton from "../EditButton.vue";
import { useI18n } from '../../i18n/useI18n';

withDefaults(defineProps<{
    entry?: Entry | null;
    publication?: string;
}>(), {
    entry: null,
    publication: "—",
});

const { t } = useI18n();

const emit = defineEmits<{
    (e: "view", entry: Entry): void;
    (e: "edit", entry: Entry): void;
    (e: "delete", entry: Entry): void;
}>();
</script>
