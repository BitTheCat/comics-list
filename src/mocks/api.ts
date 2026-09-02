// Mock API for frontend development without Tauri/Rust backend.
export const mockSeries = [
  {
    id: 's1',
    title: 'Spawn',
    series_title: 'Spawn (Casa 1)',
    publisher: { id: 'p1', name: 'Casa 1' },
    start_year: 1990,
    end_year: 1995,
    notes: '',
    entries: [
      { id: 'e1', series_id: 's1', issue_number: '1', title: 'Spawn #1', pub_year: 1990, pub_month: 5, cover_path: null, tags: ['spawn'] },
      { id: 'e2', series_id: 's1', issue_number: '2', title: 'Spawn #2', pub_year: 1990, pub_month: 6, cover_path: null, tags: [] }
    ]
  },
  {
    id: 's2',
    title: 'Spawn',
    series_title: 'Spawn (Casa 2)',
    publisher: { id: 'p2', name: 'Casa 2' },
    start_year: 1996,
    end_year: null,
    notes: '',
    entries: [
      { id: 'e3', series_id: 's2', issue_number: '1', title: 'Spawn (reboot) #1', pub_year: 1996, pub_month: 2, cover_path: null, tags: [] }
    ]
  }
];

export async function getSeries() {
  // simulate async
  return new Promise(resolve => setTimeout(() => resolve(mockSeries), 100));
}

export async function getEntries(seriesId: string) {
  const s = mockSeries.find((s) => s.id === seriesId);
  return new Promise(resolve => setTimeout(() => resolve(s ? s.entries : []), 50));
}

export async function createSeries(payload: any) {
  const id = 's' + Math.random().toString(36).slice(2, 9);
  const item = { ...payload, id, entries: [] };
  mockSeries.push(item);
  return new Promise(resolve => setTimeout(() => resolve(id), 50));
}

