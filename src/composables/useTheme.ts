import { ref } from 'vue';

export type Theme = 'light' | 'dark';

const STORAGE_KEY = 'comics-list.theme';

const resolveInitialTheme = (): Theme => {
  if (typeof window === 'undefined') return 'light';

  const stored = window.localStorage.getItem(STORAGE_KEY);
  if (stored === 'light' || stored === 'dark') return stored;

  const fromAttribute = document.documentElement.getAttribute('data-theme');
  if (fromAttribute === 'light' || fromAttribute === 'dark') return fromAttribute;

  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
};

const theme = ref<Theme>(resolveInitialTheme());

const applyTheme = (value: Theme) => {
  theme.value = value;
  if (typeof window !== 'undefined') {
    document.documentElement.setAttribute('data-theme', value);
    window.localStorage.setItem(STORAGE_KEY, value);
  }
};

if (typeof window !== 'undefined') {
  applyTheme(theme.value);
}

const toggleTheme = () => {
  applyTheme(theme.value === 'dark' ? 'light' : 'dark');
};

export const useTheme = () => ({
  theme,
  applyTheme,
  toggleTheme,
});
