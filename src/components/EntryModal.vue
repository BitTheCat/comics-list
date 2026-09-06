<template>
  <div v-if="modelValue" class="modal modal-open" @click="close">
    <div class="modal-box" @click.stop>
      <h3 class="text-lg font-bold">{{ form.id ? 'Edit' : 'Create' }} entry</h3>

      <form @submit.prevent="createOrUpdate">
        <input
          id="entry-issue-number"
          v-model="form.issue_number"
          class="input w-full"
          name="issue_number"
          placeholder="New entry number #"
        />
        <input
          id="entry-title"
          v-model="form.title"
          class="input w-full"
          name="title"
          placeholder="Title"
        />
        <YearSelect
          id="entry-publication-year"
          v-model="form.pub_year"
          label="Publication year"
          name="pub_year"
        />
        <input
          id="entry-publication-month"
          v-model.number="form.pub_month"
          class="input w-full"
          name="pub_month"
          placeholder="Publication month"
        />
        <div class="modal-action">
          <button class="btn" type="button" @click="close">Cancel</button>
          <button class="btn" type="submit">{{ form.id ? 'Update' : 'Create' }}</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, toRef, watch, onBeforeUnmount } from 'vue';
import { api } from '../api';
import type { Entry } from '../types';
import YearSelect from './YearSelect.vue';

const props = withDefaults(defineProps<{
  modelValue?: boolean;
  series_id: string;
  entry?: Entry | null;
}>(), {
  modelValue: false,
  entry: null,
});

const modelValue = toRef(props, 'modelValue');

const emit = defineEmits<{
  (e: 'created'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

const createForm = (entry?: Entry | null): Entry => ({
  id: entry?.id,
  issue_number: entry?.issue_number ?? '0',
  series_id: entry?.series_id ?? props.series_id,
  title: entry?.title ?? '',
  pub_year: entry?.pub_year ?? null,
  pub_month: entry?.pub_month ?? null,
  cover_path: entry?.cover_path ?? null,
  tags: entry?.tags ?? [],
});

const form = ref<Entry>(createForm(props.entry));

const close = () => {
  emit('update:modelValue', false);
};

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') close();
};

watch(modelValue, (val) => {
  if (val) {
    form.value = createForm(props.entry);
    window.addEventListener('keydown', onKeydown);
  } else {
    window.removeEventListener('keydown', onKeydown);
  }
});

onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown));

const create = async () => {
  if (!form.value) return;
  try {
    await api.createEntry(form.value);
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
  }
};

const update = async () => {
  if (!form.value.id) return;
  try {
    await api.updateEntry(form.value.id, form.value);
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
  }
};

const createOrUpdate = async () => {
  if (form.value.id) {
    await update();
  } else {
    await create();
  }
};
</script>
