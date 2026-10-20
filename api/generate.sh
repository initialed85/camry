#!/bin/bash

set -e

cd "$(dirname "$0")"

# print-schema writes to stdout instead of print_schema.file when --except-tables is used,
# hence the redirect. spatial_ref_sys is PostGIS bookkeeping and schema_migrations is
# diesel's own; neither belongs in the API.
diesel print-schema --except-tables 'spatial_ref_sys|schema_migrations' >src/schema.rs

# -I: diesel_ext only prints `use diesel::prelude::*;` when --add-table-name is set, so
#     name it here or the derives have nothing to resolve against.
# -M: teach diesel_ext's type map the columns it cannot name (pg point/polygon, interval).
#     Any new "is not recognized" line means a placeholder landed in models.rs, so fail.
if ! warnings=$(diesel_ext -s src/schema.rs -m \
	-I 'diesel::prelude::*' \
	-M 'Point crate::pg_types::PgPoint' \
	-M 'Polygon crate::pg_types::PgPolygon' \
	-M 'Interval crate::pg_types::PgDuration' 2>&1 >src/models.rs) || [ -n "$warnings" ]; then
	printf '%s\n' "$warnings" >&2
	exit 1
fi

# The tables are created by the Go API's auto-migrate, so --diff-schema has nothing to diff
# and every run leaves an empty migrations/<timestamp>_initial behind. Opt in once
# migrations are the source of truth for the schema.
if [ "${WITH_DIFF:-}" = "1" ]; then
	diesel migration generate initial --diff-schema=src/schema.rs --database-url 'postgresql://postgres:NoNVR!11@localhost/camry'
fi

cargo clippy --fix --allow-dirty
