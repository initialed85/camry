//! The Djangolang query grammar, on Diesel.
//!
//! Wire types come from the published OpenAPI spec (`openapi::models`), the
//! database types come from `schema.rs` + `models.rs`. Everything in between -
//! the `__op` grammar, ordering, pagination, the response envelope and the
//! error body - is written once here and applied per model by `crud_model!`.

use std::collections::HashMap;

use axum::body::Bytes;
use axum::extract::{FromRequest, Request as AxumRequest};
use axum::response::{IntoResponse, Response};
use axum::Json;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;


#[macro_use]
mod helpers;

mod query;
pub mod audio_event;
pub mod camera;
pub mod detection;
pub mod track;
pub mod video;

pub fn router() -> axum::Router<PgPool> {
    camera::routes()
        .merge(video::routes())
        .merge(detection::routes())
        .merge(track::routes())
        .merge(audio_event::routes())
}

pub const DEFAULT_LIMIT: i64 = 50;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type Result<T> = std::result::Result<T, Error>;

pub use axum::extract::{Path, State};
pub use query::{InvalidQuery, RawQuery};

/// Body extractor that fails the way djangolang does.
///
/// axum answers a bad JSON body with 422 and its own text; the reference API
/// reads the whole body, tries to decode it into the handler's request type (an
/// array of models for create, a single model for update), and answers 400 with
/// the raw body in the message.
pub struct Body<T>(pub T);

impl<T, S> FromRequest<S> for Body<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: AxumRequest, state: &S) -> std::result::Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|err| err.to_string());

        Self::read(bytes)
    }
}

impl<T: serde::de::DeserializeOwned> Body<T> {
    /// The reference checks the params a route accepts before it reads the
    /// body, so a detail handler that wants that order takes the bytes itself.
    #[allow(clippy::result_large_err)]
    pub fn read(body: std::result::Result<Bytes, String>) -> std::result::Result<Self, Response> {
        let bytes = body.map_err(|err| body_error("", err))?;
        let raw = String::from_utf8_lossy(&bytes).to_string();

        match serde_json::from_str::<T>(&raw) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(body_error(&raw, err.to_string())),
        }
    }
}

/// The request body, read but not interpreted.
pub struct RawBody(pub std::result::Result<Bytes, String>);

impl<S> FromRequest<S> for RawBody
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request(req: AxumRequest, state: &S) -> std::result::Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|err| err.to_string());

        Ok(Self(bytes))
    }
}

fn body_error(raw: &str, detail: String) -> Response {
    Error::bad_body(raw, detail).into_response()
}

use crate::pg_types::{PgPoint, PgPolygon};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    NotIn,
    Contains,
    NotContains,
    IsNull,
    IsNotNull,
    IsTrue,
    IsFalse,
    Like,
    NotLike,
    ILike,
    NotILike,
}

impl Op {
    fn parse(raw: &str) -> Option<Op> {
        Some(match raw {
            "eq" => Op::Eq,
            "ne" => Op::Ne,
            "gt" => Op::Gt,
            "gte" => Op::Gte,
            "lt" => Op::Lt,
            "lte" => Op::Lte,
            "in" => Op::In,
            "notin" => Op::NotIn,
            "contains" => Op::Contains,
            "notcontains" => Op::NotContains,
            "isnull" => Op::IsNull,
            "isnotnull" => Op::IsNotNull,
            "istrue" => Op::IsTrue,
            "isfalse" => Op::IsFalse,
            "like" => Op::Like,
            "notlike" => Op::NotLike,
            "ilike" => Op::ILike,
            "notilike" => Op::NotILike,
            _ => return None,
        })
    }

    pub fn suffix(self) -> &'static str {
        match self {
            Op::Eq => "eq",
            Op::Ne => "ne",
            Op::Gt => "gt",
            Op::Gte => "gte",
            Op::Lt => "lt",
            Op::Lte => "lte",
            Op::In => "in",
            Op::NotIn => "notin",
            Op::Contains => "contains",
            Op::NotContains => "notcontains",
            Op::IsNull => "isnull",
            Op::IsNotNull => "isnotnull",
            Op::IsTrue => "istrue",
            Op::IsFalse => "isfalse",
            Op::Like => "like",
            Op::NotLike => "notlike",
            Op::ILike => "ilike",
            Op::NotILike => "notilike",
        }
    }

    /// The operator djangolang writes into SQL for a geometry comparison.
    pub fn comparison(self) -> &'static str {
        match self {
            Op::Lt => "<",
            Op::Lte => "<=",
            Op::Gt => ">",
            Op::Gte => ">=",
            _other => "=",
        }
    }
}

/// A query value the way djangolang sees it: it tries `json.Unmarshal`, then
/// the same value quoted as a JSON string, and only then gives up on the raw
/// text. An instant is text, not a time, because nothing here parses one before
/// a column's parse function does. The kind that comes out decides what that
/// function accepts, because every one of them switches on the Go type.
#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    Text(String),
    Num(f64),
    Bool(bool),
    Null,
    Seq(Vec<Param>),
    Map(Vec<(String, Param)>),
}

impl Param {
    pub fn decode(raw: &str) -> Param {
        if let Ok(value) = serde_json::from_str::<Value>(raw) {
            return Param::from_value(value);
        }

        match serde_json::from_str::<Value>(&format!("\"{raw}\"")) {
            Ok(Value::String(text)) => Param::Text(text),
            _ => Param::Text(raw.to_string()),
        }
    }

    fn from_value(value: Value) -> Param {
        match value {
            Value::String(text) => Param::Text(text),
            Value::Number(number) => number.as_f64().map(Param::Num).unwrap_or(Param::Text(number.to_string())),
            Value::Bool(flag) => Param::Bool(flag),
            Value::Null => Param::Null,
            Value::Array(items) => Param::Seq(items.into_iter().map(Param::from_value).collect()),
            Value::Object(object) => {
                // serde_json without `preserve_order` is a BTreeMap, which
                // matches the sorted keys Go prints for a map.
                Param::Map(object.into_iter().map(|(k, v)| (k, Param::from_value(v))).collect())
            }
        }
    }

    /// Go's `%s`, which is what djangolang echoes back in a bad params error.
    pub fn display(&self) -> String {
        match self {
            Param::Text(text) => text.clone(),
            Param::Num(number) => format!("%!s(float64={})", go_float(*number)),
            Param::Bool(flag) => format!("%!s(bool={flag})"),
            Param::Null => "%!s(<nil>)".to_string(),
            Param::Seq(items) => {
                format!("[{}]", join(items.iter().map(|item| item.display()), " "))
            }
            Param::Map(object) => {
                format!("map[{}]", join(object.iter().map(|(k, v)| format!("{k}:{}", v.display())), " "))
            }
        }
    }

    /// Go's `%#v`, which is what a parse function quotes in its error.
    pub fn repr(&self) -> String {
        match self {
            Param::Text(text) => format!("{text:?}"),
            Param::Num(number) => go_float(*number),
            Param::Bool(flag) => flag.to_string(),
            Param::Null => "<nil>".to_string(),
            Param::Seq(items) => format!("[]interface {{ {{{}}} }}", join(items.iter().map(|item| item.repr()), ", ")),
            Param::Map(object) => format!(
                "map[string]interface {{ {{{}}} }}",
                join(object.iter().map(|(k, v)| format!("{k:?}: {}", v.repr())), ", ")
            ),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Param::Text(_) => "string",
            Param::Num(_) => "float64",
            Param::Bool(_) => "bool",
            Param::Null => "<nil>",
            Param::Seq(_) => "[]interface {}",
            Param::Map(_) => "map[string]interface {}",
        }
    }

    /// Go's `%v`, which is how `__in` re-textualizes a value it cannot split as
    /// a string. `%s` refuses the numeric kinds where `%v` prints them.
    pub fn value_text(&self) -> String {
        match self {
            Param::Num(number) => go_float(*number),
            Param::Bool(flag) => flag.to_string(),
            Param::Null => "<nil>".to_string(),
            Param::Seq(items) => format!("[{}]", join(items.iter().map(Param::value_text), " ")),
            Param::Map(object) => format!(
                "map[{}]",
                join(object.iter().map(|(k, v)| format!("{k}:{}", v.value_text())), " ")
            ),
            _other => self.display(),
        }
    }

    /// djangolang turns a comma separated filter back into text, and the pieces
    /// are then parsed as strings, never as their decoded kind.
    pub fn list(&self) -> Vec<Param> {
        self.value_text()
            .split(',')
            .map(|part| Param::Text(part.to_string()))
            .collect()
    }

    pub fn json(&self) -> Value {
        match self {
            Param::Text(text) => serde_json::from_str(text)
                .unwrap_or_else(|_| Value::String(text.clone())),
            Param::Num(number) => Value::from(*number),
            Param::Bool(flag) => Value::Bool(*flag),
            Param::Null => Value::Null,
            Param::Seq(items) => Value::Array(items.iter().map(Param::json).collect()),
            Param::Map(object) => {
                Value::Object(object.iter().map(|(k, v)| (k.clone(), v.json())).collect())
            }
        }
    }
}

fn join<I: Iterator<Item = String>>(values: I, separator: &str) -> String {
    values.collect::<Vec<String>>().join(separator)
}

/// Go's `%v` on a float64: the shortest digits that round-trip, written
/// exponentially once the decimal exponent leaves `-4 <= exp < 6` (measured
/// against the reference: `150000`, `1.234567e+06`, `1e-05`, `0.0001`).
fn go_float(value: f64) -> String {
    if !value.is_finite() {
        return match value {
            f64::INFINITY => "+Inf".into(),
            f64::NEG_INFINITY => "-Inf".into(),
            _ => "NaN".into(),
        };
    }

    if value == 0.0 {
        return if value.is_sign_negative() { "-0".into() } else { "0".into() };
    }

    let scientific = format!("{value:e}");
    let (mantissa, exponent) = match scientific.split_once('e') {
        Some(parts) => parts,
        None => return format!("{value}"),
    };
    let exp = exponent.parse::<i32>().unwrap_or(0);

    if !(-4..6).contains(&exp) {
        let width = if exp.abs() < 10 { 2 } else { format!("{exp}").len() };

        return format!(
            "{mantissa}e{}{:0width$}",
            if exp < 0 { '-' } else { '+' },
            exp.abs(),
            width = width
        );
    }

    format!("{value}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Asc,
    Desc,
}

impl Direction {
    pub fn suffix(self) -> &'static str {
        match self {
            Direction::Asc => "asc",
            Direction::Desc => "desc",
        }
    }
}

#[derive(Debug)]
pub struct Filter {
    pub key: String,
    pub column: String,
    pub op: Op,
    pub value: Param,
}

#[derive(Debug)]
pub struct Request {
    pub filters: Vec<Filter>,
    pub order: Vec<(String, Direction)>,
    pub load: Vec<(String, Param)>,
    pub unknown: Vec<String>,
    pub insane: Vec<String>,
    pub limit: i64,
    pub offset: i64,
    pub depth: i32,
}

impl Request {
    /// The grammar is djangolang's: `col__op=value`, `col__asc`/`col__desc` as
    /// bare flags, `col__in=a,b,c` as one comma separated value, and anything
    /// else is an error rather than a silently ignored filter.
    ///
    /// Keys are sorted before they are read, so a multi column ORDER BY comes
    /// out the same on every request. djangolang ranges a Go map and shuffles
    /// its own order params, which makes `?a__asc&b__asc` non-deterministic.
    pub fn parse(raw: &[(String, String)]) -> Self {
        let mut req = Request {
            filters: Vec::new(),
            order: Vec::new(),
            load: Vec::new(),
            unknown: Vec::new(),
            insane: Vec::new(),
            limit: DEFAULT_LIMIT,
            offset: 0,
            depth: 1,
        };

        for (key, raw_value) in raw {
            let value = Param::decode(raw_value);

            // limit / offset / depth only apply when they decoded to a number,
            // because djangolang type asserts `.(float64)`.
            match key.as_str() {
                "limit" => {
                    req.limit = value.as_number().unwrap_or(DEFAULT_LIMIT);
                    continue;
                }
                "offset" => {
                    req.offset = value.as_number().unwrap_or(0);
                    continue;
                }
                "depth" => {
                    req.depth = value.as_number().unwrap_or(1) as i32;
                    continue;
                }
                _ => {}
            }

            // A key with two separators addresses a nested table, which
            // djangolang skips on a top level query.
            if key.matches("__").count() == 2 {
                continue;
            }

            let (column, suffix) = match key.rsplit_once("__") {
                Some((column, suffix)) => (column, Some(suffix)),
                None => (key.as_str(), None),
            };

            match suffix {
                Some("asc") | Some("desc") => {
                    let direction = if suffix == Some("asc") {
                        Direction::Asc
                    } else {
                        Direction::Desc
                    };

                    // One direction for the whole query, like djangolang: a
                    // second, different one is an insane order param.
                    if req.order.iter().any(|(_, set)| *set != direction) {
                        req.insane.push(key.clone());
                    } else {
                        req.order.push((column.to_string(), direction));
                    }
                }
                Some("load") => req.load.push((column.to_string(), value)),
                Some(suffix) => match Op::parse(suffix) {
                    Some(op) => req.filters.push(Filter {
                        key: key.to_string(),
                        column: column.to_string(),
                        op,
                        value,
                    }),
                    None => req.unknown.push(format!("{key}={}", value.display())),
                },
                None => req.unknown.push(format!("{key}={}", value.display())),
            }
        }

        req
    }
}

impl Param {
    /// djangolang reads limit / offset / depth as a `float64` and truncates it.
    fn as_number(&self) -> Option<i64> {
        match self {
            Param::Num(number) => Some(*number as i64),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Error {
    message: String,
    /// djangolang wraps some errors with a separator, and the client gets one
    /// array entry per wrap level. A joined list is a single entry.
    joined: bool,
    /// A detail route refuses to look at params it does not know, so the
    /// failure happens before the handler and the answer is a 400.
    status: u16,
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl Error {
    fn new(message: String) -> Self {
        Self {
            message,
            joined: false,
            status: 500,
        }
    }

    fn joined(message: String) -> Self {
        Self {
            message,
            joined: true,
            status: 500,
        }
    }

    pub fn params(raw: &str) -> Self {
        Self::joined(format!("unrecognized params {raw}"))
    }

    pub fn unsupported(key: &str, value: &str) -> Self {
        Self::joined(format!("unrecognized params {key}={value}"))
    }

    pub fn unparseable(raw: &str) -> Self {
        Self::new(format!("unparseable params {raw}"))
    }

    /// djangolang's per value message: `key=decoded (parse func error)`.
    pub fn unparseable_param(key: &str, value: &Param, cause: String) -> Self {
        Self::new(format!(
            "unparseable params {key}={} ({cause})",
            value.display()
        ))
    }

    pub fn insane_order(keys: &[String]) -> Self {
        Self::joined(format!(
            "insane order params (e.g. conflicting asc / desc) {}",
            keys.join(", ")
        ))
    }

    pub fn pool(err: diesel::r2d2::PoolError) -> Self {
        Self::new(format!("pool: {err}"))
    }

    pub fn db(err: diesel::result::Error) -> Self {
        Self::new(err.to_string())
    }

    /// A failure that belongs to Postgres rather than to a parse function, e.g.
    /// a geometry comparison, whose operand the reference leaves untyped.
    pub fn select(table: &str, cause: &str) -> Self {
        Self::select_with(table, &format!("ERROR: {cause}"))
    }

    /// Some values fail in the driver before Postgres sees them, so the tail is
    /// the driver's own text rather than an `ERROR:`.
    pub fn select_driver(table: &str, cause: &str) -> Self {
        Self::select_with(table, cause)
    }

    fn select_with(table: &str, tail: &str) -> Self {
        // The generated Go select function is named after the model, and the
        // list route calls the plural form.
        let mut name = String::new();

        for word in format!("{table}s").split('_') {
            let mut letters = word.chars();

            if let Some(first) = letters.next() {
                name.extend(first.to_uppercase());
                name.push_str(letters.as_str());
            }
        }

        Self::new(format!(
            "failed to call Select{name}; failed to call GetRowEstimate during Select; {tail}"
        ))
    }

    /// The reference reads the body inside the handler, so a bad one is a 400
    /// with the raw text echoed back.
    pub fn bad_body(raw: &str, detail: String) -> Self {
        Self {
            message: format!("failed to handle reqBody {raw} as JSON; {detail}"),
            joined: false,
            status: 400,
        }
    }

    /// The detail route validates its param names before it parses any value.
    pub fn unrecognized_query(keys: &[String]) -> Self {
        Self {
            message: format!("unrecognized query params: {}", keys.join(", ")),
            joined: true,
            status: 400,
        }
    }

    /// djangolang lets the driver's error text reach the client.
    pub fn missing() -> Self {
        Self::new("sql: no rows in result set".to_string())
    }
}

/// One array entry per wrap level: semicolons always divide, and a message
/// that survives undivided is a parse failure, whose cause follows the comma.
fn split_chain(message: &str) -> Vec<&str> {
    let parts = message.split("; ").collect::<Vec<&str>>();

    if parts.len() > 1 {
        return parts;
    }

    match message.rfind(", ") {
        Some(at) => vec![&message[..at], &message[at + 2..]],
        None => parts,
    }
}

/// Djangolang's detail routes read `depth` and nothing else, and reject the
/// rest by name before they look at a value or a body.
pub fn check_detail_params(raw: &[(String, String)]) -> Option<Error> {
    let unknown = raw
        .iter()
        .filter(|(key, _)| key != "depth")
        .map(|(key, _)| key.clone())
        .collect::<Vec<String>>();

    if unknown.is_empty() {
        None
    } else {
        Some(Error::unrecognized_query(&unknown))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use axum::http::StatusCode;

        let message = if self.status == 400 {
            self.message
        } else {
            format!("failed to invoke handler: {}", self.message)
        };
        // djangolang wraps errors with either separator, and the client gets
        // one array entry per wrap level.
        let error = if self.joined {
            vec![message]
        } else {
            split_chain(&message).into_iter().map(str::to_string).collect()
        };
        let status = match self.status {
            400 => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (
            status,
            Json(Failure {
                status: self.status,
                success: false,
                error,
                ..Default::default()
            }),
        )
            .into_response()
    }
}

/// The envelope a handler returns when it did not get as far as a query, with
/// the same field order as the reference.
#[derive(Serialize, Default)]
struct Failure {
    status: u16,
    success: bool,
    error: Vec<String>,
    count: i64,
    total_count: i64,
    limit: i64,
    offset: i64,
}

#[derive(Serialize)]
pub struct Envelope<T: Serialize> {
    pub status: u16,
    pub success: bool,
    // An empty result set has a nil slice behind it in Go, so the key is
    // missing rather than `[]`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub objects: Vec<T>,
    pub count: i64,
    pub total_count: i64,
    pub limit: i64,
    pub offset: i64,
}

impl<T: Serialize> Envelope<T> {
    pub fn response(
        objects: Vec<T>,
        total_count: i64,
        limit: i64,
        offset: i64,
        status: axum::http::StatusCode,
    ) -> Response {
        Self::with_status(objects, total_count, limit, offset, status, status)
    }

    /// djangolang answers a create with HTTP 201 but keeps the envelope at 200.
    pub fn with_status(
        objects: Vec<T>,
        total_count: i64,
        limit: i64,
        offset: i64,
        status: axum::http::StatusCode,
        body_status: axum::http::StatusCode,
    ) -> Response {
        let count = objects.len() as i64;

        (
            status,
            Json(Envelope {
                status: body_status.as_u16(),
                success: true,
                objects,
                count,
                total_count,
                limit,
                offset,
            }),
        )
            .into_response()
    }
}

pub fn error_response(err: Error) -> Response {
    err.into_response()
}

pub fn status_response(status: axum::http::StatusCode) -> Response {
    use axum::response::IntoResponse;

    status.into_response()
}

/// Attach batch loaded children to their parents by id, which is what
/// djangolang does per relation, only without the N+1.
pub fn attach<W, C>(
    objects: &mut [W],
    children: Vec<(Uuid, C)>,
    parent_id: impl Fn(&W) -> Option<Uuid>,
    mut set: impl FnMut(&mut W, C),
) where
    C: Clone,
{
        let by_id: HashMap<Uuid, C> = children.into_iter().collect();

        for object in objects {
            if let Some(child) = parent_id(object).and_then(|id| by_id.get(&id)) {
                set(object, child.clone());
            }
        }
}

/// Wire <-> database conversions for the shapes djangolang invented JSON for:
/// `{"X": .., "Y": ..}` points, arrays of them for polygons, and `{}` for an
/// absent jsonb column.
pub fn point_of(point: PgPoint) -> openapi::models::ArrayOfVec2Inner {
    openapi::models::ArrayOfVec2Inner {
        x: Some(point.x),
        y: Some(point.y),
    }
}

pub fn point_into(point: &openapi::models::ArrayOfVec2Inner) -> PgPoint {
    PgPoint::new(point.x.unwrap_or_default(), point.y.unwrap_or_default())
}

pub fn polygon_of(polygon: &PgPolygon) -> Vec<openapi::models::ArrayOfVec2Inner> {
    polygon.0.iter().map(|point| point_of(*point)).collect()
}

pub fn polygon_into(
    points: &Option<Vec<openapi::models::ArrayOfVec2Inner>>,
) -> PgPolygon {
    PgPolygon::new(
        points
            .as_ref()
            .map(|points| points.iter().map(point_into).collect())
            .unwrap_or_default(),
    )
}

pub fn json_of(object: &Option<openapi::types::Object>) -> Value {
    object
        .as_ref()
        .map(|openapi::types::Object(value)| value.clone())
        .unwrap_or_else(|| serde_json::json!({}))
}

pub fn object_of(value: &Value) -> openapi::types::Object {
    openapi::types::Object(value.clone())
}

pub mod value {
    use crate::pg_types::{PgDuration, PgPoint, PgPolygon};
    use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
    use serde_json::Value;
    use uuid::Uuid;

    use super::Param;

    /// A parse failure, in the shape the reference prints for a param that
    /// exists but holds an unusable value.
    fn unparseable(key: &str, param: &Param, detail: &str) -> String {
        format!("unparseable params {key}={} ({detail})", param.display())
    }

    /// A list parses its pieces, so the message names the first piece inside a
    /// failure blamed on the whole value.
    fn identified_with(key: &str, param: &Param, piece: &Param, name: &str) -> String {
        unparseable(
            key,
            param,
            &format!(
                "{} ({}) could not be identified for {name}",
                piece.repr(),
                piece.type_name()
            ),
        )
    }

    /// A value whose type the parse function has no branch for.
    fn identified(key: &str, param: &Param, name: &str) -> String {
        unparseable(
            key,
            param,
            &format!(
                "{} ({}) could not be identified for {name}",
                param.repr(),
                param.type_name()
            ),
        )
    }

    /// A value of a type the parse function knows, whose parse still failed.
    /// djangolang joins the cause with a comma, which is where the error list
    /// the response carries picks up.
    fn parsed_with(key: &str, param: &Param, func: &str, name: &str, cause: &str) -> String {
        unparseable(
            key,
            param,
            &format!(
                "{} ({}) could not be parsed with {func} for {name}, {cause}",
                param.repr(),
                param.type_name()
            ),
        )
    }

    /// ParseString knows only strings, so a decoded number or bool is refused.
    pub fn text(key: &str, param: &Param) -> Result<String, String> {
        match param {
            Param::Text(text) => Ok(text.clone()),
            _other => Err(identified(key, param, "ParseString")),
        }
    }

    pub fn texts(key: &str, param: &Param) -> Result<Vec<String>, String> {
        list(key, param, text)
    }

    /// A valueless `__in` matches nothing, which is the reference's own behaviour
    /// once it has rejected the empty pieces it split out of the value.
    pub fn empty_texts() -> Vec<String> {
        Vec::new()
    }

    /// A text filter is matched by value, so nothing may break out of the
    /// statement it is embedded in.
    pub fn safe_text(key: &str, param: &Param) -> Result<String, String> {
        match text(key, param) {
            Ok(text) if !text.contains(['\'', '\\']) => Ok(text),
            _other => Err(unparseable(key, param, &param.display())),
        }
    }

    /// A pattern of a value, with the wildcards every like operator implies.
    pub fn pattern(key: &str, param: &Param) -> Result<String, String> {
        Ok(format!("%{}%", safe_text(key, param)?))
    }

    /// `__in` turns the value back into text with `%v` and splits it at every
    /// comma, so the pieces are strings and never their decoded kind: `[1, 2]`
    /// arrives as the single string `[1 2]`, and an empty piece is a value of
    /// its own, which `strconv` rejects for everything but a string.
    fn items(param: &Param) -> Vec<Param> {
        param.list()
    }

    /// The array parses every piece with the scalar parse, and the reference
    /// then blames the whole value, so the piece's own display is exchanged for
    /// the param's in the message.
    fn list<T>(
        key: &str,
        param: &Param,
        parse: impl Fn(&str, &Param) -> Result<T, String>,
    ) -> Result<Vec<T>, String> {
        items(param)
            .iter()
            .map(|item| parse(key, item).map_err(|cause| redress(key, param, item, &cause)))
            .collect::<Result<Vec<T>, String>>()
    }

    fn redress(key: &str, param: &Param, piece: &Param, cause: &str) -> String {
        // The reference names the whole param, then quotes the piece inside the
        // parentheses the parse function's own message occupies.
        let whole = format!("unparseable params {key}=");
        let mut detail = match cause.strip_prefix(&format!("{whole}{} (", piece.display())) {
            Some(detail) => detail,
            None => cause.strip_prefix(&whole).unwrap_or(cause),
        };

        // A bare parse error already came wrapped; a piece's message did not.
        detail = detail.strip_prefix('(').unwrap_or(detail);
        detail = detail.strip_suffix(')').unwrap_or(detail);

        unparseable(key, param, detail)
    }

    /// ParseString of every item, for a list of strings.
    pub fn stringies(key: &str, param: &Param) -> Result<Vec<String>, String> {
        texts(key, param)
    }

    /// ParseBool knows strings and bools. A string failure surfaces strconv's
    /// own message, unadorned, because that is all ParseBool returns.
    pub fn boolean(key: &str, param: &Param) -> Result<bool, String> {
        match param {
            Param::Bool(value) => Ok(*value),
            Param::Text(text) => strconv_bool(text)
                .map_err(|cause| unparseable(key, param, &cause)),
            Param::Num(number) => Err(unparseable(
                key,
                param,
                &strconv_error("ParseBool", &go_float_text(*number)),
            )),
            _other => Err(identified(key, param, "ParseBool")),
        }
    }

    /// A JSON array of bools is cast, anything else is split into pieces and
    /// parsed, so `1` and `TRUE` are as good as `true`.
    pub fn booleans(key: &str, param: &Param) -> Result<Vec<bool>, String> {
        list(key, param, boolean)
    }

    /// ParseInt truncates a float and parses a string in the 64-bit range, so
    /// anything it returns fits in the database's integer.
    pub fn i64(key: &str, param: &Param) -> Result<i64, String> {
        match param {
            Param::Num(number) => Ok(*number as i64),
            Param::Text(text) => go_int(text),
            _other => Err(identified(key, param, "ParseInt")),
        }
    }

    pub fn i32(key: &str, param: &Param) -> Result<i32, String> {
        i64(key, param).map(|value| value as i32)
    }

    pub fn i32s(key: &str, param: &Param) -> Result<Vec<i32>, String> {
        list(key, param, i64)
            .map(|values| values.into_iter().map(|value| value as i32).collect())
    }

    pub fn i64s(key: &str, param: &Param) -> Result<Vec<i64>, String> {
        list(key, param, i64)
    }

    pub fn f64(key: &str, param: &Param) -> Result<f64, String> {
        match param {
            Param::Num(number) => Ok(*number),
            Param::Text(text) => go_float(text),
            _other => Err(identified(key, param, "ParseFloat")),
        }
    }

    /// Floats are the one family whose list never parses: outside a JSON array
    /// the pieces are strings, and `ParseFloat` has no string branch, so the
    /// reference fails on the first item whatever the list held.
    pub fn f64s(key: &str, param: &Param) -> Result<Vec<f64>, String> {
        let first = items(param).into_iter().next().unwrap_or(Param::Text(String::new()));

        Err(identified_with(key, param, &first, "ParseFloat"))
    }

    /// ParseTime knows strings and times, and every filter value is a string.
    pub fn timestamp(key: &str, param: &Param) -> Result<DateTime<Utc>, String> {
        match param {
            Param::Text(text) => go_time(text)
                .map_err(|cause| parsed_with(key, param, "time.Parse", "ParseTime", &cause)),
            _other => Err(unparseable(
                key,
                param,
                &format!(
                    "{} ({}) could not be identified ParseTime",
                    param.repr(),
                    param.type_name()
                ),
            )),
        }
    }

    pub fn timestamps(key: &str, param: &Param) -> Result<Vec<DateTime<Utc>>, String> {
        list(key, param, timestamp)
    }

    /// ParseDuration keeps only the microseconds of an interval, so months and
    /// days in the value make no difference to the comparison.
    pub fn duration(key: &str, param: &Param) -> Result<PgDuration, String> {
        match param {
            Param::Num(number) => Ok(PgDuration(*number as i64)),
            Param::Text(text) => scan_interval(text)
                .map(PgDuration)
                .map_err(|cause| parsed_with(key, param, "pgtype.Interval.Scan", "ParseDuration", &cause)),
            _other => Err(unparseable(
                key,
                param,
                &format!(
                    "{} ({}) could not be identified ParseDuration",
                    param.repr(),
                    param.type_name()
                ),
            )),
        }
    }

    pub fn durations(key: &str, param: &Param) -> Result<Vec<PgDuration>, String> {
        list(key, param, duration)
    }

    pub fn uuid(key: &str, param: &Param) -> Result<Uuid, String> {
        match param {
            Param::Text(text) => go_uuid(text)
                .map_err(|cause| parsed_with(key, param, "uuid.Parse", "ParseUUID", &cause)),
            _other => Err(identified(key, param, "ParseUUID")),
        }
    }

    pub fn uuids(key: &str, param: &Param) -> Result<Vec<Uuid>, String> {
        list(key, param, uuid)
    }

    /// ParseJSON returns its value untouched, so text reaches the database as
    /// text, where a jsonb column rejects it.
    pub fn json(key: &str, param: &Param) -> Result<Value, String> {
        match param {
            Param::Text(text) => serde_json::from_str::<Value>(text)
                .map_err(|_| unparseable(key, param, "invalid input syntax for type json")),
            _other => Ok(param.json()),
        }
    }

    pub fn jsons(key: &str, param: &Param) -> Result<Vec<Value>, String> {
        list(key, param, json)
    }

    /// A point filter is a `{"X": .., "Y": ..}` object, optionally wrapped in a
    /// `{"P": ..}` object, or that object as text.
    pub fn point(key: &str, param: &Param) -> Result<PgPoint, String> {
        match param {
            Param::Map(object) => point_from_map(key, param, object),
            Param::Text(text) => match serde_json::from_str::<Value>(text) {
                Ok(value) => match Param::from_value(value) {
                    Param::Map(object) => point_from_map(key, param, &object),
                    Param::Text(inner) => point_text(key, param, &inner),
                    _other => Err(as_point(key, param, "bad type for 'P' key")),
                },
                Err(_) => match point_text_cause(text) {
                    Ok(point) => Ok(point),
                    Err(cause) => Err(parsed_with(
                        key,
                        param,
                        "pgtype.Point.UnmarshalJSON",
                        "ParsePoint",
                        &cause,
                    )),
                },
            },
            _other => Err(identified(key, param, "ParsePoint")),
        }
    }

    /// pgx reads a point as text by dropping one byte from each end and cutting
    /// the remainder at the first comma, so `(1, 2)` fails on ` 2` while
    /// ` 1,2 ` parses. A malformed JSON value still gets this try, because
    /// UnmarshalJSON's own error is not what the reference reports.
    /// A point list is cut at every comma first, which no point survives.
    pub fn points(key: &str, param: &Param) -> Result<Vec<PgPoint>, String> {
        list(key, param, point)
    }

    /// A polygon list, cut the same way.
    pub fn polygons(key: &str, param: &Param) -> Result<Vec<PgPolygon>, String> {
        list(key, param, polygon)
    }

    /// How Go prints a float64, for the driver's own error text.
    pub fn float_text(value: f64) -> String {
        crate::crud::go_float(value)
    }

    fn point_text(key: &str, param: &Param, text: &str) -> Result<PgPoint, String> {
        point_text_cause(text).map_err(|cause| {
            parsed_with(
                key,
                param,
                "pgtype.Point.UnmarshalJSON",
                "ParsePoint",
                &cause,
            )
        })
    }

    fn point_text_cause(text: &str) -> Result<PgPoint, String> {
        let bytes = text.as_bytes();

        if bytes.len() < 5 {
            return Err(format!("invalid length for point: {}", bytes.len()));
        }

        let inner = String::from_utf8_lossy(&bytes[1..bytes.len() - 1]).into_owned();
        let (sx, sy, found) = match inner.split_once(',') {
            Some((sx, sy)) => (sx, sy, true),
            None => ("", "", false),
        };

        if !found {
            return Err("invalid format for point".into());
        }

        Ok(PgPoint::new(go_float(sx)?, go_float(sy)?))
    }

    fn point_from_map(key: &str, param: &Param, object: &[(String, Param)]) -> Result<PgPoint, String> {
        let mut object = object;

        // A `{"P": {"X": .., "Y": ..}}` wrapper is unwrapped first.
        if let Some((_, inner)) = object.iter().find(|(key, _)| key == "P") {
            match inner {
                Param::Map(inner) => object = inner,
                _other => return Err(as_point(key, param, "bad type for 'P' key")),
            }
        }

        let coordinate = |name: &str| -> Result<f64, String> {
            match object.iter().find(|(key, _)| key == name) {
                None => Err(as_point(key, param, &format!("missing '{name}' key"))),
                Some((_, Param::Num(number))) => Ok(*number),
                Some(_) => Err(as_point(key, param, &format!("bad type for '{name}' key"))),
            }
        };

        let x = coordinate("X")?;
        let y = coordinate("Y")?;

        Ok(PgPoint { x, y })
    }

    fn as_point(key: &str, param: &Param, cause: &str) -> String {
        unparseable(
            key,
            param,
            &format!(
                "{} ({}) could not be parsed as pgtype.Point for ParsePoint, {cause}",
                param.repr(),
                param.type_name()
            ),
        )
    }

    /// A polygon filter is `{"P": [{"X": .., "Y": ..}, ..]}` or that array on
    /// its own. There is no string case, so text never parses.
    pub fn polygon(key: &str, param: &Param) -> Result<PgPolygon, String> {
        let points: Vec<Param> = match param {
            Param::Map(object) => match object.iter().find(|(key, _)| key == "P") {
                None => return Err(as_point(key, param, "missing 'P' key")),
                Some((_, Param::Seq(points))) => points.clone(),
                _other => return Err(as_point(key, param, "bad type for 'P' key")),
            },
            Param::Seq(points) => points.clone(),
            _other => return Err(identified(key, param, "ParsePolygon")),
        };

        let mut polygon = Vec::new();

        for item in points {
            let point = point(key, &item).map_err(|cause| {
                unparseable(
                    key,
                    param,
                    &format!(
                        "{} ({}) could not be parsed by item for ParsePolygon, {cause}",
                        param.repr(),
                        param.type_name()
                    ),
                )
            })?;

            polygon.push(point);
        }

        Ok(PgPolygon(polygon))
    }

    /// Go's strconv.ParseInt, including its range errors.
    fn go_int(text: &str) -> Result<i64, String> {
        let (body, bits) = match text.strip_prefix("uint64=") {
            Some(body) => (body, 64),
            None => (text, 64),
        };

        match body.parse::<i64>() {
            Ok(value) if bits == 64 || value >= 0 => Ok(value),
            Ok(_) => Err(format!("value \"{text}\" out of range for type int64")),
            Err(_) if body.parse::<u64>().is_ok() => {
                Err(format!("value \"{text}\" out of range for type int64"))
            }
            Err(_) => Err(strconv_error("ParseInt", text)),
        }
    }

    fn go_float(text: &str) -> Result<f64, String> {
        match text.parse::<f64>() {
            Ok(value) if value.is_finite() => Ok(value),
            Ok(_) => Err(format!("value \"{text}\" out of range")),
            Err(_) => Err(strconv_error("ParseFloat", text)),
        }
    }

    fn go_uuid(text: &str) -> Result<Uuid, String> {
        match text.parse::<Uuid>() {
            Ok(value) => Ok(value),
            Err(_) => Err(match text.len() {
                32 | 36 | 38 | 45 => "invalid UUID format".to_string(),
                length => format!("invalid UUID length: {length}"),
            }),
        }
    }

    fn strconv_error(func: &str, text: &str) -> String {
        format!("strconv.{func}: parsing \"{text}\": invalid syntax")
    }

    /// Go's strconv.ParseBool, which also takes "t", "T", "TRUE", "true", "1".
    fn strconv_bool(text: &str) -> Result<bool, String> {
        match text {
            "1" | "t" | "T" | "true" | "TRUE" | "True" => Ok(true),
            "0" | "f" | "F" | "false" | "FALSE" | "False" => Ok(false),
            _other => Err(strconv_error("ParseBool", text)),
        }
    }

    /// Go's shortest representation of a float, which is what a stringified
    /// value carries into the next parse.
    fn go_float_text(value: f64) -> String {
        if value == value.trunc() && value.abs() < 1e15 {
            format!("{}", value as i64)
        } else {
            format!("{value}")
        }
    }

    /// Go's time.Parse, which is strict about digits, separators, and offsets.
    /// Only the layouts ParseTime tries are understood.
    fn go_time(text: &str) -> Result<DateTime<Utc>, String> {
        let mut last = String::new();

        for layout in [
            "2006-01-02T15:04:05.999999999Z07:00",
            "2006-01-02T15:04:05Z07:00",
            "2006-01-02T15:04:05Z",
        ] {
            match parse_with_layout(text, layout) {
                Ok(at) => return Ok(at),
                Err(cause) => last = cause.message(text, layout),
            }
        }

        Err(last)
    }

    /// Where a value disagreed with a layout.
    enum Cause {
        /// The text from `value_at` did not match the token.
        Structure { token: &'static str, value_at: usize },
        /// A parsed field was outside the range Go allows.
        Range(String),
    }

    impl Cause {
        fn message(&self, text: &str, layout: &str) -> String {
            match self {
                Cause::Structure { token, value_at } => format!(
                    "parsing time \"{text}\" as \"{layout}\": cannot parse \"{}\" as \"{token}\"",
                    &text[*value_at..]
                ),
                Cause::Range(value) => {
                    format!("parsing time \"{text}\": value \"{value}\" out of range")
                }
            }
        }
    }

    fn parse_with_layout(text: &str, layout: &str) -> Result<DateTime<Utc>, Cause> {
        let mut at = 0;
        let mut fields = [0; 6];
        let mut fraction = String::new();
        let offset: Option<i32>;

        // The layouts agree up to the seconds, then differ in the fraction and
        // the zone, so both are read from the layout that is in play.
        let mut layout_at = 0;

        for (index, token, digits) in [
            (0, "2006", 4),
            (1, "01", 2),
            (2, "02", 2),
            (3, "15", 2),
            (4, "04", 2),
            (5, "05", 2),
        ] {
            // Skip the literal that separates this field from the last.
            if index > 0 {
                let separator = layout.get(layout_at..layout_at + 1).unwrap_or("");
                let expected = match index {
                    1 | 2 => "-",
                    3 => "T",
                    _ => ":",
                };

                if separator != expected {
                    layout_at += 1;
                }

                if !text[at..].starts_with(expected) {
                    return Err(Cause::Structure {
                        token: expected,
                        value_at: at,
                    });
                }

                at += 1;
                layout_at += 2;
            }

            let value = digits_of(text, at, digits).ok_or(Cause::Structure { token, value_at: at })?;

            fields[index] = value.0;
            at = value.1;
            layout_at += token.len();
        }

        let rest = text.get(at..).unwrap_or("");

        if layout.contains(".999999999") {
            let tail = rest.strip_prefix('.').unwrap_or(rest);
            let digits = tail.chars().take_while(|c| c.is_ascii_digit()).count();

            if digits > 9 {
                return Err(Cause::Structure {
                    token: "Z07:00",
                    value_at: at + 1 + digits,
                });
            }

            fraction = tail[..digits].to_string();
            at += digits + usize::from(rest.starts_with('.'));
        } else if rest.starts_with('.') {
            return Err(Cause::Structure {
                token: "Z07:00",
                value_at: at,
            });
        }

        let zone = text.get(at..).unwrap_or("");
        let tail = if layout.ends_with("Z07:00") {
            "Z07:00"
        } else if layout.ends_with("Z07") {
            "Z07"
        } else {
            "Z"
        };

        if tail == "Z" {
            if zone != "Z" && zone != "z" {
                return Err(Cause::Structure {
                    token: "Z",
                    value_at: at,
                });
            }

            offset = Some(0);
        } else if zone.is_empty() {
            return Err(Cause::Structure {
                token: tail,
                value_at: at,
            });
        } else if zone.starts_with("Z") || zone.starts_with("z") {
            if zone.len() != 1 {
                return Err(Cause::Structure {
                    token: tail,
                    value_at: at + 1,
                });
            }

            offset = Some(0);
        } else {
            let sign = match zone.as_bytes().first() {
                Some(b'+') => 1,
                Some(b'-') => -1,
                _other => {
                    return Err(Cause::Structure { token: tail, value_at: at })
                }
            };

            let body = &zone[1..];
            let (hours, minutes) = match tail.chars().any(|c| c == ':') {
                true => match body.split_once(':') {
                    Some((hours, minutes)) if (1..=2).contains(&hours.len()) && minutes.len() == 2 => {
                        (hours.parse::<i32>().unwrap_or_default(), minutes.parse::<i32>().unwrap_or_default())
                    }
                    _other => {
                        return Err(Cause::Structure {
                            token: tail,
                            value_at: at,
                        })
                    }
                },
                false => match body.len() {
                    4 => (
                        body[..2].parse::<i32>().unwrap_or_default(),
                        body[2..].parse::<i32>().unwrap_or_default(),
                    ),
                    _other => {
                        return Err(Cause::Structure {
                            token: tail,
                            value_at: at,
                        })
                    }
                },
            };

            if minutes > 59 {
                return Err(Cause::Range(format!("{minutes:02}")));
            }

            offset = Some(sign * (hours * 3600 + minutes * 60));
        }

        let (year, month, day, hour, minute, second) =
            (fields[0], fields[1], fields[2], fields[3], fields[4], fields[5]);

        for (value, limit) in [(month, 12), (hour, 24), (minute, 59), (second, 60)] {
            if value > limit || (value == 0 && limit < 24 && false) {
                return Err(Cause::Range(format!("{value:02}")));
            }
        }

        if month < 1 {
            return Err(Cause::Range(format!("{month:02}")));
        }

        let nanos = match fraction.is_empty() {
            true => 0,
            false => format!("{fraction:0<9}")
                .parse::<u32>()
                .unwrap_or_default(),
        };

        let date = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .ok_or(Cause::Range(format!("{day:02}")))?;
        let time = NaiveTime::from_hms_nano_opt(hour as u32, minute as u32, second as u32, nanos)
            .ok_or(Cause::Range(format!("{second:02}")))?;

        let naive = date.and_time(time);
        let at = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc) - chrono::Duration::seconds(offset.unwrap_or(0) as i64);

        Ok(at)
    }

    /// Exactly `digits` digits at `at`, as Go's getnum requires them.
    fn digits_of(text: &str, at: usize, digits: usize) -> Option<(i32, usize)> {
        let rest = text.get(at..).unwrap_or("");
        let count = rest.chars().take_while(|c| c.is_ascii_digit()).count();

        if count < digits {
            return None;
        }

        let value = rest[..digits].parse::<i32>().ok()?;

        Some((value, at + digits))
    }

    /// pgtype.Interval's own text scan: pairs of scalar and unit, then a time
    /// in the last slot. Months and days are kept apart from the microseconds,
    /// which is why they never affect a comparison.
    fn scan_interval(text: &str) -> Result<i64, String> {
        let parts = text.split(' ').collect::<Vec<&str>>();
        let mut microseconds = 0;
        let mut index = 0;

        while index + 1 < parts.len() {
            let scalar = parts[index]
                .parse::<i64>()
                .map_err(|_| "bad interval format".to_string())?;

            match parts[index + 1] {
                "day" | "days" => microseconds += scalar * 86_400_000_000,
                // A month is a calendar month, which no comparison can use.
                "year" | "years" | "mon" | "mons" => {}
                unit => return Err(format!("bad interval format: {unit:?}")),
            }

            index += 2;
        }

        if parts.len() % 2 == 1 {
            microseconds += time_microseconds(parts[parts.len() - 1])?;
        }

        Ok(microseconds)
    }

    fn time_microseconds(text: &str) -> Result<i64, String> {
        let pieces = text.splitn(3, ':').collect::<Vec<&str>>();

        if pieces.len() != 3 {
            return Err("bad interval format".to_string());
        }

        let negative = pieces[0].starts_with('-');
        let body = pieces[0].trim_start_matches('-');

        let number = |text: &str, unit: &str| -> Result<i64, String> {
            text.parse::<i64>()
                .map_err(|_| format!("bad interval {unit} format: {text}"))
        };

        let hours = number(body, "hour")?;
        let minutes = number(pieces[1], "minute")?;
        let (sec, frac) = match pieces[2].split_once('.') {
            Some((whole, fraction)) => (whole, Some(fraction)),
            None => (pieces[2], None),
        };

        let seconds = number(sec, "second")?;
        let mut microseconds =
            hours * 3_600_000_000 + minutes * 60_000_000 + seconds * 1_000_000;

        if let Some(frac) = frac {
            let mut fraction = number(frac, "decimal")?;

            for _ in 0..(6usize.saturating_sub(frac.len())) {
                fraction *= 10;
            }

            microseconds += fraction;
        }

        Ok(if negative { -microseconds } else { microseconds })
    }

}