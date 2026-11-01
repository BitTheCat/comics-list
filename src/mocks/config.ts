// Mock API for the app config (mirrors the Rust commands, in memory only).
import type { AppConfig, UpdateConfigPayload } from '../types';

const mockConfig: AppConfig = {
  portable: true,
  db_path: null,
  covers_dir: null,
  theme: null,
  language: null,
};

const normalize = (value: string): string | null => value.trim() || null;

export async function getConfig(): Promise<AppConfig> {
  return new Promise((resolve) =>
    setTimeout(() => resolve({ ...mockConfig }), 50),
  );
}

export async function updateConfig(payload: UpdateConfigPayload): Promise<AppConfig> {
  if (payload.db_path !== undefined) mockConfig.db_path = normalize(payload.db_path);
  if (payload.covers_dir !== undefined) mockConfig.covers_dir = normalize(payload.covers_dir);
  if (payload.theme !== undefined) mockConfig.theme = normalize(payload.theme);
  if (payload.language !== undefined) mockConfig.language = normalize(payload.language);

  return new Promise((resolve) =>
    setTimeout(() => resolve({ ...mockConfig }), 50),
  );
}
