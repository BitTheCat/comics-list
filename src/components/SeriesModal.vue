<template>
  <!-- DaisyUI-style modal using divs (not native <dialog>) -->
  <div v-if="modelValue" class="modal modal-open" @click="close">
    <div class="modal-box" @click.stop>
      <h3 class="text-lg font-bold">
        {{ form.id ? 'Update' : 'Create' }} Series
      </h3>

      <form @submit.prevent="createOrUpdate">
        <div class="form-group">
          <input
                      id="series-title"
                      v-model="form.title"
                      class="input w-full"
                      name="title"
                      placeholder="New series title"
                    />
        </div>
        <div class="form-group">
          <input
            id="series-publisher"
            v-model="form.publisher_name"
            class="input w-full"
            name="publisher_name"
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
          <p class="text-xs text-muted">
            If the publisher does not exist, it will be created.
          </p>
        </div>
        <div class="form-group">
          <textarea
                      id="series-description"
                      v-model="form.description"
                      class="textarea w-full"
                      name="description"
                      placeholder="Description (optional)"
                    ></textarea>
        </div>
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <YearSelect
            id="series-start-year"
            v-model="form.start_year"
            label="Start year"
            name="start_year"
          />
          <YearSelect
            id="series-end-year"
            v-model="form.end_year"
            label="End year"
            name="end_year"
          />
        </div>
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
import type { Publisher, Series } from '../types';
import YearSelect from './YearSelect.vue';

const props = withDefaults(defineProps<{
  modelValue?: boolean;
  series?: Series | null;
}>(), {
  modelValue: false,
  series: null,
});

const modelValue = toRef(props, 'modelValue');

const emit = defineEmits<{
  (e: 'created'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

interface Form {
  id?: number;
  title: string;
  description: string;
  publisher_name: string | null;
  start_year: number | null;
  end_year: number | null;
  notes: string;
}

const createForm = (series?: Series | null): Form => ({
  id: series?.id,
  title: series?.title ?? '',
  description: series?.description ?? '',
  publisher_name: series?.publisher.name ?? null,
  start_year: series?.start_year ?? null,
  end_year: series?.end_year ?? null,
  notes: series?.notes ?? '',
});

const form = ref<Form>(createForm(props.series));


const publisherSuggestions = ref<Publisher[]>([]);
let searchSequence = 0;

const close = () => {
  emit('update:modelValue', false);
};

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') close();
};


onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown));

const searchPublishers = async () => {
  const sequence = ++searchSequence;
  const publishers = await api.listPublishers(
    form.value.publisher_name ? form.value.publisher_name : ''
  ) as Publisher[];
  if (sequence === searchSequence) publisherSuggestions.value = publishers;
};

const create = async () => {
  if (!form.value.title.trim() || !form.value.publisher_name?.trim()) return;
  try {
    const publisher = await api.findOrCreatePublisher(form.value.publisher_name) as Publisher;
    await api.createSeries({
      ...form.value,
      publisher_id: publisher.id,
    });
    publisherSuggestions.value = [];
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
  }
};

const update = async () => {
  if (!form.value.title.trim() || !form.value.publisher_name?.trim()) return;
  try {
    const publisher = await api.findOrCreatePublisher(form.value.publisher_name) as Publisher;
    await api.updateSeries({
      ...form.value,
      publisher_id: publisher.id,
    });
    publisherSuggestions.value = [];
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
  }
};


const createOrUpdate = async () => {
  if (!form.value.id) {
    await create();
  } else {
    await update();
  }
};

watch(modelValue, (val) => {
  if (val) {
    form.value = createForm(props.series);
    window.addEventListener('keydown', onKeydown);
  } else {
    window.removeEventListener('keydown', onKeydown);
  }
});
</script>
