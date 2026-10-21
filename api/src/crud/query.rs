//! A query string read the way `net/url.ParseQuery` reads one.
//!
//! Axum's `Query` deserializes into a schema, so an unencoded `{` or `"` in a
//! value is a 400 before djangolang would ever have looked at it. The reference
//! service keeps everything as text, splits at `&` and the first `=`, and turns
//! `+` and `%XX` back into bytes, and only a broken escape is an error.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub struct RawQuery(pub Vec<(String, String)>);

pub struct InvalidQuery(pub String);

impl IntoResponse for InvalidQuery {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": 400,
                "success": false,
                "error": [format!("unparseable query params: {}", self.0)],
            })),
        )
            .into_response()
    }
}

impl<S> FromRequestParts<S> for RawQuery
where
    S: Send + Sync,
{
    type Rejection = InvalidQuery;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parse_query(parts.uri.query().unwrap_or_default()).map(RawQuery)
    }
}

fn parse_query(raw: &str) -> Result<Vec<(String, String)>, InvalidQuery> {
    let mut pairs = Vec::new();

    for part in raw.split('&') {
        if part.is_empty() {
            continue;
        }

        let (key, value) = match part.split_once('=') {
            Some((key, value)) => (key, value),
            None => (part, ""),
        };
        let key = unescape(key)?;
        let value = unescape(value)?;

        if key.is_empty() {
            return Err(InvalidQuery(format!(
                "invalid url query string: unable to find key for kv pair `{}`",
                part
            )));
        }

        pairs.push((key, value));
    }

    Ok(pairs)
}

/// Go's `unescape` with `plusIsSpace` set: `+` is a space, `%XX` is a byte, and
/// a `%` that isn't a valid escape is an error over the whole string.
fn unescape(raw: &str) -> Result<String, InvalidQuery> {
    let bytes = raw.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' => {
                if i + 2 >= bytes.len() {
                    return Err(InvalidQuery(format!(
                        "invalid escape sequence `{}` in URL path",
                        &raw[i..]
                    )));
                }

                let hex = &bytes[i + 1..i + 3];
                let high = (hex[0] as char).to_digit(16);
                let low = (hex[1] as char).to_digit(16);

                match (high, low) {
                    (Some(high), Some(low)) => {
                        out.push((high * 16 + low) as u8);
                        i += 3;
                    }
                    _ => {
                        return Err(InvalidQuery(format!(
                            "invalid escape sequence `%{}` in URL path",
                            String::from_utf8_lossy(hex)
                        )))
                    }
                }
            }
            other => {
                out.push(other);
                i += 1;
            }
        }
    }

    Ok(String::from_utf8_lossy(&out).into_owned())
}
