//! Rust <-> PostgreSQL native geometric/interval types.
//!
//! `centroid` is `pg_catalog.point`, `bounding_box` is `pg_catalog.polygon` and
//! `video.duration` is `interval`. None of them have a Rust counterpart in Diesel, and
//! PostGIS is not involved, so the wire codecs live here. The JSON shapes mirror the
//! published OpenAPI contract (Go's `pgtype.Vec2` and `time.Duration`) so the existing
//! frontend and generated clients keep working.

use crate::schema::sql_types::{Point, Polygon};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Interval;
use serde::{Deserialize, Serialize};
use std::io::Write;

const NANOS_PER_MICRO: i64 = 1_000;
const MICROS_PER_DAY: i64 = 24 * 60 * 60 * 1_000_000;
const MICROS_PER_MONTH: i64 = 30 * MICROS_PER_DAY;
const POINT_BYTES: usize = 16;

/// A `point`: two `float8`, big-endian on the wire.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, AsExpression, FromSqlRow, Serialize, Deserialize,
)]
#[diesel(sql_type = Point)]
#[serde(rename_all = "PascalCase")]
pub struct PgPoint {
    pub x: f64,
    pub y: f64,
}

impl PgPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// `(x, y)`, the text form PostgreSQL uses for a point.
    pub fn parse(raw: &str) -> Result<Self, String> {
        let body = raw.trim().trim_matches(|c| c == '(' || c == ')');
        let mut parts = body.split(',').map(str::trim);

        let x = parts
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("invalid point {raw}"))?;
        let y = parts
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("invalid point {raw}"))?;

        Ok(Self { x, y })
    }
}

fn read_f64(bytes: &[u8], offset: usize) -> deserialize::Result<f64> {
    let end = offset + 8;
    let slice = bytes
        .get(offset..end)
        .ok_or_else(|| format!("expected at least {} bytes, got {}", end, bytes.len()))?;
    Ok(f64::from_be_bytes(slice.try_into().unwrap()))
}

impl FromSql<Point, Pg> for PgPoint {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();
        if bytes.len() != POINT_BYTES {
            return Err(format!(
                "expected {} bytes for point, got {}",
                POINT_BYTES,
                bytes.len()
            )
            .into());
        }
        Ok(Self {
            x: read_f64(bytes, 0)?,
            y: read_f64(bytes, 8)?,
        })
    }
}

impl ToSql<Point, Pg> for PgPoint {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(&self.x.to_be_bytes())?;
        out.write_all(&self.y.to_be_bytes())?;
        Ok(IsNull::No)
    }
}

/// A `polygon`: `int32` vertex count then that many `point`s. PostgreSQL's in-memory
/// area field is not part of the wire format.
#[derive(Debug, Clone, Default, PartialEq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Polygon)]
#[serde(transparent)]
pub struct PgPolygon(pub Vec<PgPoint>);

impl PgPolygon {
    pub fn new(points: Vec<PgPoint>) -> Self {
        Self(points)
    }

    /// `((x, y), (x, y))`, the text form PostgreSQL uses for a polygon.
    pub fn parse(raw: &str) -> Result<Self, String> {
        let mut points = Vec::new();
        let mut depth = 0usize;
        let mut start = None;

        for (index, character) in raw.char_indices() {
            match character {
                '(' => {
                    depth += 1;

                    if depth == 2 {
                        start = Some(index + 1);
                    }
                }
                ')' => {
                    if depth == 2 {
                        points.push(PgPoint::parse(&raw[start.take().unwrap_or(index)..index])?);
                    }

                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
        }

        Ok(Self(points))
    }
}

impl FromSql<Polygon, Pg> for PgPolygon {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();
        if bytes.len() < 4 {
            return Err(
                format!("expected at least 4 bytes for polygon, got {}", bytes.len()).into(),
            );
        }
        let count = i32::from_be_bytes(bytes[..4].try_into()?) as usize;
        if bytes.len() != 4 + count * POINT_BYTES {
            return Err(format!(
                "polygon claims {} vertices, got {} bytes",
                count,
                bytes.len() - 4
            )
            .into());
        }
        let mut points = Vec::with_capacity(count);
        for i in 0..count {
            points.push(PgPoint {
                x: read_f64(bytes, 4 + i * POINT_BYTES)?,
                y: read_f64(bytes, 12 + i * POINT_BYTES)?,
            });
        }
        Ok(Self(points))
    }
}

impl ToSql<Polygon, Pg> for PgPolygon {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(&(self.0.len() as i32).to_be_bytes())?;
        for point in &self.0 {
            out.write_all(&point.x.to_be_bytes())?;
            out.write_all(&point.y.to_be_bytes())?;
        }
        Ok(IsNull::No)
    }
}

/// An `interval` as nanoseconds, which is what the Go API and every generated client
/// already exchange. PostgreSQL has no fixed-length month or day, so they are flattened
/// with the same 30/24 hour assumptions `pgtype::Interval` uses; `video.duration` never
/// carries them. Sub-microsecond precision is dropped, as it is on the Go side.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    AsExpression,
    FromSqlRow,
    Serialize,
    Deserialize,
)]
#[diesel(sql_type = Interval)]
#[serde(transparent)]
pub struct PgDuration(pub i64);

impl PgDuration {
    pub fn nanoseconds(&self) -> i64 {
        self.0
    }
}

impl FromSql<Interval, Pg> for PgDuration {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        let bytes = value.as_bytes();
        if bytes.len() != 16 {
            return Err(format!("expected 16 bytes for interval, got {}", bytes.len()).into());
        }
        let micros = i64::from_be_bytes(bytes[..8].try_into()?);
        let days = i32::from_be_bytes(bytes[8..12].try_into()?) as i64;
        let months = i32::from_be_bytes(bytes[12..16].try_into()?) as i64;
        Ok(Self(
            (months * MICROS_PER_MONTH + days * MICROS_PER_DAY + micros) * NANOS_PER_MICRO,
        ))
    }
}

impl ToSql<Interval, Pg> for PgDuration {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(&(self.0 / NANOS_PER_MICRO).to_be_bytes())?;
        out.write_all(&0i32.to_be_bytes())?;
        out.write_all(&0i32.to_be_bytes())?;
        Ok(IsNull::No)
    }
}
