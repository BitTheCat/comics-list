import { invoke } from '@tauri-apps/api/core';
import * as mock from '../mocks/series';
import type { PaginatedResponse, PaginationRequest, SeriesResource } from '../types';

const isTauriRuntime = () =>
  typeof window !== 'undefined' &&
  ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);

async function invokeTauri<T>(cmd: string, payload?: Record<string, unknown>) {
  return invoke<T>(cmd, payload);
}

export const seriesApi = {
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
};

