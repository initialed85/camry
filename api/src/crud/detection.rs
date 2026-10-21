//! /detections

use crate::crud::{
    self, camera, point_into, point_of, polygon_into, polygon_of, track, video,
};
use crate::models as db;
use crate::schema::{
    camera as camera_schema, detection, track as track_schema, video as video_schema,
};
use chrono::Utc;
use diesel::prelude::*;
use openapi::models as wire;
use uuid::Uuid;

fn wire_of(row: &db::Detection) -> wire::Detection {
    wire::Detection {
        id: Some(row.id),
        created_at: Some(row.created_at),
        updated_at: Some(row.updated_at),
        deleted_at: row.deleted_at,
        seen_at: Some(row.seen_at),
        class_id: Some(row.class_id),
        class_name: Some(row.class_name.clone()),
        score: Some(row.score),
        centroid: Some(point_of(row.centroid)),
        bounding_box: Some(polygon_of(&row.bounding_box)),
        video_id: Some(row.video_id),
        camera_id: Some(row.camera_id),
        track_id: row.track_id,
        ..wire::Detection::new()
    }
}

/// Belongs-to objects at the default depth.
fn load_relations(
    conn: &mut PgConnection,
    objects: &mut [wire::Detection],
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

    let track_ids: Vec<Uuid> = objects.iter().filter_map(|object| object.track_id).collect();

    if !track_ids.is_empty() {
        let rows: Vec<db::Track> = track_schema::table
            .filter(track_schema::id.eq_any(&track_ids))
            .load(conn)
            .map_err(crud::Error::db)?;
        let children = rows
            .iter()
            .map(|row| (row.id, track::to_wire(row)))
            .collect();

        crud::attach(objects, children, |object| object.track_id, |object, joined| {
            object.track_id_object = Some(joined)
        });
    }

    Ok(())
}

crud_model! {
    table: detection,
    model: db::Detection,
    wire: wire::Detection,
    path: "/detections",
    columns: [
        id: Uuid,
        created_at: Timestamptz,
        updated_at: Timestamptz,
        deleted_at: Timestamptz,
        seen_at: Timestamptz,
        class_id: Int8,
        class_name: Text,
        score: Float8,
        centroid: Point,
        bounding_box: Polygon,
        video_id: Uuid,
        camera_id: Uuid,
        track_id: Uuid,
    ],
    relations: ["video", "camera", "track"],
    to_wire: wire_of,
    insert: |body: &wire::Detection| (
        detection::seen_at.eq(body.seen_at.unwrap_or_else(Utc::now)),
        detection::class_id.eq(body.class_id.unwrap_or_default()),
        detection::class_name.eq(body.class_name.clone().unwrap_or_default()),
        detection::score.eq(body.score.unwrap_or_default()),
        detection::centroid.eq(body.centroid.as_ref().map(point_into).unwrap_or_default()),
        detection::bounding_box.eq(polygon_into(&body.bounding_box)),
        detection::video_id.eq(body.video_id.unwrap_or_else(Uuid::nil)),
        detection::camera_id.eq(body.camera_id.unwrap_or_else(Uuid::nil)),
        detection::track_id.eq(body.track_id),
    ),
    patch: |body: &wire::Detection| (
        detection::deleted_at.eq(body.deleted_at),
        detection::seen_at.eq(body.seen_at.unwrap_or_else(Utc::now)),
        detection::class_id.eq(body.class_id.unwrap_or_default()),
        detection::class_name.eq(body.class_name.clone().unwrap_or_default()),
        detection::score.eq(body.score.unwrap_or_default()),
        detection::centroid.eq(body.centroid.as_ref().map(point_into).unwrap_or_default()),
        detection::bounding_box.eq(polygon_into(&body.bounding_box)),
        detection::video_id.eq(body.video_id.unwrap_or_else(Uuid::nil)),
        detection::camera_id.eq(body.camera_id.unwrap_or_else(Uuid::nil)),
        detection::track_id.eq(body.track_id),
    ),
    load: load_relations,
}
