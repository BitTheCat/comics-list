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
          :min-year="minYear"
          :label="t('entry_modal_publication_year')"
          name="pub_year"
        />
        <Select
          v-model="form.pub_month"
          :options="monthOptions"
          class="mt-3"
          input-id="entry-publication-month"
          name="pub_month"
          :label="t('entry_modal_publication_month')"
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
import type { ComicCondition, Entry } from '../../types';
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
  seriesId: number;
  entry?: Entry | null;
  minYear?: number;
}>(), {
  modelValue: false,
  entry: null,
  minYear: 1950,
});

const modelValue = toRef(props, 'modelValue');

const emit = defineEmits<{
  (e: 'created'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

type EntryForm = {
  id?: number;
  issue_number: string;
  series_id: number;
  title: string;
  pub_year: number | null;
  pub_month: string | null;
  cover_path: string | null;
  tags: string[];
  condition: ComicCondition | null;
};

const createForm = (entry?: Entry | null): EntryForm => ({
  id: entry?.id,
  issue_number: entry?.issue_number ?? '0',
  series_id: entry?.series_id ?? props.seriesId,
  title: entry?.title ?? '',
  pub_year: entry?.pub_year ?? null,
  pub_month: entry?.pub_month != null ? String(entry.pub_month) : null,
  cover_path: entry?.cover_path ?? null,
  tags: entry?.tags ?? [],
  condition: entry?.condition ?? null,
});

const form = ref<EntryForm>(createForm(props.entry));
const { showToast } = useToast();
const { t, locale } = useI18n();

watch(modelValue, (val) => {
  if (val) form.value = createForm(props.entry);
});

const parseMonth = (value: string | null): number | null => {
  if (!value) return null;
  const month = Number(value);
  if (!Number.isInteger(month) || month < 1 || month > 12) return null;
  return month;
};

const toPayload = (currentForm: EntryForm): Entry => ({
  ...currentForm,
  pub_month: parseMonth(currentForm.pub_month),
});

const create = async () => {
  if (!form.value) return;
  try {
    await api.createEntry(toPayload(form.value));
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

const monthOptions = computed(() => {
  const formatter = new Intl.DateTimeFormat(locale.value === 'it' ? 'it-IT' : 'en-US', {
    month: 'long',
  });

  return Array.from({ length: 12 }, (_, index) => {
    const monthNumber = index + 1;
    const monthLabel = formatter
      .format(new Date(2000, index, 1))
      .replace(/^./, (char) => char.toUpperCase());

    return {
      value: String(monthNumber),
      label: monthLabel,
    };
  });
});

const update = async () => {
  if (!form.value.id) return;
  try {
    await api.updateEntry(toPayload(form.value));
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
