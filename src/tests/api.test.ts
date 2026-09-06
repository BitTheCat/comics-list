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

