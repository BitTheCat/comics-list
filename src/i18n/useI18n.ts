import { computed, ref } from 'vue';
import { messages, type Locale, type MessageKey } from './messages';

const STORAGE_KEY = 'comics-list.locale';

const getBrowserLocale = (): Locale => {
  if (typeof window === 'undefined') return 'it';
  const language = window.navigator.language.toLowerCase();
  return language.startsWith('it') ? 'it' : 'en';
};

const initialLocale = (): Locale => {
  if (typeof window === 'undefined') return 'it';
  const stored = window.localStorage.getItem(STORAGE_KEY);
  if (stored === 'it' || stored === 'en') return stored;
  return getBrowserLocale();
};

const locale = ref<Locale>(initialLocale());

const setLocale = (value: Locale) => {
  locale.value = value;
  if (typeof window !== 'undefined') {
    window.localStorage.setItem(STORAGE_KEY, value);
  }
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
  setLocale,
  t,
  availableLocales,
});
