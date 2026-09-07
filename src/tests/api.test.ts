import { describe, it, expect } from 'vitest';
import { COMIC_CONDITIONS } from '../types';
import {
  createPublisher,
  getEntriesPaginated,
  getPublisher,
  getPublishers,
  getSeries,
  getSeriesPaginated,
  mockPublishers,
} from '../mocks/api';

describe('mock api', () => {
  it('getSeries returns an array', async () => {
    const s = await getSeries();
    expect(Array.isArray(s)).toBe(true);
  });

  it('returns the entries count without embedding the entries', async () => {
    const series = await getSeries() as Array<{
      id: number;
      entries_count: number;
      entries?: unknown[];
    }>;

    expect(series.find((item) => item.id === 1)?.entries_count).toBe(32);
    expect(series.find((item) => item.id === 2)?.entries_count).toBe(11);
    expect(series.every((item) => item.entries === undefined)).toBe(true);
  });

  it('returns a page of series with pagination metadata', async () => {
    const response = await getSeriesPaginated({ page: 2, page_size: 10 });

    expect(response.items).toHaveLength(10);
    expect(response.pagination).toEqual({
      page: 2,
      page_size: 10,
      total_items: 25,
      total_pages: 3,
    });
  });

  it('filters paginated series before calculating pagination', async () => {
    const response = await getSeriesPaginated({
      page: 1,
      page_size: 10,
      query: 'spawn',
    });

    expect(response.items).toHaveLength(2);
    expect(response.items.every((series) => series.title === 'Spawn')).toBe(true);
    expect(response.pagination.total_items).toBe(2);
    expect(response.pagination.total_pages).toBe(1);
  });

  it('returns a page of entries for a series with pagination metadata', async () => {
    const response = await getEntriesPaginated(1, { page: 2, page_size: 10 });

    expect(response.items).toHaveLength(10);
    expect(response.items[0].issue_number).toBe('11');
    expect(response.items.every((entry) => entry.series_id === 1)).toBe(true);
    expect(response.pagination).toEqual({
      page: 2,
      page_size: 10,
      total_items: 32,
      total_pages: 4,
    });
    expect(
      response.items.every((entry) => COMIC_CONDITIONS.includes(entry.condition)),
    ).toBe(true);
  });

  it('searches publishers without considering case', async () => {
    const publishers = await getPublishers('panini');

    expect(publishers.map((publisher) => publisher.name)).toContain('Panini Comics');
  });

  it('creates and retrieves a publisher', async () => {
    const publisher = await createPublisher({ name: '  Nuovo Editore  ' });
    const retrievedPublisher = await getPublisher(publisher.id);

    expect(retrievedPublisher).toEqual(publisher);
    expect(publisher.name).toBe('Nuovo Editore');
  });

  it('reuses a publisher when its name already exists', async () => {
    const initialCount = mockPublishers.length;
    const publisher = await createPublisher({ name: 'PANINI COMICS' });

    expect(publisher.id).toBe('p2');
    expect(mockPublishers).toHaveLength(initialCount);
  });
});

