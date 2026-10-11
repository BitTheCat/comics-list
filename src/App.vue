<template>
  <div id="app" class="p-4">
    <header class="mb-4 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <img
          :src="logoUrl"
          alt=""
          class="h-10 w-10 rounded-lg"
        />
        <div>
          <h1 class="text-2xl font-bold">{{ t('app_title') }}</h1>
          <span class="mt-0 text-xs text-muted">{{ t('app_subtitle') }}</span>
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
      :show-submit="false"
      :cancel-label="t('common_close')"
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
      </div>
    </Modal>

    <ToastContainer />
  </div>
</template>

<script lang="ts" setup>
import { Moon, Settings2, Sun } from '@lucide/vue';
import { ref } from 'vue';
import logoUrl from './assets/logo.png';
import Modal from './components/Modal.vue';
import ToastContainer from './components/ToastContainer.vue';
import { useTheme } from './composables/useTheme';
import { useI18n } from './i18n/useI18n';
import type { Locale } from './i18n/messages';

const showSettings = ref(false);

const { theme, toggleTheme } = useTheme();
const { locale, setLocale, t, availableLocales } = useI18n();

const onLocaleChange = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value;
  if (value === 'it' || value === 'en') setLocale(value as Locale);
};
</script>
