import { computed, ref } from 'vue';
import { api } from '../api';
import { messages, type Locale, type MessageKey } from './messages';

const getBrowserLocale = (): Locale => {
  if (typeof window === 'undefined') return 'it';
  const language = window.navigator.language.toLowerCase();
  return language.startsWith('it') ? 'it' : 'en';
};

const locale = ref<Locale>(getBrowserLocale());

// Applies the locale without persisting it (used at startup, from the config).
const applyLocale = (value: Locale) => {
  locale.value = value;
};

const setLocale = (value: Locale) => {
  applyLocale(value);
  api.updateConfig({ language: value }).catch((error) => console.error(error));
};

const t = (key: MessageKey, params?: Record<string, string | number>): string => {
  const template = String(messages[locale.value][key] ?? messages.en[key] ?? key);
  if (!params) return template;

  return Object.entries(params).reduce<string>(
    (acc, [paramKey, paramValue]) => acc.split(`{${paramKey}}`).join(String(paramValue)),
    template,
  );
};

const availableLocales = computed(() => [
  { value: 'it' as const, label: 'Italiano' },
  { value: 'en' as const, label: 'English' },
]);

export const useI18n = () => ({
  locale,
  applyLocale,
  setLocale,
  t,
  availableLocales,
});
