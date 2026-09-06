import { describe, it, expect } from 'vitest';
import {
  createPublisher,
  getPublisher,
  getPublishers,
  getSeries,
  mockPublishers,
} from '../mocks/api';

describe('mock api', () => {
  it('getSeries returns an array', async () => {
    const s = await getSeries();
    expect(Array.isArray(s)).toBe(true);
  });

  it('returns the entries count without embedding the entries', async () => {
    const series = await getSeries() as Array<{
      id: string;
      entries_count: number;
      entries?: unknown[];
    }>;

    expect(series.find((item) => item.id === 's1')?.entries_count).toBe(2);
    expect(series.find((item) => item.id === 's2')?.entries_count).toBe(1);
    expect(series.every((item) => item.entries === undefined)).toBe(true);
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

