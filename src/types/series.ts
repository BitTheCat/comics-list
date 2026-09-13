export interface Series {
  id?: number;
  title: string;
  description: string;
  publisher: Publisher;
  start_year: number;
  end_year: number;
  notes: string;
  entries_count?: number;
}

export interface Publisher {
  id: number | string;
  name: string;
}
