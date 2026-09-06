<template>
  <Select
    v-model="model"
    :label="label"
    :placeholder="placeholder"
    :name="name"
    :disabled="disabled"
    :required="required"
    :options="years"
  />
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import Select from './Select.vue';

const props = withDefaults(defineProps<{
  modelValue?: number | null;
  label?: string;
  placeholder?: string;
  name?: string;
  minYear?: number;
  maxYear?: number;
  disabled?: boolean;
  required?: boolean;
}>(), {
  modelValue: null,
  label: undefined,
  placeholder: 'Not specified',
  name: undefined,
  minYear: 1950,
  maxYear: () => new Date().getFullYear() + 1,
  disabled: false,
  required: false,
});

const emit = defineEmits<{
  (event: 'update:modelValue', value: number | null): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: (value: string | number | null) => {
    emit('update:modelValue', typeof value === 'number' ? value : null);
  },
});

const years = computed(() => {
  const firstYear = Math.max(props.minYear, props.maxYear);
  const lastYear = Math.min(props.minYear, props.maxYear);

  return Array.from(
    { length: firstYear - lastYear + 1 },
    (_, index) => firstYear - index,
  );
});
</script>
