<template>
  <div v-if="modelValue" class="modal modal-open" @click="close">
    <div class="modal-box" @click.stop>
      <h3 class="text-lg font-bold">Create an entry</h3>

      <form @submit.prevent="create">
        <input class="input" v-model="entry.issue_number" placeholder="New entry number #" />
        <br />
        <input class="input" v-model="entry.title" placeholder="Title" />
        <br />
        <input class="input" v-model="entry.pub_year" placeholder="Publication year" />
        <br />
        <input class="input" v-model="entry.pub_month" placeholder="Publication month" />
        <br />
        <div class="modal-action" style="display:flex; gap:8px; justify-content:flex-end; margin-top:12px;">
          <button class="btn" type="button" @click="close">Cancel</button>
          <button class="btn" type="submit">Create</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, toRef, watch, onBeforeUnmount } from 'vue';
import { api } from '../api';
import { Entry } from '../types';

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  series_id: {
    type: String,
    required: true,
  },
});

const modelValue = toRef(props, 'modelValue');

const emit = defineEmits<{
  (e: 'created'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

const entry = ref<Entry>({
  issue_number: '0',
  series_id: props.series_id,
  title: '',
  pub_year: null,
  pub_month: null,
  cover_path: null,
  tags: [],
});

const close = () => {
  emit('update:modelValue', false);
};

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') close();
};

watch(modelValue, (val) => {
  if (val) window.addEventListener('keydown', onKeydown);
  else window.removeEventListener('keydown', onKeydown);
});

onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown));

const create = async () => {
  if (!entry.value) return;
  try {
    await api.createEntry(entry.value);
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
  }
};
</script>
