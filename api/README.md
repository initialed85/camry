# api

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

diesel print-schema > src/schema.rs

cargo install diesel_cli_ext

```

## Generating models

```sh
./generate.sh
```

The script is `diesel print-schema` -> `diesel_ext`, and both steps have sharp edges:

- **`use diesel::prelude::*;`** - `diesel_ext` only prints that line when you pass
  `-t/--add-table-name` (that flag is what sets its internal `diesel_macro_use` switch). With
  plain `-m` you get `#[derive(Queryable, Identifiable)]` with nothing to import them from.
  `generate.sh` passes `-I diesel::prelude::*` instead, which keeps the models free of
  `#[diesel(table_name = ...)]` until we want `Insertable`/`Selectable`.
- **Unknown column types** - `diesel_ext` never fails on a type it cannot name; it writes a
  `/* TODO: unknown type Point */` comment and only warns on stderr. `generate.sh` maps the
  three types we need with `-M` and turns any remaining warning into a hard failure.
- **Schema noise** - `--except-tables 'spatial_ref_sys|schema_migrations'` keeps PostGIS and
  Diesel's own bookkeeping out of `src/schema.rs`. `print-schema` dumps tables only, so the
  `video_with_seen_person` and `v_audio_event_hour` views have to be declared by hand if the
  Rust API wants them.
- **`WITH_DIFF=1`** - the `diesel migration generate initial --diff-schema` step is off by
  default. The tables are created by the Go API's auto-migrate, so the diff is empty and every
  run littered `migrations/` with a no-op migration.

## Column types we own

`centroid` (`pg_catalog.point`) and `bounding_box` (`pg_catalog.polygon`) are native Postgres
geometric types, not PostGIS - no Camry column uses `geometry`. Diesel supports neither, and
no maintained crate covers `polygon` on Diesel 2, so `src/pg_types.rs` holds the codecs:
`PgPoint`, `PgPolygon` (a vertex `Vec`; `polygon` is an open shape, `path` is the closed one)
and `PgDuration` (an `interval` as nanoseconds, mirroring the Go API's `time.Duration`). All
three serialize into the JSON shapes already published in `schema/openapi.json`, so the
frontend and the generated TS/Python/Go clients are unaffected.

Their tests live in `tests/pg_types.rs` and bind/select through a live server, so `cargo test`
needs the local database running.

Two things to remember when querying these columns: Postgres defines **no `=` operator** for
`point`/`polygon` - use the geometric `~=`, `@>`, `<@`, `&&`, or the GiST-indexed `<->`
distance - and either type can reach PostGIS with a `::geometry` cast if spatial work ever
wants it.
