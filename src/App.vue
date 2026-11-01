<template>
  <div id="app" class="p-4">
    <header class="mb-4 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <img
          :src="logoUrl"
          alt=""
          class="h-10 w-10 rounded-lg"
        />
        <div class="flex flex-col">
          <h1 class="text-2xl font-bold leading-none">{{ t('app_title') }}</h1>
          <span class="text-xs text-muted leading-none">{{ t('app_subtitle') }}</span>
        </div>
      </div>

      <button class="btn btn-sm" type="button" @click="showSettings = true">
        <Settings2 :size="16" />
        {{ t('settings_open') }}
      </button>
    </header>

    <RouterView />

    <Modal
      v-model="showSettings"
      :title="t('settings_title')"
      :submit-label="t('common_save')"
      :cancel-label="t('common_close')"
      @submit="saveSettings"
    >
      <div class="space-y-4">
        <label class="form-group">
          <span class="label">{{ t('settings_language') }}</span>
          <select
            class="select w-full"
            :value="locale"
            @change="onLocaleChange"
          >
            <option
              v-for="option in availableLocales"
              :key="option.value"
              :value="option.value"
            >
              {{ option.label }}
            </option>
          </select>
        </label>

        <div class="form-group">
          <span class="label">{{ t('settings_theme') }}</span>
          <button class="btn btn-primary" type="button" @click="toggleTheme">
            <Sun v-if="theme === 'light'" :size="16" />
            <Moon v-else :size="16" />
            {{ theme === 'light' ? t('settings_theme_light') : t('settings_theme_dark') }}
          </button>
        </div>

        <Input
          id="settings-db-path"
          v-model="settingsForm.db_path"
          name="db_path"
          :label="t('settings_db_path')"
          :placeholder="t('settings_path_placeholder')"
        />
        <Input
          id="settings-covers-dir"
          v-model="settingsForm.covers_dir"
          name="covers_dir"
          :label="t('settings_covers_dir')"
          :placeholder="t('settings_path_placeholder')"
        />
        <p class="text-xs text-muted">{{ t('settings_restart_notice') }}</p>
      </div>
    </Modal>

    <ToastContainer />
  </div>
</template>

<script lang="ts" setup>
import { Moon, Settings2, Sun } from '@lucide/vue';
import { ref, watch } from 'vue';
import { api } from './api';
import logoUrl from './assets/logo.png';
import Input from './components/Input.vue';
import Modal from './components/Modal.vue';
import ToastContainer from './components/ToastContainer.vue';
import { useTheme } from './composables/useTheme';
import { useToast } from './composables/useToast';
import { useI18n } from './i18n/useI18n';
import type { Locale } from './i18n/messages';

const showSettings = ref(false);
const settingsForm = ref({ db_path: '', covers_dir: '' });
const { showToast } = useToast();

const { theme, toggleTheme } = useTheme();
const { locale, setLocale, t, availableLocales } = useI18n();

// Theme and language apply (and persist) immediately; paths are saved on submit.
watch(showSettings, async (open) => {
  if (!open) return;
  try {
    const config = await api.getConfig();
    settingsForm.value = {
      db_path: config.db_path ?? '',
      covers_dir: config.covers_dir ?? '',
    };
  } catch (error) {
    console.error(error);
  }
});

const saveSettings = async () => {
  try {
    await api.updateConfig({
      db_path: settingsForm.value.db_path,
      covers_dir: settingsForm.value.covers_dir,
    });
    showToast(t('settings_toast_save_success'), 'success');
    showSettings.value = false;
  } catch (error) {
    console.error(error);
    showToast(t('settings_toast_save_error'), 'error');
  }
};

const onLocaleChange = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value;
  if (value === 'it' || value === 'en') setLocale(value as Locale);
};
</script>
