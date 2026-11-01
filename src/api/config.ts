import { invoke } from '@tauri-apps/api/core';
import * as mock from '../mocks/config';
import type { AppConfig, UpdateConfigPayload } from '../types';

const isTauriRuntime = () =>
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);

async function invokeTauri<T>(cmd: string, payload?: Record<string, unknown>) {
  return invoke<T>(cmd, payload);
}

export const configApi = {
  getConfig: (): Promise<AppConfig> => {
    if (isTauriRuntime()) return invokeTauri<AppConfig>('api_get_config');
    return mock.getConfig();
  },
  updateConfig: (payload: UpdateConfigPayload): Promise<AppConfig> => {
    if (isTauriRuntime()) return invokeTauri<AppConfig>('api_update_config', { payload });
    return mock.updateConfig(payload);
  },
};
