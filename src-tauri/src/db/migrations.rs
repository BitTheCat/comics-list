use rusqlite::{params, Connection, Result};

struct Migration {
    version: i64,
    name: &'static str,
    sql: &'static str,
    should_run: fn(&Connection) -> Result<bool>,
}

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
    )?;

    let migrations = [
        Migration {
            version: 1,
            name: "initial schema",
            sql: include_str!("../../migrations/0001_initial_schema.sql"),
            should_run: |_| Ok(true),
        },
        Migration {
            version: 2,
            name: "rename series title to description",
            sql: include_str!("../../migrations/0002_rename_series_title_to_description.sql"),
            should_run: |connection| column_exists(connection, "series", "series_title"),
        },
        Migration {
            version: 3,
            name: "use integer series identifiers",
            sql: include_str!("../../migrations/0003_integer_series_ids.sql"),
            should_run: |connection| {
                let id_type: String = connection.query_row(
                    "SELECT type FROM pragma_table_info('series') WHERE name = 'id'",
                    [],
                    |row| row.get(0),
                )?;
                Ok(!id_type.eq_ignore_ascii_case("INTEGER"))
            },
        },
        Migration {
            version: 4,
            name: "add entry condition",
            sql: include_str!("../../migrations/0004_add_entry_condition.sql"),
            should_run: |connection| Ok(!column_exists(connection, "entries", "condition")?),
        },
        Migration {
            version: 5,
            name: "remove old column for entry",
            sql: include_str!("../../migrations/0005_remove_old_column_entry.sql"),
            should_run: |connection| {
                Ok(column_exists(connection, "entries", "volume")?
                    && column_exists(connection, "entries", "rating")?
                    && column_exists(connection, "entries", "read_status")?)
            },
        },
        Migration {
            version: 6,
            name: "remove notes column from publishers",
            sql: include_str!("../../migrations/0006_remove_notes_column_publisher.sql"),
            should_run: |connection| column_exists(connection, "publishers", "notes"),
        },
        Migration {
            version: 7,
            name: "use integer publisher identifiers",
            sql: include_str!("../../migrations/0007_integer_publishers_ids.sql"),
            should_run: |connection| {
                let publisher_id_type = column_type(connection, "publishers", "id")?;
                let series_publisher_id_type = column_type(connection, "series", "publisher_id")?;
                Ok(!publisher_id_type.eq_ignore_ascii_case("INTEGER")
                    || !series_publisher_id_type.eq_ignore_ascii_case("INTEGER"))
            },
        },
        Migration {
            version: 8,
            name: "use integer entry identifiers",
            sql: include_str!("../../migrations/0008_integer_entries_ids.sql"),
            should_run: |connection| {
                let entries_id_type = column_type(connection, "entries", "id")?;
                Ok(!entries_id_type.eq_ignore_ascii_case("INTEGER"))
            },
        },
    ];

    for migration in migrations {
        let was_applied: bool = conn.query_row(
            "SELECT EXISTS(
                SELECT 1 FROM schema_migrations WHERE version = ?1
            )",
            [migration.version],
            |row| row.get(0),
        )?;
        if was_applied {
            continue;
        }

        if (migration.should_run)(conn)? {
            conn.execute_batch(migration.sql)?;
        }
        conn.execute(
            "INSERT INTO schema_migrations (version, name) VALUES (?1, ?2)",
            params![migration.version, migration.name],
        )?;
    }

    Ok(())
}

fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool> {
    conn.query_row(
        "SELECT EXISTS(
            SELECT 1 FROM pragma_table_info(?1) WHERE name = ?2
        )",
        params![table, column],
        |row| row.get(0),
    )
}

fn column_type(conn: &Connection, table: &str, column: &str) -> Result<String> {
    conn.query_row(
        "SELECT type FROM pragma_table_info(?1) WHERE name = ?2",
        params![table, column],
        |row| row.get(0),
    )
}
