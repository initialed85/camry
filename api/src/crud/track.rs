//! /tracks

use crate::crud::{
    self, camera, json_of, object_of, point_into, point_of, polygon_into, polygon_of, video,
};
use crate::models as db;
use crate::schema::{
    camera as camera_schema, track as track_schema, video as video_schema,
};
use chrono::Utc;
use diesel::prelude::*;
use openapi::models as wire;
use uuid::Uuid;

fn wire_of(row: &db::Track) -> wire::Track {
    wire::Track {
        id: Some(row.id),
        key: Some(row.key),
        created_at: Some(row.created_at),
        updated_at: Some(row.updated_at),
        deleted_at: row.deleted_at,
        camera_id: Some(row.camera_id),
        video_id: Some(row.video_id),
        class_id: Some(row.class_id),
        class_name: Some(row.class_name.clone()),
        started_at: Some(row.started_at),
        ended_at: Some(row.ended_at),
        frame_count: Some(row.frame_count as i64),
        mean_score: Some(row.mean_score),
        max_score: Some(row.max_score),
        centroid: Some(point_of(row.centroid)),
        bounding_box: Some(polygon_of(&row.bounding_box)),
        trajectory: Some(object_of(&row.trajectory)),
        is_static: Some(row.is_static),
        status: Some(row.status.clone()),
        ..wire::Track::new()
    }
}

fn load_relations(conn: &mut PgConnection, objects: &mut [wire::Track]) -> crud::Result<()> {
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
    table: track,
    model: db::Track,
    wire: wire::Track,
    path: "/tracks",
    columns: [
        id: Uuid,
        key: Uuid,
        created_at: Timestamptz,
        updated_at: Timestamptz,
        deleted_at: Timestamptz,
        camera_id: Uuid,
        video_id: Uuid,
        class_id: Int8,
        class_name: Text,
        started_at: Timestamptz,
        ended_at: Timestamptz,
        frame_count: Int4,
        mean_score: Float8,
        max_score: Float8,
        centroid: Point,
        bounding_box: Polygon,
        trajectory: Jsonb,
        is_static: Bool,
        status: Text,
    ],
    relations: ["camera", "video"],
    to_wire: wire_of,
    insert: |body: &wire::Track| (
        track_schema::key.eq(body.key.unwrap_or_else(Uuid::nil)),
        track_schema::camera_id.eq(body.camera_id.unwrap_or_else(Uuid::nil)),
        track_schema::video_id.eq(body.video_id.unwrap_or_else(Uuid::nil)),
        track_schema::class_id.eq(body.class_id.unwrap_or_default()),
        track_schema::class_name.eq(body.class_name.clone().unwrap_or_default()),
        track_schema::started_at.eq(body.started_at.unwrap_or_else(Utc::now)),
        track_schema::ended_at.eq(body.ended_at.unwrap_or_else(Utc::now)),
        track_schema::frame_count.eq(body.frame_count.unwrap_or(0) as i32),
        track_schema::mean_score.eq(body.mean_score.unwrap_or_default()),
        track_schema::max_score.eq(body.max_score.unwrap_or_default()),
        track_schema::centroid.eq(body.centroid.as_ref().map(point_into).unwrap_or_default()),
        track_schema::bounding_box.eq(polygon_into(&body.bounding_box)),
        track_schema::trajectory.eq(json_of(&body.trajectory)),
        track_schema::is_static.eq(body.is_static.unwrap_or(false)),
        track_schema::status.eq(body.status.clone().unwrap_or_default()),
    ),
    patch: |body: &wire::Track| (
        track_schema::deleted_at.eq(body.deleted_at),
        track_schema::camera_id.eq(body.camera_id.unwrap_or_else(Uuid::nil)),
        track_schema::video_id.eq(body.video_id.unwrap_or_else(Uuid::nil)),
        track_schema::class_id.eq(body.class_id.unwrap_or_default()),
        track_schema::class_name.eq(body.class_name.clone().unwrap_or_default()),
        track_schema::started_at.eq(body.started_at.unwrap_or_else(Utc::now)),
        track_schema::ended_at.eq(body.ended_at.unwrap_or_else(Utc::now)),
        track_schema::frame_count.eq(body.frame_count.unwrap_or(0) as i32),
        track_schema::mean_score.eq(body.mean_score.unwrap_or_default()),
        track_schema::max_score.eq(body.max_score.unwrap_or_default()),
        track_schema::centroid.eq(body.centroid.as_ref().map(point_into).unwrap_or_default()),
        track_schema::bounding_box.eq(polygon_into(&body.bounding_box)),
        track_schema::trajectory.eq(json_of(&body.trajectory)),
        track_schema::is_static.eq(body.is_static.unwrap_or(false)),
        track_schema::status.eq(body.status.clone().unwrap_or_default()),
    ),
    load: load_relations,
}
