// Mock API for entries (issues).
import { COMIC_CONDITIONS } from '../types';
import type { PaginatedResponse, PaginationRequest } from '../types';

export const mockEntries = [
  ...Array.from({ length: 32 }, (_, index) => ({
    id: index + 1,
    series_id: 1,
    issue_number: String(index + 1),
    title: `Spawn #${index + 1}`,
    pub_year: 1990 + Math.floor(index / 12),
    pub_month: (index % 12) + 1,
    cover_path: null,
    tags: ['spawn'],
    condition: COMIC_CONDITIONS[index % COMIC_CONDITIONS.length],
  })),
  ...Array.from({ length: 11 }, (_, index) => ({
    id: 33 + index,
    series_id: 2,
    issue_number: String(index + 1),
    title: `Spawn (reboot) #${index + 1}`,
    pub_year: 1996,
    pub_month: (index % 12) + 1,
    cover_path: null,
    tags: ['spawn'],
    condition: COMIC_CONDITIONS[(index + 2) % COMIC_CONDITIONS.length],
  })),
];

export async function createEntry(payload: any) {
  // payload should contain at least series_id and issue_number
  const id = Math.max(0, ...mockEntries.map((entry) => entry.id)) + 1;
  const entry = { id, ...payload };
  mockEntries.push(entry);
  return new Promise((resolve) => setTimeout(() => resolve(entry), 50));
}

export async function getEntries(seriesId: number) {
  const list = mockEntries.filter((e) => e.series_id === seriesId);
  return new Promise((resolve) => setTimeout(() => resolve(list), 50));
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

export async function updateEntry(payload: any) {
  const idx = mockEntries.findIndex((e: any) => e.id === payload.id);
  if (idx !== -1) {
    mockEntries[idx] = { ...mockEntries[idx], ...payload };
    return new Promise((resolve) => setTimeout(() => resolve(mockEntries[idx]), 50));
  }
  return new Promise((_, reject) => setTimeout(() => reject(new Error('not found')), 50));
}

export async function deleteEntry(id: number) {
  const idx = mockEntries.findIndex((e: any) => e.id === id);
  if (idx !== -1) {
    const removed = mockEntries.splice(idx, 1)[0];
    return new Promise((resolve) => setTimeout(() => resolve(removed), 50));
  }
  return new Promise((_, reject) => setTimeout(() => reject(new Error('not found')), 50));
}
