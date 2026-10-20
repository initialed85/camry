// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "point", schema = "pg_catalog"))]
    pub struct Point;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "polygon", schema = "pg_catalog"))]
    pub struct Polygon;
}

diesel::table! {
    audio_event (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        camera_id -> Uuid,
        video_id -> Uuid,
        seen_at -> Timestamptz,
        offset_ms -> Int4,
        ended_at -> Timestamptz,
        duration_ms -> Int4,
        kind -> Text,
        rms_dbfs -> Float8,
        peak_dbfs -> Float8,
        floor_dbfs -> Float8,
        prominence_db -> Float8,
        crest_db -> Float8,
        score -> Float8,
        spectral -> Jsonb,
    }
}

diesel::table! {
    camera (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        name -> Text,
        stream_url -> Text,
        last_seen -> Timestamptz,
        segment_producer_claimed_until -> Timestamptz,
        stream_producer_claimed_until -> Timestamptz,
        low_res_stream_url -> Nullable<Text>,
        overlay_offset_ms -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Point;
    use super::sql_types::Polygon;

    detection (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        seen_at -> Timestamptz,
        class_id -> Int8,
        class_name -> Text,
        score -> Float8,
        centroid -> Point,
        bounding_box -> Polygon,
        video_id -> Uuid,
        camera_id -> Uuid,
        track_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Point;
    use super::sql_types::Polygon;

    track (id) {
        id -> Uuid,
        key -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        camera_id -> Uuid,
        video_id -> Uuid,
        class_id -> Int8,
        class_name -> Text,
        started_at -> Timestamptz,
        ended_at -> Timestamptz,
        frame_count -> Int4,
        mean_score -> Float8,
        max_score -> Float8,
        centroid -> Point,
        bounding_box -> Polygon,
        trajectory -> Jsonb,
        is_static -> Bool,
        status -> Text,
    }
}

diesel::table! {
    video (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        file_name -> Text,
        started_at -> Timestamptz,
        ended_at -> Nullable<Timestamptz>,
        duration -> Nullable<Interval>,
        file_size -> Nullable<Float8>,
        thumbnail_name -> Nullable<Text>,
        status -> Nullable<Text>,
        object_detector_claimed_until -> Timestamptz,
        object_tracker_claimed_until -> Timestamptz,
        camera_id -> Uuid,
        detection_summary -> Jsonb,
        is_low_res -> Bool,
        track_summary -> Jsonb,
        overlay_offset_ms -> Nullable<Int4>,
        audio_detector_claimed_until -> Timestamptz,
        audio_detected_at -> Nullable<Timestamptz>,
        audio_profile -> Jsonb,
        audio_summary -> Jsonb,
    }
}

diesel::joinable!(audio_event -> camera (camera_id));
diesel::joinable!(audio_event -> video (video_id));
diesel::joinable!(detection -> camera (camera_id));
diesel::joinable!(detection -> track (track_id));
diesel::joinable!(detection -> video (video_id));
diesel::joinable!(track -> camera (camera_id));
diesel::joinable!(track -> video (video_id));
diesel::joinable!(video -> camera (camera_id));

diesel::allow_tables_to_appear_in_same_query!(audio_event, camera, detection, track, video,);
