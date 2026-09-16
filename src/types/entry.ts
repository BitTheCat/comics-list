export const COMIC_CONDITIONS = [
  'MT',
  'NM/MT',
  'NM',
  'VF/NM',
  'VF',
  'FN/VF',
  'FN',
  'VG/FN',
  'VG',
  'GD/VG',
  'GD',
  'FR/GD',
  'FR',
  'PR',
] as const;

export type ComicCondition = (typeof COMIC_CONDITIONS)[number];

export interface Entry {
  id?: number;
  series_id: number;
  issue_number: string;
  title: string;
  pub_year: number | null;
  pub_month: number | null;
  cover_path: string | null;
  tags: string[];
  condition: ComicCondition | null;
}
