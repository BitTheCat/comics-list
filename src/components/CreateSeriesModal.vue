<template>
  <!-- DaisyUI-style modal using divs (not native <dialog>) -->
  <div v-if="modelValue" class="modal modal-open" @click="close">
    <div class="modal-box" @click.stop>
      <h3 class="text-lg font-bold">Create work</h3>
      <p class="py-4">Press ESC key or click outside to close</p>

      <form @submit.prevent="create">
        <input class="input" v-model="title" placeholder="New series title" />
        <br />
        <input
          class="input"
          v-model="publisherName"
          list="publisher-options"
          placeholder="Publisher"
          @input="searchPublishers"
        />
        <datalist id="publisher-options">
          <option
            v-for="publisher in publisherSuggestions"
            :key="publisher.id"
            :value="publisher.name"
          />
        </datalist>
        <p class="text-sm opacity-70">
          If the publisher does not exist, it will be created.
        </p>
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
import type { Publisher } from '../mocks/api';

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
const publisherName = ref('');
const publisherSuggestions = ref<Publisher[]>([]);
let searchSequence = 0;

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

const searchPublishers = async () => {
  const sequence = ++searchSequence;
  const publishers = await api.listPublishers(publisherName.value) as Publisher[];
  if (sequence === searchSequence) publisherSuggestions.value = publishers;
};

const create = async () => {
  if (!title.value.trim() || !publisherName.value.trim()) return;
  try {
    const publisher = await api.findOrCreatePublisher(publisherName.value) as Publisher;
    await api.createSeries({
      title: title.value.trim(),
      description: description.value || null,
      publisher_id: publisher.id,
      start_year: null,
      end_year: null,
      notes: null,
    });
    title.value = '';
    description.value = '';
    publisherName.value = '';
    publisherSuggestions.value = [];
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
  }
};
</script>
