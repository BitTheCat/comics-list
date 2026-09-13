// Test setup file for Vitest (happy-dom environment)
// Keep tests deterministic by forcing English locale.

if (typeof window !== 'undefined') {
  window.localStorage.setItem('comics-list.locale', 'en');
}

export {};
