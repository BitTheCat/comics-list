<template>
  <div v-if="modelValue" class="modal modal-open" @mousedown.self="close">
    <div class="modal-box text-base-content">
      <h3 v-if="title" class="text-lg font-bold">{{ title }}</h3>

      <slot />

      <div class="modal-action">
        <button class="btn" type="button" @click="close">
          {{ resolvedCancelLabel }}
        </button>
        <button
          v-if="showSubmit"
          class="btn"
          :class="isDelete ? 'btn-error' : 'btn-primary'"
          type="button"
          @click="submit"
        >
          {{ resolvedSubmitLabel }}
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, onBeforeUnmount, toRef, watch } from 'vue';
import { useI18n } from '../i18n/useI18n';

const props = withDefaults(defineProps<{
  modelValue?: boolean;
  title?: string;
  submitLabel?: string;
  showSubmit?: boolean;
  cancelLabel?: string;
  isDelete?: boolean;
}>(), {
  modelValue: false,
  submitLabel: undefined,
  showSubmit: true,
  cancelLabel: undefined,
  isDelete: false,
});

const modelValue = toRef(props, 'modelValue');
const { t } = useI18n();

const resolvedCancelLabel = computed(() => props.cancelLabel ?? t('common_cancel'));
const resolvedSubmitLabel = computed(() =>
  props.submitLabel ?? (props.isDelete ? t('common_delete') : t('common_update')),
);

const emit = defineEmits<{
  (e: 'submit'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

const close = () => {
  emit('update:modelValue', false);
};

const submit = () => {
  emit('submit');
};

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') close();
};

watch(modelValue, (isOpen) => {
  if (isOpen) {
    window.addEventListener('keydown', onKeydown);
  } else {
    window.removeEventListener('keydown', onKeydown);
  }
}, { immediate: true });

onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown));
</script>
