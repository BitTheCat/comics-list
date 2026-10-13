// Mock API for series.
import type { PaginatedResponse, PaginationRequest, SeriesResource } from '../types';
import { mockPublishers, getPublisher } from './publisher';
import { mockEntries } from './entry';

type StoredSeries = Omit<SeriesResource, 'entries_count'>;

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
    publisher: mockPublishers[0],
    start_year: 1990,
    end_year: 1995,
    notes: '',
  },
  {
    id: 2,
    title: 'Spawn',
    description: 'Edizione italiana successiva di Spawn.',
    publisher: mockPublishers[1],
    start_year: 1996,
    end_year: null,
    notes: '',
  },
  ...additionalSeries,
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
  const query = request.query?.trim().toLocaleLowerCase() ?? null;
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

export async function deleteSeries(id: number) {
  const index = mockSeries.findIndex((item) => item.id === id);
  if (index === -1) throw new Error('Series not found');
  mockSeries.splice(index, 1);
  return new Promise(resolve => setTimeout(() => resolve(id), 50));
}
