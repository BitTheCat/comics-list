<template>
    <aside class="sticky top-4 hidden min-h-64 rounded-box border border-base-content/10 bg-base-100 p-5 md:block">
        <div v-if="entry" class="space-y-5">
            <div>
                <p class="text-sm text-base-content/60">Entry details</p>
                <div class="flex justify-between gap-3">
                    <h2 class="text-xl font-bold">
                        {{ entry.title || `Issue #${entry.issue_number}` }}
                    </h2>
                    <div class="flex gap-2">
                        <EditButton class="btn btn-sm" @click="emit('edit', entry)" />
                        <DeleteButton
                            v-if="entry.id"
                            class="btn btn-sm"
                            @click="emit('delete', entry)"
                        />
                    </div>
                </div>
            </div>

            <div class="flex justify-between gap-10">
                <dl class="grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-sm">
                    <dt class="font-semibold">Issue</dt>
                    <dd>#{{ entry.issue_number }}</dd>
                    <dt class="font-semibold">Title</dt>
                    <dd>{{ entry.title || "—" }}</dd>
                    <dt class="font-semibold">Publication</dt>
                    <dd>{{ publication }}</dd>
                    <dt class="font-semibold">Cover</dt>
                    <dd>{{ entry.cover_path ? "Available" : "Not available" }}</dd>
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
                <p class="mb-2 text-sm font-semibold">Tags</p>
                <div class="flex flex-wrap gap-2">
                    <span
                        v-for="tag in entry.tags"
                        :key="tag"
                        class="badge badge-outline"
                    >
                        {{ tag }}
                    </span>
                </div>
            </div>
        </div>

        <div v-else class="flex min-h-54 items-center justify-center text-center text-base-content/50">
            <p>Select an entry from the table to preview its details.</p>
        </div>
    </aside>
</template>

<script setup lang="ts">
import type { Entry } from "../../types";
import DeleteButton from "../DeleteButton.vue";
import EditButton from "../EditButton.vue";
import ViewButton from "../ViewButton.vue";

withDefaults(defineProps<{
    entry?: Entry | null;
    publication?: string;
}>(), {
    entry: null,
    publication: "—",
});

const emit = defineEmits<{
    (e: "view", entry: Entry): void;
    (e: "edit", entry: Entry): void;
    (e: "delete", entry: Entry): void;
}>();
</script>
