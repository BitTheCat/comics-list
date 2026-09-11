<template>
  <label class="form-group">
    <span v-if="resolvedLabel" class="label">{{ resolvedLabel }}</span>
    <input
      ref="fileInput"
      :id="inputId"
      class="file-input w-full"
      type="file"
      :name="name"
      :accept="accept"
      @change="onFileSelected"
    />
    <p class="label">{{ helpText }}</p>
    <p v-if="error" class="text-error text-sm">{{ error }}</p>
    <div v-if="modelValue" class="mt-2 flex items-start gap-3">
      <img
        :src="modelValue"
        :alt="resolvedPreviewAlt"
        class="h-40 w-28 rounded object-cover shadow"
      />
      <button class="btn btn-sm" type="button" @click="removeImage">
        {{ resolvedRemoveLabel }}
      </button>
    </div>
  </label>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from '../i18n/useI18n';

const props = withDefaults(defineProps<{
  modelValue?: string | null;
  inputId?: string;
  name?: string;
  label?: string;
  accept?: string;
  maxSizeMb?: number;
  previewAlt?: string;
  removeLabel?: string;
}>(), {
  modelValue: null,
  inputId: undefined,
  name: 'image',
  label: undefined,
  accept: 'image/*',
  maxSizeMb: 5,
  previewAlt: undefined,
  removeLabel: undefined,
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | null): void;
}>();

const fileInput = ref<HTMLInputElement | null>(null);
const error = ref('');
const { t } = useI18n();
const resolvedLabel = computed(() => props.label ?? t('image_input_label'));
const resolvedPreviewAlt = computed(() => props.previewAlt ?? t('image_input_preview_alt'));
const resolvedRemoveLabel = computed(() => props.removeLabel ?? t('image_input_remove'));
const helpText = computed(() => t('image_input_help', { maxSize: props.maxSizeMb }));

const onFileSelected = (event: Event) => {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  error.value = '';

  if (!file) return;
  if (!file.type.startsWith('image/')) {
    error.value = t('image_input_invalid_file');
    input.value = '';
    return;
  }
  if (file.size > props.maxSizeMb * 1024 * 1024) {
    error.value = t('image_input_too_large', { maxSize: props.maxSizeMb });
    input.value = '';
    return;
  }

  const reader = new FileReader();
  reader.onload = () => {
    if (typeof reader.result === 'string') emit('update:modelValue', reader.result);
  };
  reader.onerror = () => {
    error.value = t('image_input_read_error');
  };
  reader.readAsDataURL(file);
};

const removeImage = () => {
  emit('update:modelValue', null);
  error.value = '';
  if (fileInput.value) fileInput.value.value = '';
};
</script>
