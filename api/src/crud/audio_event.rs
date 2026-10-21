//! /audio-events

use crate::crud::{self, camera, json_of, object_of, video};
use crate::models as db;
use crate::schema::{audio_event as audio_event_schema, camera as camera_schema, video as video_schema};
use chrono::Utc;
use diesel::prelude::*;
use openapi::models as wire;
use uuid::Uuid;

fn wire_of(row: &db::AudioEvent) -> wire::AudioEvent {
    wire::AudioEvent {
        id: Some(row.id),
        created_at: Some(row.created_at),
        updated_at: Some(row.updated_at),
        deleted_at: row.deleted_at,
        camera_id: Some(row.camera_id),
        video_id: Some(row.video_id),
        seen_at: Some(row.seen_at),
        offset_ms: Some(row.offset_ms as i64),
        ended_at: Some(row.ended_at),
        duration_ms: Some(row.duration_ms as i64),
        kind: Some(row.kind.clone()),
        rms_dbfs: Some(row.rms_dbfs),
        peak_dbfs: Some(row.peak_dbfs),
        floor_dbfs: Some(row.floor_dbfs),
        prominence_db: Some(row.prominence_db),
        crest_db: Some(row.crest_db),
        score: Some(row.score),
        spectral: Some(object_of(&row.spectral)),
        ..wire::AudioEvent::new()
    }
}

fn load_relations(
    conn: &mut PgConnection,
    objects: &mut [wire::AudioEvent],
) -> crud::Result<()> {
    let camera_ids: Vec<Uuid> = objects.iter().filter_map(|object| object.camera_id).collect();

    if !camera_ids.is_empty() {
        let rows: Vec<db::Camera> = camera_schema::table
            .filter(camera_schema::id.eq_any(&camera_ids))
            .load(conn)
            .map_err(crud::Error::db)?;
        let children = rows
            .iter()
            .map(|row| (row.id, camera::to_wire(row)))
            .collect();

        crud::attach(objects, children, |object| object.camera_id, |object, joined| {
            object.camera_id_object = Some(joined)
        });
    }

    let video_ids: Vec<Uuid> = objects.iter().filter_map(|object| object.video_id).collect();

    if !video_ids.is_empty() {
        let rows: Vec<db::Video> = video_schema::table
            .filter(video_schema::id.eq_any(&video_ids))
            .load(conn)
            .map_err(crud::Error::db)?;
        let children = rows
            .iter()
            .map(|row| (row.id, video::to_wire(row)))
            .collect();

        crud::attach(objects, children, |object| object.video_id, |object, joined| {
            object.video_id_object = Some(joined)
        });
    }

    Ok(())
}

crud_model! {
    table: audio_event,
    model: db::AudioEvent,
    wire: wire::AudioEvent,
    path: "/audio-events",
    columns: [
        id: Uuid,
        created_at: Timestamptz,
        updated_at: Timestamptz,
        deleted_at: Timestamptz,
        camera_id: Uuid,
        video_id: Uuid,
        seen_at: Timestamptz,
        offset_ms: Int4,
        ended_at: Timestamptz,
        duration_ms: Int4,
        kind: Text,
        rms_dbfs: Float8,
        peak_dbfs: Float8,
        floor_dbfs: Float8,
        prominence_db: Float8,
        crest_db: Float8,
        score: Float8,
        spectral: Jsonb,
    ],
    relations: ["camera", "video"],
    to_wire: wire_of,
    insert: |body: &wire::AudioEvent| (
        audio_event_schema::camera_id.eq(body.camera_id.unwrap_or_else(Uuid::nil)),
        audio_event_schema::video_id.eq(body.video_id.unwrap_or_else(Uuid::nil)),
        audio_event_schema::seen_at.eq(body.seen_at.unwrap_or_else(Utc::now)),
        audio_event_schema::offset_ms.eq(body.offset_ms.unwrap_or(0) as i32),
        audio_event_schema::ended_at.eq(body.ended_at.unwrap_or_else(Utc::now)),
        audio_event_schema::duration_ms.eq(body.duration_ms.unwrap_or(0) as i32),
        audio_event_schema::kind.eq(body.kind.clone().unwrap_or_default()),
        audio_event_schema::rms_dbfs.eq(body.rms_dbfs.unwrap_or_default()),
        audio_event_schema::peak_dbfs.eq(body.peak_dbfs.unwrap_or_default()),
        audio_event_schema::floor_dbfs.eq(body.floor_dbfs.unwrap_or_default()),
        audio_event_schema::prominence_db.eq(body.prominence_db.unwrap_or_default()),
        audio_event_schema::crest_db.eq(body.crest_db.unwrap_or_default()),
        audio_event_schema::score.eq(body.score.unwrap_or_default()),
        audio_event_schema::spectral.eq(json_of(&body.spectral)),
    ),
    patch: |body: &wire::AudioEvent| (
        audio_event_schema::deleted_at.eq(body.deleted_at),
        audio_event_schema::camera_id.eq(body.camera_id.unwrap_or_else(Uuid::nil)),
        audio_event_schema::video_id.eq(body.video_id.unwrap_or_else(Uuid::nil)),
        audio_event_schema::seen_at.eq(body.seen_at.unwrap_or_else(Utc::now)),
        audio_event_schema::offset_ms.eq(body.offset_ms.unwrap_or(0) as i32),
        audio_event_schema::ended_at.eq(body.ended_at.unwrap_or_else(Utc::now)),
        audio_event_schema::duration_ms.eq(body.duration_ms.unwrap_or(0) as i32),
        audio_event_schema::kind.eq(body.kind.clone().unwrap_or_default()),
        audio_event_schema::rms_dbfs.eq(body.rms_dbfs.unwrap_or_default()),
        audio_event_schema::peak_dbfs.eq(body.peak_dbfs.unwrap_or_default()),
        audio_event_schema::floor_dbfs.eq(body.floor_dbfs.unwrap_or_default()),
        audio_event_schema::prominence_db.eq(body.prominence_db.unwrap_or_default()),
        audio_event_schema::crest_db.eq(body.crest_db.unwrap_or_default()),
        audio_event_schema::score.eq(body.score.unwrap_or_default()),
        audio_event_schema::spectral.eq(json_of(&body.spectral)),
    ),
    load: load_relations,
}
