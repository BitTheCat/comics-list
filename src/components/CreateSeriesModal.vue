<template>
  <!-- DaisyUI-style modal using divs (not native <dialog>) -->
  <div v-if="modelValue" class="modal modal-open" @click="close">
    <div class="modal-box" @click.stop>
      <h3 class="text-lg font-bold">Create work</h3>
      <p class="py-4">Press ESC key or click outside to close</p>

      <form @submit.prevent="create">
        <input class="input" v-model="title" placeholder="New work title" />
        <br />
        <textarea class="textarea" v-model="description" placeholder="Description (optional)"></textarea>
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

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
});

const modelValue = toRef(props, 'modelValue');

const emit = defineEmits<{
  (e: 'created'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

const title = ref('');
const description = ref('');

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
  if (!title.value) return;
  try {
    await api.createSeries({
      title: title.value,
      series_title: description.value || null,
      publisher_id: null,
      start_year: null,
      end_year: null,
      notes: null,
    });
    title.value = '';
    description.value = '';
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
  }
};
</script>
