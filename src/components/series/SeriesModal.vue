<template>
  <Modal
    :model-value="modelValue"
    :submit-label="form.id ? t('common_update') : t('common_create')"
    :title="form.id ? t('series_modal_title_edit') : t('series_modal_title_create')"
    @submit="createOrUpdate"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <form @submit.prevent="createOrUpdate">
        <Input
          id="series-title"
          v-model="form.title"
          name="title"
          :label="t('series_modal_title_label')"
          :placeholder="t('series_modal_title_placeholder')"
        />
        <div class="form-group">
          <Input
            id="series-publisher"
            v-model="form.publisher_name"
            name="publisher_name"
            :label="t('series_modal_publisher_label')"
            list="publisher-options"
            :placeholder="t('series_modal_publisher_placeholder')"
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
            {{ t('series_modal_publisher_hint') }}
          </p>
        </div>
        <div class="form-group">
          <textarea
                      id="series-description"
                      v-model="form.description"
                      class="textarea w-full"
                      name="description"
                      :placeholder="t('series_modal_description_placeholder')"
                    ></textarea>
        </div>
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <YearSelect
            id="series-start-year"
            v-model="form.start_year"
            :label="t('series_modal_start_year')"
            name="start_year"
          />
          <YearSelect
            id="series-end-year"
            v-model="form.end_year"
            :label="t('series_modal_end_year')"
            name="end_year"
          />
        </div>
    </form>
  </Modal>
</template>

<script lang="ts" setup>
import { ref, toRef, watch } from 'vue';
import { api } from '../../api';
import type { Publisher, Series } from '../../types';
import Modal from '../Modal.vue';
import YearSelect from '../YearSelect.vue';
import Input from '../Input.vue';
import { useToast } from '../../composables/useToast';
import { useI18n } from '../../i18n/useI18n';

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
  publisher_name: series?.publisher?.name ?? null,
  start_year: series?.start_year ?? null,
  end_year: series?.end_year ?? null,
  notes: series?.notes ?? '',
});

const form = ref<Form>(createForm(props.series));
const { showToast } = useToast();
const { t } = useI18n();


const publisherSuggestions = ref<Publisher[]>([]);
let searchSequence = 0;


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
    showToast(t('series_toast_create_success'), 'success');
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
    showToast(t('series_toast_create_error'), 'error');
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
    showToast(t('series_toast_update_success'), 'success');
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
    showToast(t('series_toast_update_error'), 'error');
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
  if (val) form.value = createForm(props.series);
});
</script>
