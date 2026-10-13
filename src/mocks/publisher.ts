// Mock API for publishers.
import type { Publisher } from '../types';

export const mockPublishers: Publisher[] = [
  { id: 1, name: 'Star Comics', notes: null },
  { id: 2, name: 'Panini Comics', notes: null },
  { id: 3, name: 'Cult Comics', notes: null },
];

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

export async function getPublisher(id: number) {
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
    id: Math.max(0, ...mockPublishers.map((item) => item.id)) + 1,
    name,
    notes: payload.notes?.trim() || null,
  };
  mockPublishers.push(publisher);

  return new Promise<Publisher>((resolve) =>
    setTimeout(() => resolve(publisher), 50),
  );
}
