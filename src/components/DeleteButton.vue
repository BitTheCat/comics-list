<template>
    <button
        v-bind="buttonAttrs"
        title="Delete"
        class="text-red-500 disabled:opacity-50"
        @click="showModal = true"
    >
        <Trash :size="16" />
    </button>

    <Modal
        v-model="showModal"
        title=""
        submit-label="Delete"
        :is-delete="true"
        @submit="confirmDelete"
    >
        <div class="flex flex-col items-center gap-3 text-center">
            <TriangleAlert :size="64" class="text-red-700" />
            <p>Are you sure you want to delete this entry?</p>
        </div>
    </Modal>
</template>

<script setup lang="ts">
import { computed, ref, useAttrs } from 'vue';
import { Trash, TriangleAlert } from '@lucide/vue';
import Modal from './Modal.vue';

defineOptions({
  inheritAttrs: false,
});

const attrs = useAttrs();

const buttonAttrs = computed(() => {
  const { onClick: _onClick, ...rest } = attrs;
  return rest;
});

const showModal = ref(false);

const emit = defineEmits<{
  (e: 'delete'): void;
}>();

const confirmDelete = () => {
  emit('delete');
  showModal.value = false;
};
</script>
