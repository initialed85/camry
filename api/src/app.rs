//! App assembly: the pool, the CRUD routes and the custom routes that
//! djangolang never emits into the OpenAPI schema.

use std::env;

use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use serde::Deserialize;
use serde_json::json;

use crate::crud::{self, PgPool};

pub const COOKIE_NAME: &str = "camry_token";

pub fn pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let max_connections = env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(10);

    PgPool::builder()
        .max_size(max_connections)
        .build(ConnectionManager::new(database_url))
        .expect("failed to build pool")
}

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/custom/login", post(login))
        .route("/custom/auth", get(auth))
        .merge(crud::router())
        .with_state(pool)
}

#[derive(Deserialize)]
struct Login {
    password: String,
}

/// Exchanges the fixed password for the fixed token. The token also goes out as
/// a cookie because <video> and <img> cannot send an Authorization header.
async fn login(headers: HeaderMap, body: Json<Login>) -> Response {
    let password = env::var("PASSWORD").unwrap_or_default();
    let token = env::var("TOKEN").unwrap_or_default();

    if password.is_empty() || token.is_empty() {
        return error(
            StatusCode::SERVICE_UNAVAILABLE,
            "auth is not configured (PASSWORD and TOKEN must both be set)",
        );
    }

    if body.password.trim() != password {
        return error(StatusCode::UNAUTHORIZED, "invalid password");
    }

    let secure = is_secure(&headers);

    (
        StatusCode::OK,
        [(
            axum::http::header::SET_COOKIE,
            format!("{COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax; Secure={secure}"),
        )],
        Json(json!({
            "status": 200,
            "success": true,
            "objects": [{ "token": token }],
            "count": 1,
            "total_count": 1,
            "limit": 0,
            "offset": 0,
        })),
    )
        .into_response()
}

/// Traefik's forwardAuth target.
async fn auth(headers: HeaderMap) -> Response {
    let password = env::var("PASSWORD").unwrap_or_default();
    let token = env::var("TOKEN").unwrap_or_default();

    if password.is_empty() || token.is_empty() {
        return ok(json!({ "auth_required": false, "authenticated": true }));
    }

    match token_of(&headers) {
        None => error(
            StatusCode::UNAUTHORIZED,
            &format!("missing token (Authorization: Bearer or the {COOKIE_NAME} cookie)"),
        ),
        Some(presented) if presented != token => error(StatusCode::UNAUTHORIZED, "invalid token"),
        Some(_) => ok(json!({ "auth_required": true, "authenticated": true })),
    }
}

/// The token arrives as a bearer header or, for media requests, as a cookie.
fn token_of(headers: &HeaderMap) -> Option<String> {
    if let Some(authorization) = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    {
        let mut parts = authorization.split_whitespace();

        if parts
            .next()
            .is_some_and(|scheme| scheme.eq_ignore_ascii_case("bearer"))
        {
            return parts.next().map(str::to_string);
        }
    }

    headers
        .get(axum::http::header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .map(str::trim)
        .find_map(|cookie| cookie.strip_prefix(&format!("{COOKIE_NAME}=")))
        .map(str::to_string)
}

fn is_secure(headers: &HeaderMap) -> bool {
    headers
        .get("x-forwarded-proto")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("https"))
}

fn ok(object: serde_json::Value) -> Response {
    (
        StatusCode::OK,
        Json(json!({
            "status": 200,
            "success": true,
            "objects": [object],
            "count": 1,
            "total_count": 1,
            "limit": 0,
            "offset": 0,
        })),
    )
        .into_response()
}

fn error(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(json!({
            "status": status.as_u16(),
            "success": false,
            "error": [message],
            "count": 0,
            "total_count": 0,
            "limit": 0,
            "offset": 0,
        })),
    )
        .into_response()
}
