import { ref } from 'vue';

export type ToastType = 'success' | 'error';

export interface ToastItem {
  id: number;
  type: ToastType;
  message: string;
}

const toasts = ref<ToastItem[]>([]);
let nextId = 1;

const removeToast = (id: number) => {
  toasts.value = toasts.value.filter((toast) => toast.id !== id);
};

const showToast = (message: string, type: ToastType = 'success', duration = 3000) => {
  const id = nextId++;
  toasts.value.push({ id, type, message });

  window.setTimeout(() => {
    removeToast(id);
  }, duration);
};

export const useToast = () => ({
  toasts,
  showToast,
  removeToast,
});
