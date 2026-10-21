//! The macros behind `crud_model!`: one arm per column type, so a filter
//! string becomes a typed Diesel call instead of a runtime interpreter.
//!
//! Loaded with `#[macro_use]` so the model modules inherit them textually;
//! `macro_export` would put them at the crate root, where an absolute path to
//! one is rejected inside another macro's expansion.

/// `col @> $1` and friends: `infix_operator!` generates a `pub(crate)`
/// constructor, so a column name glued to an operator is written as raw SQL
/// with an explicit bind instead. The column name comes from `stringify!` at
/// the call site, which keeps the SQL string 'static.
macro_rules! predicate {
    ($sql:expr, $returns:ty, $bind:ty, $value:expr) => {
        diesel::dsl::sql::<$returns>($sql).bind::<$bind, _>($value)
    };
}


/// Compare, sort and paginate arms shared by every scalar column.
macro_rules! scalar_filter {
    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $one:ident, $many:ident) => {{
        let name = stringify!($column);
        let key = format!("{name}__{}", $crate::crud::Op::suffix($op));

        match $op {
            $crate::crud::Op::Eq => {
                Ok($q.filter($col.eq($crate::crud::value::$one(&key, &$value)?)))
            }
            $crate::crud::Op::Ne => {
                Ok($q.filter($col.ne($crate::crud::value::$one(&key, &$value)?)))
            }
            $crate::crud::Op::Gt => {
                Ok($q.filter($col.gt($crate::crud::value::$one(&key, &$value)?)))
            }
            $crate::crud::Op::Gte => {
                Ok($q.filter($col.ge($crate::crud::value::$one(&key, &$value)?)))
            }
            $crate::crud::Op::Lt => {
                Ok($q.filter($col.lt($crate::crud::value::$one(&key, &$value)?)))
            }
            $crate::crud::Op::Lte => {
                Ok($q.filter($col.le($crate::crud::value::$one(&key, &$value)?)))
            }
            $crate::crud::Op::In => {
                Ok($q.filter($col.eq_any($crate::crud::value::$many(&key, &$value)?)))
            }
            $crate::crud::Op::NotIn => {
                // djangolang binds more than one ne_all operand into a scalar
                // comparison, and Postgres drops the whole condition.
                let mut values = $crate::crud::value::$many(&key, &$value)?;

                if values.len() == 1 {
                    Ok($q.filter($col.ne(values.swap_remove(0))))
                } else {
                    Ok($q.filter($col.is_not_null()))
                }
            }
            $crate::crud::Op::IsNull => Ok($q.filter($col.is_null())),
            $crate::crud::Op::IsNotNull => Ok($q.filter($col.is_not_null())),
            _other => Err($crate::crud::Error::unsupported(&key, &$value.display())),
        }
    }};
}

macro_rules! filter_arm {
    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Text) => {{
        let name = stringify!($column);
        let key = format!("{name}__{}", $crate::crud::Op::suffix($op));

        match $op {
            $crate::crud::Op::Like => {
                Ok($q.filter($col.like($crate::crud::value::pattern(&key, &$value)?)))
            }
            $crate::crud::Op::NotLike => Ok($q.filter(
                $col.not_like($crate::crud::value::pattern(&key, &$value)?),
            )),
            $crate::crud::Op::ILike => Ok($q.filter(
                $col.ilike($crate::crud::value::pattern(&key, &$value)?),
            )),
            $crate::crud::Op::NotILike => Ok($q.filter(
                $col.not_ilike($crate::crud::value::pattern(&key, &$value)?),
            )),
            _other => scalar_filter!($q, $col, $column, _other, $value, text, texts),
        }
    }};

    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Bool) => {{
        match $op {
            $crate::crud::Op::IsTrue => Ok($q.filter($col.eq(true))),
            $crate::crud::Op::IsFalse => Ok($q.filter($col.eq(false))),
            _other => scalar_filter!($q, $col, $column, _other, $value, boolean, booleans),
        }
    }};

    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Jsonb) => {{
        let name = stringify!($column);
        let key = format!("{name}__{}", $crate::crud::Op::suffix($op));

        match $op {
            $crate::crud::Op::Contains => {
                let value = $crate::crud::value::json(&key, &$value)?;

                Ok($q.filter(predicate!(
                    concat!(stringify!($column), " @> "),
                    diesel::sql_types::Bool,
                    diesel::sql_types::Jsonb,
                    value
                )))
            }
            $crate::crud::Op::NotContains => {
                let value = $crate::crud::value::json(&key, &$value)?;

                Ok($q.filter(predicate!(
                    concat!(stringify!($column), " NOT @> "),
                    diesel::sql_types::Bool,
                    diesel::sql_types::Jsonb,
                    value
                )))
            }
            _other => scalar_filter!($q, $col, $column, _other, $value, json, jsons),
        }
    }};

    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Point) => {{
        let name = stringify!($column);
        let key = format!("{name}__{}", $crate::crud::Op::suffix($op));

        match $op {
            $crate::crud::Op::IsNull => Ok($q.filter($col.is_null())),
            $crate::crud::Op::IsNotNull => Ok($q.filter($col.is_not_null())),
            // A geometry filter binds an untyped operand, so Postgres rejects the
            // comparison itself; the value still has to parse first.
            _other => {
                let point = match $op {
                    $crate::crud::Op::In | $crate::crud::Op::NotIn => {
                        $crate::crud::value::points(&key, &$value)?;

                        None
                    }
                    _other => Some($crate::crud::value::point(&key, &$value)?),
                };

                Err(match $op {
                    // pgx has no binary encode plan for a point it built from text.
                    $crate::crud::Op::Ne => $crate::crud::Error::select_driver(
                        stringify!($table),
                        &format!(
                            "failed to encode args[0]: unable to encode pgtype.Vec2{{X: {}, Y: {}}} into binary format for point (OID 600): cannot find encode plan",
                            $crate::crud::value::float_text(point.unwrap().x),
                            $crate::crud::value::float_text(point.unwrap().y)
                        ),
                    ),
                    // `@>` is asked of the point as jsonb, which it never is.
                    $crate::crud::Op::Contains => $crate::crud::Error::select(
                        stringify!($table),
                        "cannot cast type point to jsonb (SQLSTATE 42846)",
                    ),
                    _other => $crate::crud::Error::select(
                        stringify!($table),
                        &format!(
                            "operator does not exist: point {} unknown (SQLSTATE 42883)",
                            $op.comparison()
                        ),
                    ),
                })
            }
        }
    }};
    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Polygon) => {{
        let name = stringify!($column);
        let key = format!("{name}__{}", $crate::crud::Op::suffix($op));

        match $op {
            $crate::crud::Op::IsNull => Ok($q.filter($col.is_null())),
            $crate::crud::Op::IsNotNull => Ok($q.filter($col.is_not_null())),
            // A geometry filter binds an untyped operand, so Postgres rejects the
            // comparison itself; the value still has to parse first.
            _other => {
                match $op {
                    $crate::crud::Op::In | $crate::crud::Op::NotIn => {
                        $crate::crud::value::polygons(&key, &$value)?;
                    }
                    _other => {
                        $crate::crud::value::polygon(&key, &$value)?;
                    }
                }

                Err($crate::crud::Error::select(
                    stringify!($table),
                    "operator does not exist: polygon = unknown (SQLSTATE 42883)",
                ))
            }
        }
    }};
    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Timestamptz) => {
        scalar_filter!($q, $col, $column, $op, $value, timestamp, timestamps)
    };

    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Uuid) => {
        scalar_filter!($q, $col, $column, $op, $value, uuid, uuids)
    };

    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Int4) => {
        scalar_filter!($q, $col, $column, $op, $value, i32, i32s)
    };

    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Int8) => {
        scalar_filter!($q, $col, $column, $op, $value, i64, i64s)
    };

    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Float8) => {
        scalar_filter!($q, $col, $column, $op, $value, f64, f64s)
    };

    ($q:expr, $col:expr, $column:ident, $op:expr, $value:expr, $table:ident, Interval) => {
        scalar_filter!($q, $col, $column, $op, $value, duration, durations)
    };
}

macro_rules! order_arm {
    ($q:expr, $col:expr, $direction:expr) => {{
        if $direction == $crate::crud::Direction::Asc {
            $q.order($col.asc())
        } else {
            $q.order($col.desc())
        }
    }};
}

/// One REST resource out of one table: the five CRUD routes, the filter
/// grammar and the envelope.
///
/// `columns` is copied from `schema.rs` and carries the entire filter and
/// order surface, so a new column costs one line. `to_wire`, `insert`,
/// `patch` and `load` stay per model because only they know about the OpenAPI
/// shapes and the relationships.
/// Columns djangolang owns on insert: it ignores a client `id`, stamps both
/// timestamps with the server clock, and never soft-deletes on create.
macro_rules! server_insert {
    ($table:ident) => {
        (
            $crate::schema::$table::id.eq(uuid::Uuid::new_v4()),
            $crate::schema::$table::created_at.eq(chrono::Utc::now()),
            $crate::schema::$table::updated_at.eq(chrono::Utc::now()),
            $crate::schema::$table::deleted_at.eq(None::<chrono::DateTime<chrono::Utc>>),
        )
    };
}

/// ...and on update, where `created_at` is restamped too and `deleted_at` comes
/// from the body, because that is how a caller soft-deletes a row.
macro_rules! server_update {
    ($table:ident) => {
        (
            $crate::schema::$table::created_at.eq(chrono::Utc::now()),
            $crate::schema::$table::updated_at.eq(chrono::Utc::now()),
        )
    };
}

macro_rules! crud_model {
    (
        table: $table:ident,
        model: $model:ty,
        wire: $wire:ty,
        path: $path:literal,
        columns: [ $($column:ident : $kind:ident),* $(,)? ],
        relations: [ $($relation:literal),* $(,)? ],
        to_wire: $to_wire:expr,
        insert: $insert:expr,
        patch: $patch:expr,
        $(load: $load:expr,)?
    ) => {
        pub fn to_wire(row: &$model) -> $wire {
            ($to_wire)(row)
        }

        pub fn routes() -> axum::Router<$crate::crud::PgPool> {
            use axum::routing::get;

            axum::Router::new()
                .route($path, get(list).post(create))
                .route(
                    &format!("{}/{{id}}", $path),
                    get(retrieve).patch(update).delete(destroy),
                )
        }

        async fn list(
            state: $crate::crud::State<$crate::crud::PgPool>,
            query: $crate::crud::RawQuery,
        ) -> axum::response::Response {
            use $crate::crud::{Error, Filter, Request};

            let $crate::crud::State(pool) = state;
            let $crate::crud::RawQuery(raw) = query;
            let req = Request::parse(&raw);
            let limit = req.limit;
            let offset = req.offset;

            let outcome = tokio::task::spawn_blocking(move || -> $crate::crud::Result<(Vec<$wire>, i64)> {
                let mut unknown = req.unknown.clone();
                let mut conn = pool.get().map_err(Error::pool)?;

                // Djangolang hides soft-deleted rows unless the caller asks
                // about `deleted_at` itself.
                let asks_about_deletions = req
                    .filters
                    .iter()
                    .any(|filter| filter.column == "deleted_at");
                let mut query = $crate::schema::$table::table
                    .select((
                        <$crate::schema::$table::table as diesel::Table>::all_columns(),
                        diesel::dsl::sql::<diesel::sql_types::BigInt>("count(*) over()"),
                    ))
                    .into_boxed::<diesel::pg::Pg>();

                if !asks_about_deletions {
                    query = query.filter($crate::schema::$table::deleted_at.is_null());
                }

                for Filter { column, op, value, key } in req.filters {
                    query = match column.as_str() {
                        $(
                            name if name == stringify!($column) => filter_arm!(
                                query,
                                $crate::schema::$table::$column,
                                $column,
                                op,
                                value,
                                $table,
                                $kind
                            )?,
                        )*
                        _other => {
                            unknown.push(format!("{key}={}", value.display()));
                            query
                        }
                    };
                }

                // Djangolang declares `<relation>__load` params next to `depth`;
                // the loader always runs, so the params only have to be named.
                for (name, value) in req.load {
                    let known = false $(|| name == $relation)*;

                    if !known {
                        unknown.push(format!("{name}__load={}", value.display()));
                    }
                }

                for (column, direction) in req.order {
                    query = match column.as_str() {
                        $(
                            name if name == stringify!($column) => order_arm!(
                                query,
                                $crate::schema::$table::$column,
                                direction
                            ),
                        )*
                        other => {
                            unknown.push(format!("{}__{}", other, direction.suffix()));
                            query
                        }
                    };
                }

                if !unknown.is_empty() {
                    return Err(Error::params(&unknown.join(", ")));
                }

                let rows: Vec<($model, i64)> = query
                    .limit(limit)
                    .offset(offset)
                    .load(&mut conn)
                    .map_err(Error::db)?;
                let total = rows.first().map(|(_, total)| *total).unwrap_or(0);
                #[allow(unused_mut)]
                let mut objects: Vec<$wire> = rows.iter().map(|(row, _)| to_wire(row)).collect();

                $( ($load)(&mut conn, &mut objects)?; )?

                Ok((objects, total))
            })
            .await;

            use $crate::crud::Envelope;

            match outcome {
                Ok(Ok((objects, total_count))) => Envelope::response(
                    objects,
                    total_count,
                    limit,
                    offset,
                    axum::http::StatusCode::OK,
                ),
                Ok(Err(err)) => $crate::crud::error_response(err),
                Err(err) => $crate::crud::error_response(Error::from(format!("worker: {err}"))),
            }
        }

        async fn retrieve(
            state: $crate::crud::State<$crate::crud::PgPool>,
            path: $crate::crud::Path<uuid::Uuid>,
            query: $crate::crud::RawQuery,
        ) -> axum::response::Response {
            use $crate::crud::Error;

            let $crate::crud::State(pool) = state;
            let $crate::crud::Path(id) = path;
            let $crate::crud::RawQuery(raw) = query;

            if let Some(err) = $crate::crud::check_detail_params(&raw) {
                return axum::response::IntoResponse::into_response(err);
            }

            let outcome = tokio::task::spawn_blocking(move || -> $crate::crud::Result<Vec<$wire>> {
                let mut conn = pool.get().map_err(Error::pool)?;
                let row = $crate::schema::$table::table
                    .filter($crate::schema::$table::id.eq(id))
                    .filter($crate::schema::$table::deleted_at.is_null())
                    .first::<$model>(&mut conn)
                    .map_err(|_| Error::missing())?;

                #[allow(unused_mut)]
                let mut objects = vec![to_wire(&row)];

                $( ($load)(&mut conn, &mut objects)?; )?

                Ok(objects)
            })
            .await;

            respond(outcome, 0, 0, axum::http::StatusCode::OK, axum::http::StatusCode::OK)
        }

        async fn create(
            state: $crate::crud::State<$crate::crud::PgPool>,
            body: $crate::crud::Body<Vec<$wire>>,
        ) -> axum::response::Response {
            use $crate::crud::Error;

            let $crate::crud::State(pool) = state;
            let $crate::crud::Body(bodies) = body;

            let outcome = tokio::task::spawn_blocking(move || -> $crate::crud::Result<Vec<$wire>> {
                let mut conn = pool.get().map_err(Error::pool)?;
                #[allow(unused_mut)]
                let mut objects = Vec::with_capacity(bodies.len());

                for body in &bodies {
                    let row = diesel::insert_into($crate::schema::$table::table)
                        .values((
                            server_insert!($table),
                            ($insert)(body),
                        ))
                        .get_result::<$model>(&mut conn)
                        .map_err(Error::db)?;

                    objects.push(to_wire(&row));
                }

                $( ($load)(&mut conn, &mut objects)?; )?

                Ok(objects)
            })
            .await;

            respond(outcome, 0, 0, axum::http::StatusCode::CREATED, axum::http::StatusCode::OK)
        }

        async fn update(
            state: $crate::crud::State<$crate::crud::PgPool>,
            path: $crate::crud::Path<uuid::Uuid>,
            query: $crate::crud::RawQuery,
            body: $crate::crud::RawBody,
        ) -> axum::response::Response {
            use $crate::crud::Error;

            let $crate::crud::State(pool) = state;
            let $crate::crud::Path(id) = path;
            let $crate::crud::RawQuery(raw) = query;
            let $crate::crud::RawBody(body) = body;

            if let Some(err) = $crate::crud::check_detail_params(&raw) {
                return axum::response::IntoResponse::into_response(err);
            }

            let $crate::crud::Body(body) = match $crate::crud::Body::read(body) {
                Ok(body) => body,
                Err(err) => return err,
            };

            let outcome = tokio::task::spawn_blocking(move || -> $crate::crud::Result<Vec<$wire>> {
                let mut conn = pool.get().map_err(Error::pool)?;
                let row = diesel::update(
                    $crate::schema::$table::table
                        .filter($crate::schema::$table::id.eq(id))
                        .filter($crate::schema::$table::deleted_at.is_null()),
                )
                .set((
                    server_update!($table),
                    ($patch)(&body),
                ))
                .get_result::<$model>(&mut conn)
                .map_err(|_| Error::missing())?;

                #[allow(unused_mut)]
                let mut objects = vec![to_wire(&row)];

                $( ($load)(&mut conn, &mut objects)?; )?

                Ok(objects)
            })
            .await;

            respond(outcome, 0, 0, axum::http::StatusCode::OK, axum::http::StatusCode::OK)
        }

        async fn destroy(
            state: $crate::crud::State<$crate::crud::PgPool>,
            path: $crate::crud::Path<uuid::Uuid>,
            query: $crate::crud::RawQuery,
        ) -> axum::response::Response {
            use $crate::crud::Error;

            let $crate::crud::State(pool) = state;
            let $crate::crud::Path(id) = path;
            let $crate::crud::RawQuery(raw) = query;

            if let Some(err) = $crate::crud::check_detail_params(&raw) {
                return axum::response::IntoResponse::into_response(err);
            }

            let outcome = tokio::task::spawn_blocking(move || -> $crate::crud::Result<usize> {
                let mut conn = pool.get().map_err(Error::pool)?;

                diesel::delete(
                    $crate::schema::$table::table
                        .filter($crate::schema::$table::id.eq(id))
                        .filter($crate::schema::$table::deleted_at.is_null()),
                )
                .execute(&mut conn)
                .map_err(Error::db)
            })
            .await;

            match outcome {
                // djangolang deletes by primary key without checking the count.
                Ok(Ok(_)) => $crate::crud::status_response(axum::http::StatusCode::NO_CONTENT),
                Ok(Err(err)) => $crate::crud::error_response(err),
                Err(err) => $crate::crud::error_response(Error::from(format!("worker: {err}"))),
            }
        }

        fn respond(
            outcome: std::result::Result<
                std::result::Result<Vec<$wire>, $crate::crud::Error>,
                tokio::task::JoinError,
            >,
            limit: i64,
            offset: i64,
            status: axum::http::StatusCode,
            body_status: axum::http::StatusCode,
        ) -> axum::response::Response {
            use $crate::crud::{Envelope, Error};

            match outcome {
                Ok(Ok(objects)) => {
                    let total = objects.len() as i64;

                    Envelope::with_status(objects, total, limit, offset, status, body_status)
                }
                Ok(Err(err)) => $crate::crud::error_response(err),
                Err(err) => $crate::crud::error_response(Error::from(format!("worker: {err}"))),
            }
        }
    };
}
