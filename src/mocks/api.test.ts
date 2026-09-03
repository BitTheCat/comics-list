import { describe, it, expect } from 'vitest';
import { getSeries } from './api';

describe('mock api', () => {
  it('getSeries returns an array', async () => {
    const s = await getSeries();
    expect(Array.isArray(s)).toBe(true);
  });
});

