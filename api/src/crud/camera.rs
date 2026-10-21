//! /cameras

use crate::models as db;
use crate::schema::camera;
use chrono::Utc;
use diesel::prelude::*;
use openapi::models as wire;

fn wire_of(row: &db::Camera) -> wire::Camera {
    wire::Camera {
        id: Some(row.id),
        created_at: Some(row.created_at),
        updated_at: Some(row.updated_at),
        deleted_at: row.deleted_at,
        name: Some(row.name.clone()),
        stream_url: Some(row.stream_url.clone()),
        low_res_stream_url: row.low_res_stream_url.clone(),
        last_seen: Some(row.last_seen),
        segment_producer_claimed_until: Some(row.segment_producer_claimed_until),
        stream_producer_claimed_until: Some(row.stream_producer_claimed_until),
        overlay_offset_ms: Some(row.overlay_offset_ms as i64),
        ..wire::Camera::new()
    }
}

crud_model! {
    table: camera,
    model: db::Camera,
    wire: wire::Camera,
    path: "/cameras",
    columns: [
        id: Uuid,
        created_at: Timestamptz,
        updated_at: Timestamptz,
        deleted_at: Timestamptz,
        name: Text,
        stream_url: Text,
        low_res_stream_url: Text,
        last_seen: Timestamptz,
        segment_producer_claimed_until: Timestamptz,
        stream_producer_claimed_until: Timestamptz,
        overlay_offset_ms: Int4,
    ],
    relations: [],
    to_wire: wire_of,
    insert: |body: &wire::Camera| (
        camera::name.eq(body.name.clone().unwrap_or_default()),
        camera::stream_url.eq(body.stream_url.clone().unwrap_or_default()),
        camera::low_res_stream_url.eq(body.low_res_stream_url.clone()),
        camera::last_seen.eq(body.last_seen.unwrap_or_else(Utc::now)),
        camera::segment_producer_claimed_until
            .eq(body.segment_producer_claimed_until.unwrap_or_else(Utc::now)),
        camera::stream_producer_claimed_until
            .eq(body.stream_producer_claimed_until.unwrap_or_else(Utc::now)),
        camera::overlay_offset_ms.eq(body.overlay_offset_ms.unwrap_or(0) as i32),
    ),
    patch: |body: &wire::Camera| (
        camera::deleted_at.eq(body.deleted_at),
        camera::name.eq(body.name.clone().unwrap_or_default()),
        camera::stream_url.eq(body.stream_url.clone().unwrap_or_default()),
        camera::low_res_stream_url.eq(body.low_res_stream_url.clone()),
        camera::last_seen.eq(body.last_seen.unwrap_or_else(Utc::now)),
        camera::segment_producer_claimed_until
            .eq(body.segment_producer_claimed_until.unwrap_or_else(Utc::now)),
        camera::stream_producer_claimed_until
            .eq(body.stream_producer_claimed_until.unwrap_or_else(Utc::now)),
        camera::overlay_offset_ms.eq(body.overlay_offset_ms.unwrap_or(0) as i32),
    ),
}
