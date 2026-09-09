<template>
  <div v-if="modelValue" class="modal modal-open" @click="close">
    <div class="modal-box" @click.stop>
      <h3 class="text-lg font-bold">{{ title }}</h3>

      <slot />

      <div class="modal-action">
        <button class="btn" type="button" @click="close">
          {{ cancelLabel }}
        </button>
        <button
          v-if="showSubmit"
          class="btn"
          :class="isDelete ? 'btn-error' : 'btn-info'"
          type="button"
          @click="submit"
        >
          {{ submitLabel }}
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { toRef, watch, onBeforeUnmount } from 'vue';

const props = withDefaults(defineProps<{
  modelValue?: boolean;
  title?: string;
  submitLabel?: string;
  showSubmit?: boolean;
  cancelLabel?: string;
  isDelete?: boolean;
}>(), {
  modelValue: false,
  submitLabel: 'Save',
  showSubmit: true,
  cancelLabel: 'Cancel',
  isDelete: false,
});

const modelValue = toRef(props, 'modelValue');

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
