// Test setup file for Vitest (happy-dom environment)
// Keep tests deterministic by forcing English locale.
import { useI18n } from '../i18n/useI18n';

useI18n().applyLocale('en');
