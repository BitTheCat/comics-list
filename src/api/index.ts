import { invoke } from '@tauri-apps/api/core';
import * as mock from '../mocks/api';
import type { ListEntriesRequest, PaginatedResponse, PaginationRequest, SeriesResource } from '../types';

const isTauriRuntime = () =>
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);

async function invokeTauri<T>(cmd: string, payload?: Record<string, unknown>) {
  return invoke<T>(cmd, payload);
}

export const api = {
  // Series
  listSeriesPaginated: async (request: PaginationRequest = {}) => {
    const normalizedRequest: PaginationRequest = {
      page: Math.max(1, Math.trunc(request.page ?? 1)),
      page_size: Math.max(1, Math.trunc(request.page_size ?? 10)),
      query: request.query,
    };

    if (isTauriRuntime()) {
      return invokeTauri<PaginatedResponse<SeriesResource>>(
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
    if (isTauriRuntime()) return invokeTauri('api_update_series', { payload: p });
    return mock.updateSeries(p);
  },
  deleteSeries: (id: number) => {
    if (isTauriRuntime()) return invokeTauri('api_delete_series', { id });
    return mock.deleteSeries(id);
  },

  // Publishers
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

    const publishers = await api.listPublishers(normalizedName) as Array<{ id: number; name: string }>;
    const exactMatch = publishers.find(
      (publisher) =>
        publisher.name.toLocaleLowerCase() === normalizedName.toLocaleLowerCase(),
    );

    if (exactMatch) return exactMatch;

    const createdId = await api.createPublisher({ name: normalizedName });

    if (typeof createdId === 'number') {
      return { id: createdId, name: normalizedName };
    }

    return createdId as unknown as { id: number; name: string };
  },

  // Entries
  listEntriesPaginated: async (request: ListEntriesRequest = { series_id: 0 }) => {
    const normalizedRequest: ListEntriesRequest = {
      page: Math.max(1, Math.trunc(request.page ?? 1)),
      page_size: Math.max(1, Math.trunc(request.page_size ?? 10)),
      query: request.query,
      series_id: request.series_id,
    };
    
    if (isTauriRuntime()) {
      return invokeTauri('api_list_entries_paginated', {request: normalizedRequest});
    }
    return mock.getEntriesPaginated(request.series_id, request);
  },
  createEntry: async (payload: any) => {
    if (isTauriRuntime()) return invokeTauri('api_create_entry', { payload });
    return import('../mocks/api_entries').then((m) => m.createEntry(payload));
  },
  updateEntry: async (payload: any) => {
    if (isTauriRuntime()) return invokeTauri('api_update_entry', { payload });
    return import('../mocks/api_entries').then((m) => m.updateEntry(payload));
  },
  deleteEntry: async (id: number) => {
    if (isTauriRuntime()) return invokeTauri('api_delete_entry', { id });
    return import('../mocks/api_entries').then((m) => m.deleteEntry(id));
  },
  nextIssueNumber: async (series_id: number): Promise<string> => {
    if (isTauriRuntime()) return invokeTauri('api_retrieve_next_issue_number', { seriesId: series_id });
    return '1';
  },
  
};

