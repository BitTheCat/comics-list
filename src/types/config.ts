export interface AppConfig {
  portable: boolean;
  db_path: string | null;
  covers_dir: string | null;
  theme: string | null;
  language: string | null;
}

// Partial update: missing fields are left untouched, an empty string clears a value.
export interface UpdateConfigPayload {
  db_path?: string;
  covers_dir?: string;
  theme?: string;
  language?: string;
}
