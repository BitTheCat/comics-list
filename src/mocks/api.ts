// Mock API for frontend development without Tauri/Rust backend.
export interface Publisher {
  id: string;
  name: string;
  notes: string | null;
}

export const mockPublishers: Publisher[] = [
  { id: 'p1', name: 'Star Comics', notes: null },
  { id: 'p2', name: 'Panini Comics', notes: null },
  { id: 'p3', name: 'Cult Comics', notes: null },
];

export const mockSeries = [
  {
    id: 's1',
    title: 'Spawn',
    description: 'Prima edizione italiana di Spawn.',
    publisher_id: 'p1',
    publisher: mockPublishers[0],
    start_year: 1990,
    end_year: 1995,
    notes: '',
  },
  {
    id: 's2',
    title: 'Spawn',
    description: 'Edizione italiana successiva di Spawn.',
    publisher_id: 'p2',
    publisher: mockPublishers[1],
    start_year: 1996,
    end_year: null,
    notes: '',
  }
];

export const mockEntries = [
    { id: 'e1', series_id: 's1', issue_number: '1', title: 'Spawn #1', pub_year: 1990, pub_month: 5, cover_path: null, tags: ['spawn'] },
    { id: 'e2', series_id: 's1', issue_number: '2', title: 'Spawn #2', pub_year: 1990, pub_month: 6, cover_path: null, tags: [] },
    { id: 'e3', series_id: 's2', issue_number: '1', title: 'Spawn (reboot) #1', pub_year: 1996, pub_month: 2, cover_path: null, tags: [] }
];

export async function getSeries() {
  // simulate async
  return new Promise(resolve => setTimeout(() => resolve(mockSeries), 100));
}

export async function getEntries(seriesId: string) {
  const s = mockEntries.filter((e) => e.series_id === seriesId);
  return new Promise(resolve => setTimeout(() => resolve(s ? s : []), 50));
}

export async function getPublishers(query = '') {
  const normalizedQuery = query.trim().toLocaleLowerCase();
  const publishers = normalizedQuery
    ? mockPublishers.filter((publisher) =>
        publisher.name.toLocaleLowerCase().includes(normalizedQuery),
      )
    : mockPublishers;

  return new Promise<Publisher[]>((resolve) =>
    setTimeout(() => resolve([...publishers]), 50),
  );
}

export async function getPublisher(id: string) {
  const publisher = mockPublishers.find((item) => item.id === id) ?? null;
  return new Promise<Publisher | null>((resolve) =>
    setTimeout(() => resolve(publisher), 50),
  );
}

export async function createPublisher(payload: { name: string; notes?: string | null }) {
  const name = payload.name.trim();
  if (!name) throw new Error('Publisher name is required');

  const existing = mockPublishers.find(
    (publisher) => publisher.name.toLocaleLowerCase() === name.toLocaleLowerCase(),
  );
  if (existing) return existing;

  const publisher: Publisher = {
    id: 'p' + Math.random().toString(36).slice(2, 9),
    name,
    notes: payload.notes?.trim() || null,
  };
  mockPublishers.push(publisher);

  return new Promise<Publisher>((resolve) =>
    setTimeout(() => resolve(publisher), 50),
  );
}

export async function createSeries(payload: any) {
  const id = 's' + Math.random().toString(36).slice(2, 9);
  const publisher = payload.publisher_id
    ? await getPublisher(payload.publisher_id)
    : null;
  const item = { ...payload, id, publisher };
  mockSeries.push(item);
  return new Promise(resolve => setTimeout(() => resolve(id), 50));
}

