<template>
  <fieldset class="fieldset">
    <legend v-if="label" class="fieldset-legend">{{ label }}</legend>
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
        :alt="previewAlt"
        class="h-40 w-28 rounded object-cover shadow"
      />
      <button class="btn btn-sm" type="button" @click="removeImage">
        {{ removeLabel }}
      </button>
    </div>
  </fieldset>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';

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
  label: 'Image',
  accept: 'image/*',
  maxSizeMb: 5,
  previewAlt: 'Image preview',
  removeLabel: 'Remove image',
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | null): void;
}>();

const fileInput = ref<HTMLInputElement | null>(null);
const error = ref('');
const helpText = computed(
  () => `PNG, JPEG, WebP or GIF, up to ${props.maxSizeMb} MB.`,
);

const onFileSelected = (event: Event) => {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  error.value = '';

  if (!file) return;
  if (!file.type.startsWith('image/')) {
    error.value = 'Select a valid image file.';
    input.value = '';
    return;
  }
  if (file.size > props.maxSizeMb * 1024 * 1024) {
    error.value = `The image must be smaller than ${props.maxSizeMb} MB.`;
    input.value = '';
    return;
  }

  const reader = new FileReader();
  reader.onload = () => {
    if (typeof reader.result === 'string') emit('update:modelValue', reader.result);
  };
  reader.onerror = () => {
    error.value = 'The image could not be read.';
  };
  reader.readAsDataURL(file);
};

const removeImage = () => {
  emit('update:modelValue', null);
  error.value = '';
  if (fileInput.value) fileInput.value.value = '';
};
</script>
