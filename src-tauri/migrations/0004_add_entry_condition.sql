ALTER TABLE entries
ADD COLUMN condition TEXT
CHECK (
  condition IS NULL OR condition IN (
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
    'PR'
  )
);
