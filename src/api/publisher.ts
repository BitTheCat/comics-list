import { invoke } from '@tauri-apps/api/core';
import * as mock from '../mocks/publisher';

const isTauriRuntime = () =>
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);

async function invokeTauri<T>(cmd: string, payload?: Record<string, unknown>) {
  return invoke<T>(cmd, payload);
}

export const publisherApi = {
  listPublishers: (query = '') => {
    if (isTauriRuntime()) return invokeTauri('api_list_publishers', { name: query });
    return mock.getPublishers(query);
  },
  getPublisher: async (name: string) => {
    if (isTauriRuntime()) {
      const matches = await invokeTauri<Array<{ id: number; name: string }>>('api_list_publishers', { name });
      return matches[0] ?? null;
    }
    const matches = await mock.getPublishers(name);
    return matches[0] ?? null;
  },
  createPublisher: (payload: { name: string; notes?: string | null }) => {
    if (isTauriRuntime()) return invokeTauri<number>('api_create_publisher', { payload });
    return mock.createPublisher(payload);
  },
  findOrCreatePublisher: async (name: string) => {
    const normalizedName = name.trim();
    if (!normalizedName) throw new Error('Publisher name is required');

    const publishers = await publisherApi.listPublishers(normalizedName) as Array<{ id: number; name: string }>;
    const exactMatch = publishers.find(
      (publisher) =>
        publisher.name.toLocaleLowerCase() === normalizedName.toLocaleLowerCase(),
    );

    if (exactMatch) return exactMatch;

    const createdId = await publisherApi.createPublisher({ name: normalizedName });

    if (typeof createdId === 'number') {
      return { id: createdId, name: normalizedName };
    }

    return createdId as unknown as { id: number; name: string };
  },
};

