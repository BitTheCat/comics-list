<template>
  <label class="form-group">
    <span v-if="label" class="label">{{ label }}</span>
    <select
      :id="id"
      v-model="model"
      class="select w-full"
      :name="name"
      :disabled="disabled"
      :required="required"
    >
      <option v-if="placeholder" :value="null">
        {{ placeholder }}
      </option>
      <option
        v-for="(option, index) in options"
        :key="getOptionKey(option, index)"
        :value="getOptionValue(option)"
        :disabled="getOptionDisabled(option)"
      >
        {{ getOptionLabel(option) }}
      </option>
    </select>
  </label>
</template>

<script lang="ts" setup>
import { computed } from 'vue';

type SelectValue = string | number | null;
type SelectOption = SelectValue | {
  label: string;
  value: SelectValue;
  disabled?: boolean;
};

const props = withDefaults(defineProps<{
  modelValue?: SelectValue;
  options: SelectOption[];
  label?: string;
  placeholder?: string;
  id?: string;
  name?: string;
  disabled?: boolean;
  required?: boolean;
}>(), {
  modelValue: null,
  label: undefined,
  placeholder: undefined,
  id: undefined,
  name: undefined,
  disabled: false,
  required: false,
});

const emit = defineEmits<{
  (event: 'update:modelValue', value: SelectValue): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: (value: SelectValue) => emit('update:modelValue', value),
});

const isOptionObject = (option: SelectOption): option is Exclude<SelectOption, SelectValue> =>
  typeof option === 'object' && option !== null;

const getOptionValue = (option: SelectOption): SelectValue =>
  isOptionObject(option) ? option.value : option;

const getOptionLabel = (option: SelectOption): string =>
  isOptionObject(option) ? option.label : String(option ?? '');

const getOptionDisabled = (option: SelectOption): boolean =>
  isOptionObject(option) && Boolean(option.disabled);

const getOptionKey = (option: SelectOption, index: number): string | number => {
  const value = getOptionValue(option);
  return value == null ? `option-${index}` : `${value}-${index}`;
};
</script>
