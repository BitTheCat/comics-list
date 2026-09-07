# Database migrations

Each schema change lives in a numbered SQL file and is applied in ascending
order by `run_migrations` in `src/db.rs`.

Applied migrations are recorded in the SQLite `schema_migrations` table, so
they are not executed again on later application starts.

Current migrations:

1. `0001_initial_schema.sql`: creates the original tables.
2. `0002_rename_series_title_to_description.sql`: renames the series field.
3. `0003_integer_series_ids.sql`: converts series IDs and related foreign keys
   to integers.
4. `0004_add_entry_condition.sql`: adds the standard comic condition grade.

To add a migration:

1. Create the next numbered `.sql` file.
2. Add its version, name, SQL file and optional precondition to the migration
   list in `run_migrations`.
3. Never edit a migration that has already been released; add a new one.
