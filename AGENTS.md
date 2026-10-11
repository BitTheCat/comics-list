# Comics List — Agent Guide

> Conventions and context for AI agents working on this repository
> (Delta, Zed, etc.). Personal, machine-specific notes live in
> `AGENTS.local.md`, which is gitignored.

## Stack

- Desktop: **Tauri 2 + Rust** (`src-tauri/`)
- Frontend: **Vue 3 + TypeScript + Vite** (`src/`), Vue Router, Tailwind CSS, DaisyUI
- Persistence: **SQLite via rusqlite** (bundled), versioned SQL migrations
- Tests: Vitest + Testing Library (frontend), `cargo test` integration tests (Rust)

## Layout

```text
src/                       Vue frontend
src/api/index.ts           API adapter: calls Tauri when running in Tauri, mocks in browser
src/mocks/                 Mock data/functions mirroring the Rust API responses
src/types/                 Shared TS types (series, entry, publisher, pagination)
src/i18n/                  Translation scaffolding (Italian first)
src-tauri/src/commands/    Tauri commands (one module per entity)
src-tauri/src/db/          Data access + migration runner
src-tauri/src/models/      Serde payload/response structs
src-tauri/migrations/      Numbered SQL migration files + README
src-tauri/tests/           Rust integration tests (in-memory DB)
```

## Model & conventions

- Entities: **publishers → series → entries**. A series is one run of a work
  (e.g. "Spawn" can exist as multiple series under different publishers).
  Each series has a single `publisher_id`.
- **IDs are INTEGER PRIMARY KEY AUTOINCREMENT** in SQLite (series, entries,
  publishers). Frontend mocks must use numeric IDs to stay aligned with Rust.
- `entries` fields: `issue_number` (string, can be "1A"/"0"/"Special"),
  `title`, `pub_year`, `pub_month`, `cover_path`, `tags` (comma-joined in DB,
  array in JSON), `condition`.
- `condition` uses the classic comic grading codes: MT, NM/MT, NM, VF/NM, VF,
  FN/VF, FN, VG/FN, VG, GD/VG, GD, FR/GD, FR, PR. Enforced by a DB CHECK
  constraint (0004) and by the TS union `ComicCondition` (`src/types/entry.ts`).
- API responses between mocks and Rust must keep the **same shape**
  (e.g. `PaginatedResponse { items, pagination: { page, page_size,
  total_items, total_pages } }`). When Rust types change, update the mocks.
- UI copy is Italian where localized; keep new strings in `src/i18n`.

## Migrations

- Add a new numbered SQL file in `src-tauri/migrations/` (e.g. `0009_...sql`).
- Register it in `src-tauri/src/db/migrations.rs` with version, name, SQL,
  and a `should_run` precondition where needed.
- Never edit an already-applied migration; append a new one.
- Applied versions are recorded in `schema_migrations` and skipped on re-run.

## Database path & AppHandle

- The DB lives in the Tauri **app data directory**
  (`app.path().app_data_dir()/comics-list.db`), not in the working directory
  (fixed in `fix(db): store database in Tauri app data directory...`).
  Windows: `%APPDATA%\comics-list\comics-list.db`.
- Every Tauri command takes `app: tauri::AppHandle` (injected by Tauri, not
  passed from JS) and calls `init_db(&app)` per invocation.
- Known next refactor: open the DB connection **once** at startup and manage
  it as Tauri state (`app.manage(Mutex<Connection>)`), letting commands take
  `State<Mutex<Connection>>` instead of re-resolving the path each time.
- Known smell to fix: update payloads still use `id: Option<i64>`; it should
  become a required `i64` so an UPDATE always targets a real row.

## Development

```sh
npm install
npm run dev            # frontend-only, uses mocks (no Tauri needed)
npm run tauri:dev      # full desktop app (requires Rust + platform deps)
npm run test -- --run  # frontend tests
npm run build          # frontend production build
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml   # Rust tests
```

## Git workflow

- Work on feature branches, merge to `main` via PRs.
- Rebase (not merge) before pushing; never rewrite shared history without the
  maintainer's explicit approval.

## Roadmap

### Done

- UI: series/entries listing with pagination and search, create/edit/delete
  modals, condition field, entry details panel, delete confirmation, toasts,
  localized settings, theme toggle, logo and platform icons.
- Rust: module split (commands/db/models), versioned migration runner,
  base CRUD for series/publisher/entry, paginated listing, search filters,
  DB path fix (app data dir).

### Next

1. Align mock entry IDs to numeric (currently strings like `e1-1` in
   `src/mocks/api.ts` / `src/mocks/api_entries.ts`).
2. Make `id` required in update payloads (remove `Option<i64>`).
3. Refactor DB connection to Tauri managed state (open once at startup).

### Later (ordered)

1. **Multi-language** phase: complete i18n coverage (Italian/English), persist
   language choice in settings.
2. **Covers**: store cover images on disk in the app data dir, generate
   thumbnails (Rust side), show in lists/details; optional hash-based dedup.
3. Import/export of the collection (JSON/CSV) including cover files.
4. Settings persistence (theme, language, DB path info).
5. **Auto-update**: check for a new version at startup and let the app download
   and install it. Use the Tauri updater plugin (`tauri-plugin-updater`) with
   `tauri-plugin-process` for relaunch; requires a signed update manifest
   (minisign keypair) and a release endpoint (e.g. GitHub Releases) that serves
   the `latest.json` manifest. Surface the check in Settings (current version,
   "check for updates" button, download/install progress) and keep the version
   in `tauri.conf.json` / `Cargo.toml` / `package.json` in sync.
6. Stretch features: CBZ/CBR reader with reading progress, metadata fetch
   (e.g. ComicVine) for cover suggestions on demand, sync.
