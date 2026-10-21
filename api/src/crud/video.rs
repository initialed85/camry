//! /videos

use crate::crud::{self, camera, json_of, object_of};
use crate::models as db;
use crate::pg_types::PgDuration;
use crate::schema::{camera as camera_schema, video};
use chrono::Utc;
use diesel::prelude::*;
use openapi::models as wire;
use uuid::Uuid;

fn wire_of(row: &db::Video) -> wire::Video {
    wire::Video {
        id: Some(row.id),
        created_at: Some(row.created_at),
        updated_at: Some(row.updated_at),
        deleted_at: row.deleted_at,
        file_name: Some(row.file_name.clone()),
        started_at: Some(row.started_at),
        ended_at: row.ended_at,
        duration: row.duration.map(|PgDuration(nanos)| nanos),
        file_size: row.file_size,
        thumbnail_name: row.thumbnail_name.clone(),
        status: row.status.clone(),
        object_detector_claimed_until: Some(row.object_detector_claimed_until),
        object_tracker_claimed_until: Some(row.object_tracker_claimed_until),
        audio_detector_claimed_until: Some(row.audio_detector_claimed_until),
        audio_detected_at: row.audio_detected_at,
        camera_id: Some(row.camera_id),
        is_low_res: Some(row.is_low_res),
        overlay_offset_ms: row.overlay_offset_ms.map(|ms| ms as i64),
        detection_summary: Some(object_of(&row.detection_summary)),
        track_summary: Some(object_of(&row.track_summary)),
        audio_profile: Some(object_of(&row.audio_profile)),
        audio_summary: Some(object_of(&row.audio_summary)),
        ..wire::Video::new()
    }
}

/// djangolang fills `<fk>_id_object` at the default depth.
fn load_relations(
    conn: &mut PgConnection,
    objects: &mut [wire::Video],
) -> crud::Result<()> {
    let ids: Vec<Uuid> = objects.iter().filter_map(|object| object.camera_id).collect();

    if ids.is_empty() {
        return Ok(());
    }

    let rows: Vec<db::Camera> = camera_schema::table
        .filter(camera_schema::id.eq_any(&ids))
        .load(conn)
        .map_err(crud::Error::db)?;
    let children = rows
        .iter()
        .map(|row| (row.id, camera::to_wire(row)))
        .collect();

    crud::attach(objects, children, |object| object.camera_id, |object, joined| {
        object.camera_id_object = Some(joined)
    });

    Ok(())
}

crud_model! {
    table: video,
    model: db::Video,
    wire: wire::Video,
    path: "/videos",
    columns: [
        id: Uuid,
        created_at: Timestamptz,
        updated_at: Timestamptz,
        deleted_at: Timestamptz,
        file_name: Text,
        started_at: Timestamptz,
        ended_at: Timestamptz,
        duration: Interval,
        file_size: Float8,
        thumbnail_name: Text,
        status: Text,
        object_detector_claimed_until: Timestamptz,
        object_tracker_claimed_until: Timestamptz,
        audio_detector_claimed_until: Timestamptz,
        audio_detected_at: Timestamptz,
        camera_id: Uuid,
        detection_summary: Jsonb,
        is_low_res: Bool,
        track_summary: Jsonb,
        overlay_offset_ms: Int4,
        audio_profile: Jsonb,
        audio_summary: Jsonb,
    ],
    relations: ["camera"],
    to_wire: wire_of,
    insert: |body: &wire::Video| (
        video::file_name.eq(body.file_name.clone().unwrap_or_default()),
        video::started_at.eq(body.started_at.unwrap_or_else(Utc::now)),
        video::ended_at.eq(body.ended_at),
        video::duration.eq(body.duration.map(PgDuration)),
        video::file_size.eq(body.file_size),
        video::thumbnail_name.eq(body.thumbnail_name.clone()),
        video::status.eq(body.status.clone()),
        video::object_detector_claimed_until
            .eq(body.object_detector_claimed_until.unwrap_or_else(Utc::now)),
        video::object_tracker_claimed_until
            .eq(body.object_tracker_claimed_until.unwrap_or_else(Utc::now)),
        video::audio_detector_claimed_until
            .eq(body.audio_detector_claimed_until.unwrap_or_else(Utc::now)),
        video::audio_detected_at.eq(body.audio_detected_at),
        video::camera_id.eq(body.camera_id.unwrap_or_else(Uuid::nil)),
        video::is_low_res.eq(body.is_low_res.unwrap_or(false)),
        video::overlay_offset_ms.eq(body.overlay_offset_ms.map(|ms| ms as i32)),
        video::detection_summary.eq(json_of(&body.detection_summary)),
        video::track_summary.eq(json_of(&body.track_summary)),
        video::audio_profile.eq(json_of(&body.audio_profile)),
        video::audio_summary.eq(json_of(&body.audio_summary)),
    ),
    patch: |body: &wire::Video| (
        video::deleted_at.eq(body.deleted_at),
        video::file_name.eq(body.file_name.clone().unwrap_or_default()),
        video::started_at.eq(body.started_at.unwrap_or_else(Utc::now)),
        video::ended_at.eq(body.ended_at),
        video::duration.eq(body.duration.map(PgDuration)),
        video::file_size.eq(body.file_size),
        video::thumbnail_name.eq(body.thumbnail_name.clone()),
        video::status.eq(body.status.clone()),
        video::object_detector_claimed_until
            .eq(body.object_detector_claimed_until.unwrap_or_else(Utc::now)),
        video::object_tracker_claimed_until
            .eq(body.object_tracker_claimed_until.unwrap_or_else(Utc::now)),
        video::audio_detector_claimed_until
            .eq(body.audio_detector_claimed_until.unwrap_or_else(Utc::now)),
        video::audio_detected_at.eq(body.audio_detected_at),
        video::camera_id.eq(body.camera_id.unwrap_or_else(Uuid::nil)),
        video::is_low_res.eq(body.is_low_res.unwrap_or(false)),
        video::overlay_offset_ms.eq(body.overlay_offset_ms.map(|ms| ms as i32)),
        video::detection_summary.eq(json_of(&body.detection_summary)),
        video::track_summary.eq(json_of(&body.track_summary)),
        video::audio_profile.eq(json_of(&body.audio_profile)),
        video::audio_summary.eq(json_of(&body.audio_summary)),
    ),
    load: load_relations,
}
