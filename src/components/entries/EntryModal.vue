<template>
  <Modal
    :model-value="modelValue"
    :submit-label="form.id ? t('common_update') : t('common_create')"
    :title="form.id ? t('entry_modal_title_edit') : t('entry_modal_title_create')"
    @submit="createOrUpdate"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <form @submit.prevent="createOrUpdate">
        <Input
          id="entry-issue-number"
          v-model="form.issue_number"
          name="issue_number"
          :label="t('entry_modal_issue_number')"
          :placeholder="t('entry_modal_issue_placeholder')"
        />
        <Input
          id="entry-title"
          v-model="form.title"
          name="title"
          :label="t('entry_modal_title_label')"
          :placeholder="t('entry_modal_title_placeholder')"
        />
        <YearSelect
          id="entry-publication-year"
          v-model="form.pub_year"
          :label="t('entry_modal_publication_year')"
          name="pub_year"
        />
        <Input
          id="entry-publication-month"
          v-model="form.pub_month"
          type="number"
          name="pub_month"
          :label="t('entry_modal_publication_month')"
          min="1"
          max="12"
          :placeholder="t('entry_modal_publication_month')"
        />

        <ImageInput
          v-model="form.cover_path"
          class="mt-3"
          input-id="entry-cover"
          name="cover"
          :label="t('entry_modal_cover_image')"
          :preview-alt="t('entry_modal_cover_preview_alt')"
          :remove-label="t('entry_modal_remove_cover')"
        />

        <Select
          v-model="form.condition"
          :options="conditionOptions"
          class="mt-3"
          input-id="entry-condition"
          name="condition"
          :label="t('entry_modal_condition')"
        />

        <TagsInput
          v-model="form.tags"
          class="mt-3"
          input-id="entry-tags"
          name="tags"
        />

    </form>
  </Modal>
</template>

<script lang="ts" setup>
import { computed, ref, toRef, watch } from 'vue';
import { api } from '../../api';
import { COMIC_CONDITIONS } from '../../types';
import type { Entry } from '../../types';
import ImageInput from '../ImageInput.vue';
import Modal from '../Modal.vue';
import TagsInput from '../TagsInput.vue';
import YearSelect from '../YearSelect.vue';
import Select from '../Select.vue';
import { useToast } from '../../composables/useToast';
import { useI18n } from '../../i18n/useI18n';
import Input from '../Input.vue';

const props = withDefaults(defineProps<{
  modelValue?: boolean;
  series_id: number;
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
  condition: entry?.condition ?? null,
});

const form = ref<Entry>(createForm(props.entry));
const { showToast } = useToast();
const { t } = useI18n();

watch(modelValue, (val) => {
  if (val) form.value = createForm(props.entry);
});

const create = async () => {
  if (!form.value) return;
  try {
    await api.createEntry(form.value);
    showToast(t('entry_toast_create_success'), 'success');
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
    showToast(t('entry_toast_create_error'), 'error');
  }
};

const conditionOptions = computed(() => {
  return COMIC_CONDITIONS.map((c) => ({ value: c, label: c }));
});

const update = async () => {
  if (!form.value.id) return;
  try {
    await api.updateEntry(form.value.id, form.value);
    showToast(t('entry_toast_update_success'), 'success');
    emit('created');
    emit('update:modelValue', false);
  } catch (e) {
    console.error(e);
    showToast(t('entry_toast_update_error'), 'error');
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
