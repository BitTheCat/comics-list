import { ref } from 'vue';
import { api } from '../api';

export type Theme = 'light' | 'dark';

const resolveInitialTheme = (): Theme => {
  if (typeof window === 'undefined') return 'light';

  const fromAttribute = document.documentElement.getAttribute('data-theme');
  if (fromAttribute === 'light' || fromAttribute === 'dark') return fromAttribute;

  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
};

const theme = ref<Theme>(resolveInitialTheme());

// Applies the theme without persisting it (used at startup, from the config).
const applyTheme = (value: Theme) => {
  theme.value = value;
  if (typeof window !== 'undefined') {
    document.documentElement.setAttribute('data-theme', value);
  }
};

if (typeof window !== 'undefined') {
  applyTheme(theme.value);
}

const toggleTheme = () => {
  const next: Theme = theme.value === 'dark' ? 'light' : 'dark';
  applyTheme(next);
  api.updateConfig({ theme: next }).catch((error) => console.error(error));
};

export const useTheme = () => ({
  theme,
  applyTheme,
  toggleTheme,
});
