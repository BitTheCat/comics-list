import * as mock from '../mocks/api';

const isTauri = !!(window && (window as any).__TAURI__);

async function invokeTauri(cmd: string, payload?: any) {
  // dynamic import to avoid errors when running in browser without Tauri
  const tauri = await import('@tauri-apps/api/tauri').catch(() => null);
  if (!tauri) throw new Error('Tauri API not available');
  return (tauri.invoke as any)(cmd, payload);
}

export const api = {
  listSeries: () => {
    if (isTauri) return invokeTauri('api_list_series');
    return mock.getSeries();
  },
  createSeries: (p: any) => {
    if (isTauri) return invokeTauri('api_create_series', { payload: p });
    return mock.createSeries(p);
  },
  getEntries: (seriesId: string) => {
    if (isTauri) return invokeTauri('api_list_entries', { seriesId });
    return mock.getEntries(seriesId);
  },
};

