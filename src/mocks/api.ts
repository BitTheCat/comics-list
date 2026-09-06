// Mock API for frontend development without Tauri/Rust backend.
export interface Publisher {
  id: string;
  name: string;
  notes: string | null;
}

export interface PaginationRequest {
  page?: number;
  page_size?: number;
  query?: string;
}

export interface PaginationMeta {
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
}

export interface PaginatedResponse<T> {
  items: T[];
  pagination: PaginationMeta;
}

export interface SeriesResource {
  id: number;
  title: string;
  description: string | null;
  publisher_id: string | null;
  publisher: Publisher | null;
  start_year: number | null;
  end_year: number | null;
  notes: string | null;
  entries_count: number;
}

type StoredSeries = Omit<SeriesResource, 'entries_count'>;

export const mockPublishers: Publisher[] = [
  { id: 'p1', name: 'Star Comics', notes: null },
  { id: 'p2', name: 'Panini Comics', notes: null },
  { id: 'p3', name: 'Cult Comics', notes: null },
];

const additionalSeries: StoredSeries[] = [
  'Batman',
  'Superman',
  'Dylan Dog',
  'Tex',
  'Dragon Ball',
  'One Piece',
  'Naruto',
  'The Walking Dead',
  'Invincible',
  'Hellboy',
  'Watchmen',
  'Sandman',
  'Thor',
  'Iron Man',
  'Amazing Spider-Man',
  'X-Men',
  'Fantastic Four',
  'Deadpool',
  'Judge Dredd',
  'The Punisher',
  'Savage Dragon',
  'Teenage Mutant Ninja Turtles',
  'Rat-Man',
].map((title, index) => {
  const publisher = mockPublishers[index % mockPublishers.length];

  return {
    id: index + 3,
    title,
    description: `Edizione italiana di ${title}.`,
    publisher_id: publisher.id,
    publisher,
    start_year: 1985 + (index % 35),
    end_year: index % 4 === 0 ? null : 1995 + (index % 25),
    notes: '',
  };
});

export const mockSeries: StoredSeries[] = [
  {
    id: 1,
    title: 'Spawn',
    description: 'Prima edizione italiana di Spawn.',
    publisher_id: 'p1',
    publisher: mockPublishers[0],
    start_year: 1990,
    end_year: 1995,
    notes: '',
  },
  {
    id: 2,
    title: 'Spawn',
    description: 'Edizione italiana successiva di Spawn.',
    publisher_id: 'p2',
    publisher: mockPublishers[1],
    start_year: 1996,
    end_year: null,
    notes: '',
  },
  ...additionalSeries,
];

export const mockEntries = [
  ...Array.from({ length: 32 }, (_, index) => ({
    id: `e1-${index + 1}`,
    series_id: 1,
    issue_number: String(index + 1),
    title: `Spawn #${index + 1}`,
    pub_year: 1990 + Math.floor(index / 12),
    pub_month: (index % 12) + 1,
    cover_path: null,
    tags: ['spawn'],
  })),
  ...Array.from({ length: 11 }, (_, index) => ({
    id: `e2-${index + 1}`,
    series_id: 2,
    issue_number: String(index + 1),
    title: `Spawn (reboot) #${index + 1}`,
    pub_year: 1996,
    pub_month: (index % 12) + 1,
    cover_path: null,
    tags: ['spawn'],
  })),
];

export async function getSeries(): Promise<SeriesResource[]> {
  const series = mockSeries.map((item) => ({
    ...item,
    entries_count: mockEntries.filter((entry) => entry.series_id === item.id).length,
  }));

  return new Promise(resolve => setTimeout(() => resolve(series), 100));
}

export async function getSeriesPaginated(
  request: PaginationRequest = {},
): Promise<PaginatedResponse<SeriesResource>> {
  const page = Math.max(1, Math.trunc(request.page ?? 1));
  const pageSize = Math.min(100, Math.max(1, Math.trunc(request.page_size ?? 10)));
  const query = request.query?.trim().toLocaleLowerCase() ?? '';
  const allSeries = await getSeries();
  const filteredSeries = query
    ? allSeries.filter((series) =>
        [
          series.title,
          series.description,
          series.publisher?.name,
        ].some((value) => value?.toLocaleLowerCase().includes(query)),
      )
    : allSeries;
  const totalItems = filteredSeries.length;
  const totalPages = Math.max(1, Math.ceil(totalItems / pageSize));
  const firstItemIndex = (page - 1) * pageSize;

  return new Promise((resolve) =>
    setTimeout(
      () =>
        resolve({
          items: filteredSeries.slice(firstItemIndex, firstItemIndex + pageSize),
          pagination: {
            page,
            page_size: pageSize,
            total_items: totalItems,
            total_pages: totalPages,
          },
        }),
      100,
    ),
  );
}

export async function getEntries(seriesId: number) {
  const entries = mockEntries.filter((entry) => entry.series_id === seriesId);
  return new Promise(resolve => setTimeout(() => resolve(entries), 50));
}

export async function getEntriesPaginated(
  seriesId: number,
  request: PaginationRequest = {},
): Promise<PaginatedResponse<(typeof mockEntries)[number]>> {
  const page = Math.max(1, Math.trunc(request.page ?? 1));
  const pageSize = Math.min(100, Math.max(1, Math.trunc(request.page_size ?? 10)));
  const query = request.query?.trim().toLocaleLowerCase() ?? '';
  const seriesEntries = mockEntries.filter((entry) => entry.series_id === seriesId);
  const filteredEntries = query
    ? seriesEntries.filter((entry) =>
        [entry.issue_number, entry.title, ...entry.tags].some((value) =>
          value.toLocaleLowerCase().includes(query),
        ),
      )
    : seriesEntries;
  const totalItems = filteredEntries.length;
  const totalPages = Math.max(1, Math.ceil(totalItems / pageSize));
  const firstItemIndex = (page - 1) * pageSize;

  return new Promise((resolve) =>
    setTimeout(
      () =>
        resolve({
          items: filteredEntries.slice(firstItemIndex, firstItemIndex + pageSize),
          pagination: {
            page,
            page_size: pageSize,
            total_items: totalItems,
            total_pages: totalPages,
          },
        }),
      100,
    ),
  );
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
  const id = Math.max(0, ...mockSeries.map((series) => series.id)) + 1;
  const publisher = payload.publisher_id
    ? await getPublisher(payload.publisher_id)
    : null;
  const item = { ...payload, id, publisher };
  mockSeries.push(item);
  return new Promise(resolve => setTimeout(() => resolve(id), 50));
}

export async function updateSeries(payload: any) {
  const index = mockSeries.findIndex((item) => item.id === payload.id);
  if (index === -1) throw new Error('Series not found');
  mockSeries[index] = { ...mockSeries[index], ...payload };
  return new Promise(resolve => setTimeout(() => resolve(payload.id), 50));
}

export async function deleteSeries(payload: any) {
  const index = mockSeries.findIndex((item) => item.id === payload.id);
  if (index === -1) throw new Error('Series not found');
  mockSeries.splice(index, 1);
  return new Promise(resolve => setTimeout(() => resolve(payload.id), 50));
}


