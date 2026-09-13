import { invoke } from '@tauri-apps/api/core';
import * as mock from '../mocks/api';

const isTauriRuntime = () =>
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);

async function invokeTauri<T>(cmd: string, payload?: Record<string, unknown>) {
  return invoke<T>(cmd, payload);
}

export const api = {
  listSeries: async () => {
    if (isTauriRuntime()) {
      const response = await invokeTauri<mock.PaginatedResponse<mock.SeriesResource>>(
        'api_list_series_paginated',
        {
          request: { page: 1, page_size: 10 },
        },
      );
      return response.items;
    }
    return mock.getSeries();
  },
  listSeriesPaginated: async (request: mock.PaginationRequest = {}) => {
    const normalizedRequest: mock.PaginationRequest = {
      page: Math.max(1, Math.trunc(request.page ?? 1)),
      page_size: Math.max(1, Math.trunc(request.page_size ?? 10)),
      query: request.query,
    };

    if (isTauriRuntime()) {
      return invokeTauri<mock.PaginatedResponse<mock.SeriesResource>>(
        'api_list_series_paginated',
        { request: normalizedRequest },
      );
    }

    return mock.getSeriesPaginated(normalizedRequest);
  },
  createSeries: (p: any) => {
    if (isTauriRuntime()) return invokeTauri('api_create_series', { payload: p });
    return mock.createSeries(p);
  },
  updateSeries: (p: any) => {
    if (isTauriRuntime()) return Promise.reject(new Error('Updating series is not supported in Tauri yet'));
    return mock.updateSeries(p);
  },
  deleteSeries: (p: any) => {
    if (isTauriRuntime()) return Promise.reject(new Error('Deleting series is not supported in Tauri yet'));
    return mock.deleteSeries(p);
  },
  listPublishers: (query = '') => {
    if (isTauriRuntime()) return invokeTauri('api_list_publishers');
    return mock.getPublishers(query);
  },
  getPublisher: (id: string) => {
    if (isTauriRuntime()) return invokeTauri('api_get_publisher', { id });
    return mock.getPublisher(id);
  },
  createPublisher: (payload: { name: string; notes?: string | null }) => {
    if (isTauriRuntime()) return invokeTauri<number>('api_create_publisher', { payload });
    return mock.createPublisher(payload);
  },
  findOrCreatePublisher: async (name: string) => {
    const normalizedName = name.trim();
    if (!normalizedName) throw new Error('Publisher name is required');

    const publishers = await api.listPublishers(normalizedName) as Array<{ id: number | string; name: string }>;
    const exactMatch = publishers.find(
      (publisher) =>
        publisher.name.toLocaleLowerCase() === normalizedName.toLocaleLowerCase(),
    );

    if (exactMatch) return exactMatch;

    const createdId = await api.createPublisher({ name: normalizedName });

    if (typeof createdId === 'number') {
      return { id: createdId, name: normalizedName };
    }

    return createdId as unknown as { id: number | string; name: string };
  },
  getEntries: (seriesId: number) => {
    if (isTauriRuntime()) return invokeTauri('api_list_entries', { seriesId });
    return mock.getEntries(seriesId);
  },
  listEntriesPaginated: (
    seriesId: number,
    request: mock.PaginationRequest = {},
  ) => {
    if (isTauriRuntime()) {
      return invokeTauri('api_list_entries_paginated', { seriesId, request });
    }
    return mock.getEntriesPaginated(seriesId, request);
  },
  createEntry: (payload: any) => {
    if (isTauriRuntime()) return invokeTauri('api_create_entry', { payload });
    return import('../mocks/api_entries').then((m) => m.createEntry(payload));
  },
  updateEntry: (id: string, payload: any) => {
    if (isTauriRuntime()) return invokeTauri('api_update_entry', { id, payload });
    return import('../mocks/api_entries').then((m) => m.updateEntry(id, payload));
  },
  deleteEntry: (id: string) => {
    if (isTauriRuntime()) return invokeTauri('api_delete_entry', { id });
    return import('../mocks/api_entries').then((m) => m.deleteEntry(id));
  },
};

