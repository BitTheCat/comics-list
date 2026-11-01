import { createApp } from 'vue';
import App from './App.vue';
import './styles/app.css';
import router from './router';
import { api } from './api';
import { useTheme } from './composables/useTheme';
import { useI18n } from './i18n/useI18n';

// Theme and language live in the config file: load them before mounting
// so the UI starts with the saved values.
const bootstrap = async () => {
  try {
    const config = await api.getConfig();
    if (config.theme === 'light' || config.theme === 'dark') {
      useTheme().applyTheme(config.theme);
    }
    if (config.language === 'it' || config.language === 'en') {
      useI18n().applyLocale(config.language);
    }
  } catch (error) {
    console.error('failed to load config', error);
  }

  createApp(App).use(router).mount('#app');
};

void bootstrap();
