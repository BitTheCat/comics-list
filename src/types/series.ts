import { PaginationRequest } from './pagination';
import type { Publisher } from './publisher';

export interface Series {
  id?: number;
  title: string;
  description: string | null;
  publisher: Publisher | null;
  start_year: number | null;
  end_year: number | null;
  notes: string | null;
}

export interface SeriesResource extends Series {
  id: number;
  entries_count: number;
}

export interface ListEntriesRequest extends PaginationRequest {
  series_id: number;
}