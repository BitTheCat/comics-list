// Additional mock API for entries (issues) management
import { mockSeries } from './api';

export async function createEntry(payload: any) {
  // payload should contain at least series_id and issue_number
  const id = 'e' + Math.random().toString(36).slice(2, 9);
  const entry = { id, ...payload };
  const s = mockSeries.find((x) => x.id === payload.series_id);
  if (s) {
    s.entries.push(entry);
  }
  return new Promise((resolve) => setTimeout(() => resolve(entry), 50));
}

export async function updateEntry(id: string, payload: any) {
  for (const s of mockSeries) {
    const idx = s.entries.findIndex((e: any) => e.id === id);
    if (idx !== -1) {
      s.entries[idx] = { ...s.entries[idx], ...payload };
      return new Promise((resolve) => setTimeout(() => resolve(s.entries[idx]), 50));
    }
  }
  return new Promise((_, reject) => setTimeout(() => reject(new Error('not found')), 50));
}

export async function deleteEntry(id: string) {
  for (const s of mockSeries) {
    const idx = s.entries.findIndex((e: any) => e.id === id);
    if (idx !== -1) {
      const removed = s.entries.splice(idx, 1)[0];
      return new Promise((resolve) => setTimeout(() => resolve(removed), 50));
    }
  }
  return new Promise((_, reject) => setTimeout(() => reject(new Error('not found')), 50));
}

