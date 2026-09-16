<template>
    <button
        v-bind="buttonAttrs"
        :title="t('button_delete')"
        class="text-red-500 disabled:opacity-50"
        @click="showModal = true"
    >
        <Trash :size="16" />
    </button>

    <Modal
        v-model="showModal"
        :title="t('delete_modal_title')"
        :submit-label="t('common_delete')"
        :is-delete="true"
        @submit="confirmDelete"
    >
        <div class="flex flex-col items-center gap-3 text-center">
            <TriangleAlert :size="64" class="text-error" />
            <p>{{ t('delete_modal_message') }}</p>
        </div>
    </Modal>
</template>

<script setup lang="ts">
import { computed, ref, useAttrs } from 'vue';
import { useI18n } from '../i18n/useI18n';
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
const { t } = useI18n();

const emit = defineEmits<{
  (e: 'delete'): void;
}>();

const confirmDelete = () => {
  emit('delete');
  showModal.value = false;
};
</script>
