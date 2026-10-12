// Mock API for entries (issues) management — entries are managed separately from series
import { mockEntries } from './api';

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

