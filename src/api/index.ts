import * as mock from '../mocks/api';

const isTauri = !!(window && (window as any).__TAURI__);

async function invokeTauri(cmd: string, payload?: any) {
  // dynamic import to avoid errors when running in browser without Tauri
  const tauri = await import('@tauri-apps/api').catch(() => null);
  if (!tauri || !tauri.core || !tauri.core.invoke) throw new Error('Tauri API not available');
  return (tauri.core.invoke as any)(cmd, payload);
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
  listPublishers: (query = '') => {
    if (isTauri) return invokeTauri('api_list_publishers', { query });
    return mock.getPublishers(query);
  },
  getPublisher: (id: string) => {
    if (isTauri) return invokeTauri('api_get_publisher', { id });
    return mock.getPublisher(id);
  },
  createPublisher: (payload: { name: string; notes?: string | null }) => {
    if (isTauri) return invokeTauri('api_create_publisher', { payload });
    return mock.createPublisher(payload);
  },
  findOrCreatePublisher: async (name: string) => {
    const normalizedName = name.trim();
    if (!normalizedName) throw new Error('Publisher name is required');

    const publishers = await api.listPublishers(normalizedName) as mock.Publisher[];
    const exactMatch = publishers.find(
      (publisher) =>
        publisher.name.toLocaleLowerCase() === normalizedName.toLocaleLowerCase(),
    );

    return exactMatch ?? api.createPublisher({ name: normalizedName });
  },
  getEntries: (seriesId: string) => {
    if (isTauri) return invokeTauri('api_list_entries', { seriesId });
    return mock.getEntries(seriesId);
  },
  createEntry: (payload: any) => {
    if (isTauri) return invokeTauri('api_create_entry', { payload });
    return import('../mocks/api_entries').then((m) => m.createEntry(payload));
  },
  updateEntry: (id: string, payload: any) => {
    if (isTauri) return invokeTauri('api_update_entry', { id, payload });
    return import('../mocks/api_entries').then((m) => m.updateEntry(id, payload));
  },
  deleteEntry: (id: string) => {
    if (isTauri) return invokeTauri('api_delete_entry', { id });
    return import('../mocks/api_entries').then((m) => m.deleteEntry(id));
  },
};

