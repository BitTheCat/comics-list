<template>
    <label class="form-group">
        <span v-if="label" class="label">{{ label }}</span>
        <input
            v-bind="inputAttrs"
            :id="resolvedId"
            v-model="inputValue"
            class="input w-full"
            :class="attrs.class"
            :name="name"
            :type="type"
            :placeholder="placeholder"
            :disabled="disabled"
            :required="required"
        />
    </label>
</template>

<script setup lang="ts">
import { computed, useAttrs } from 'vue';

defineOptions({
  inheritAttrs: false,
});

const props = withDefaults(
  defineProps<{
    modelValue?: string | number | null;
    id?: string;
    inputId?: string;
    name?: string;
    label?: string;
    placeholder?: string;
    type?: 'text' | 'number' | 'email' | 'password' | 'search' | 'url' | 'tel';
    disabled?: boolean;
    required?: boolean;
  }>(),
  {
    modelValue: '',
    id: undefined,
    inputId: undefined,
    name: undefined,
    label: undefined,
    placeholder: undefined,
    type: 'text',
    disabled: false,
    required: false,
  },
);

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number | null): void;
}>();

const attrs = useAttrs();

const inputAttrs = computed(() => {
  const { class: _class, ...rest } = attrs;
  return rest;
});

const resolvedId = computed(() => props.id ?? props.inputId);

const normalizeValue = (rawValue: string): string | number | null => {
  if (props.type !== 'number') return rawValue;

  if (rawValue.trim() === '') return null;

  const parsedValue = Number(rawValue);
  return Number.isNaN(parsedValue) ? null : parsedValue;
};

const inputValue = computed({
  get: () => (props.modelValue == null ? '' : String(props.modelValue)),
  set: (value: string) => emit('update:modelValue', normalizeValue(value)),
});
</script>
