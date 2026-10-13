import { invoke } from '@tauri-apps/api/core';
import * as mock from '../mocks/entry';
import type { ListEntriesRequest } from '../types';

const isTauriRuntime = () =>
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);

async function invokeTauri<T>(cmd: string, payload?: Record<string, unknown>) {
  return invoke<T>(cmd, payload);
}

export const entryApi = {
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
    return import('../mocks/entry').then((m) => m.createEntry(payload));
  },
  updateEntry: async (payload: any) => {
    if (isTauriRuntime()) return invokeTauri('api_update_entry', { payload });
    return import('../mocks/entry').then((m) => m.updateEntry(payload));
  },
  deleteEntry: async (id: number) => {
    if (isTauriRuntime()) return invokeTauri('api_delete_entry', { id });
    return import('../mocks/entry').then((m) => m.deleteEntry(id));
  },
  nextIssueNumber: async (series_id: number): Promise<string> => {
    if (isTauriRuntime()) return invokeTauri('api_retrieve_next_issue_number', { seriesId: series_id });
    return '1';
  },
  
};

