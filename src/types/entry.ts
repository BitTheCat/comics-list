export interface Entry {
  id?: string;
  series_id: string;
  issue_number: string;
  title: string;
  pub_year: number | null;
  pub_month: number | null;
  cover_path: string | null;
  tags: string[];
}