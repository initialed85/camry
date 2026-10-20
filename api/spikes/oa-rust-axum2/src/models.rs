#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[allow(dead_code)]
pub type SSE = std::pin::Pin<std::boxed::Box<dyn futures_util::Stream<Item = std::result::Result<axum::response::sse::Event, std::convert::Infallible>> + std::marker::Send + std::marker::Sync>>;

#[allow(dead_code)]
fn from_validation_error(e: validator::ValidationError) -> validator::ValidationErrors {
    let mut errs = validator::ValidationErrors::new();
    errs.add("na", e);
    errs
}

#[allow(dead_code)]
pub fn check_xss_string(v: &str) -> std::result::Result<(), validator::ValidationError> {
    if ammonia::is_html(v) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_vec_string(v: &[String]) -> std::result::Result<(), validator::ValidationError> {
    if v.iter().any(|i| ammonia::is_html(i)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_string(
    v: &std::collections::HashMap<String, String>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| ammonia::is_html(v)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_nested<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError>
where
    T: validator::Validate,
{
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| v.validate().is_err()) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map<T>(v: &std::collections::HashMap<String, T>) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostAudioDetectorClaimVideosQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_AudioEvent_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_audio_event_video_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Detection_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_detection_video_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Track_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_track_video_id__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "file_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "file_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "file_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "file_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "started_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "started_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "started_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "started_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "started_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "started_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "started_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "started_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "ended_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "ended_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "ended_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "ended_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "ended_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "ended_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "duration__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "duration__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__ne: Option<String>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "duration__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__gt: Option<String>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "duration__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__gte: Option<String>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "duration__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__lt: Option<String>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "duration__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__lte: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "duration__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "duration__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__notin: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "file_size__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "file_size__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "file_size__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "file_size__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "file_size__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "file_size__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "file_size__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "file_size__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__notin: Option<f64>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "thumbnail_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "thumbnail_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "thumbnail_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "thumbnail_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "status__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "status__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "status__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "status__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "object_detector_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "object_detector_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "object_detector_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "object_detector_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_detector_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_detector_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "object_tracker_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "object_tracker_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "object_tracker_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "object_tracker_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_tracker_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_tracker_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "camera_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "camera_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "camera_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "camera_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "camera_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "camera_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "detection_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "detection_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "detection_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "detection_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "is_low_res__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__eq: Option<bool>,
            /// SQL != comparison
                #[serde(rename = "is_low_res__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__ne: Option<bool>,
            /// SQL IS false comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__isfalse")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__isfalse: Option<String>,
            /// SQL IS true comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__istrue")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__istrue: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "track_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "track_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "overlay_offset_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "overlay_offset_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__notin: Option<i32>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "audio_detector_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "audio_detector_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "audio_detector_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "audio_detector_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detector_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detector_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "audio_detected_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "audio_detected_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "audio_detected_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "audio_detected_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "audio_profile__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "audio_profile__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_profile__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_profile__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "audio_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "audio_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteAudioEventPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteAudioEventQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetAudioEventPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetAudioEventQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetAudioEventsQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera__load: Option<String>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "video__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "camera_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "camera_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "camera_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "camera_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "camera_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "camera_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "video_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "video_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "video_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "video_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "video_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "video_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "video_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "video_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "video_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "video_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "seen_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "seen_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "seen_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "seen_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "seen_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "seen_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "seen_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "seen_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "seen_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "seen_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "offset_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "offset_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "offset_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "offset_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "offset_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "offset_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "offset_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "offset_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__notin: Option<i32>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "offset_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "offset_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset_ms__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "ended_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "ended_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "ended_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "ended_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "ended_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "ended_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "duration_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "duration_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "duration_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "duration_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "duration_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "duration_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "duration_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "duration_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__notin: Option<i32>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration_ms__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "kind__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "kind__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "kind__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "kind__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "kind__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "kind__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "kind__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "kind__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "kind__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "kind__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub kind__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "rms_dbfs__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "rms_dbfs__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "rms_dbfs__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "rms_dbfs__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "rms_dbfs__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "rms_dbfs__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "rms_dbfs__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "rms_dbfs__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "rms_dbfs__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "rms_dbfs__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub rms_dbfs__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "peak_dbfs__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "peak_dbfs__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "peak_dbfs__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "peak_dbfs__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "peak_dbfs__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "peak_dbfs__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "peak_dbfs__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "peak_dbfs__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "peak_dbfs__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "peak_dbfs__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub peak_dbfs__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "floor_dbfs__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "floor_dbfs__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "floor_dbfs__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "floor_dbfs__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "floor_dbfs__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "floor_dbfs__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "floor_dbfs__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "floor_dbfs__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "floor_dbfs__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "floor_dbfs__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub floor_dbfs__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "prominence_db__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "prominence_db__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "prominence_db__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "prominence_db__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "prominence_db__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "prominence_db__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "prominence_db__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "prominence_db__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "prominence_db__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "prominence_db__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub prominence_db__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "crest_db__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "crest_db__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "crest_db__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "crest_db__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "crest_db__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "crest_db__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "crest_db__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "crest_db__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "crest_db__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "crest_db__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub crest_db__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "score__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "score__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "score__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "score__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "score__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "score__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "score__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "score__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "score__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "score__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "spectral__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub spectral__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "spectral__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub spectral__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "spectral__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub spectral__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "spectral__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub spectral__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchAudioEventPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchAudioEventQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostAudioEventsQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteCameraPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteCameraQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetCameraPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetCameraQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetCamerasQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_AudioEvent_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_audio_event_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Detection_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_detection_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Track_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_track_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Video_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_video_camera_id__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "stream_url__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "stream_url__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "stream_url__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "stream_url__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_url__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_url__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "last_seen__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "last_seen__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "last_seen__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "last_seen__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "last_seen__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "last_seen__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "last_seen__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "last_seen__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "last_seen__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "last_seen__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "segment_producer_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "segment_producer_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "segment_producer_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "segment_producer_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "segment_producer_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "segment_producer_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "stream_producer_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "stream_producer_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "stream_producer_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "stream_producer_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_producer_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_producer_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "low_res_stream_url__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "low_res_stream_url__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "low_res_stream_url__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "low_res_stream_url__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "overlay_offset_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "overlay_offset_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__notin: Option<i32>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchCameraPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchCameraQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostCamerasQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostCamerasSegmentProducerClaimPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostCamerasSegmentProducerClaimQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostCamerasStreamProducerClaimPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostCamerasStreamProducerClaimQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteDetectionPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteDetectionQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetDetectionPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetDetectionQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetDetectionsQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "video__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video__load: Option<String>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera__load: Option<String>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "track__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "seen_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "seen_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "seen_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "seen_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "seen_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "seen_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "seen_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "seen_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "seen_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "seen_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub seen_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "class_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "class_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "class_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "class_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "class_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "class_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "class_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "class_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__notin: Option<i32>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "class_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "class_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "class_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "class_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "class_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "class_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "class_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "class_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "class_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "class_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "class_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "class_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "score__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "score__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "score__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "score__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "score__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "score__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "score__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "score__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "score__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "score__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub score__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "video_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "video_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "video_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "video_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "video_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "video_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "video_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "video_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "video_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "video_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "camera_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "camera_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "camera_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "camera_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "camera_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "camera_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "track_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "track_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "track_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "track_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "track_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "track_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "track_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "track_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__notin: Option<uuid::Uuid>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_id__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_id__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_id__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchDetectionPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchDetectionQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostDetectionsQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostObjectDetectorClaimVideosQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_AudioEvent_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_audio_event_video_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Detection_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_detection_video_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Track_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_track_video_id__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "file_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "file_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "file_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "file_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "started_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "started_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "started_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "started_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "started_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "started_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "started_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "started_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "ended_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "ended_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "ended_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "ended_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "ended_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "ended_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "duration__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "duration__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__ne: Option<String>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "duration__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__gt: Option<String>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "duration__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__gte: Option<String>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "duration__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__lt: Option<String>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "duration__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__lte: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "duration__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "duration__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__notin: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "file_size__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "file_size__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "file_size__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "file_size__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "file_size__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "file_size__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "file_size__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "file_size__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__notin: Option<f64>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "thumbnail_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "thumbnail_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "thumbnail_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "thumbnail_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "status__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "status__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "status__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "status__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "object_detector_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "object_detector_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "object_detector_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "object_detector_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_detector_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_detector_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "object_tracker_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "object_tracker_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "object_tracker_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "object_tracker_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_tracker_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_tracker_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "camera_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "camera_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "camera_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "camera_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "camera_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "camera_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "detection_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "detection_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "detection_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "detection_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "is_low_res__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__eq: Option<bool>,
            /// SQL != comparison
                #[serde(rename = "is_low_res__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__ne: Option<bool>,
            /// SQL IS false comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__isfalse")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__isfalse: Option<String>,
            /// SQL IS true comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__istrue")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__istrue: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "track_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "track_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "overlay_offset_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "overlay_offset_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__notin: Option<i32>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "audio_detector_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "audio_detector_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "audio_detector_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "audio_detector_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detector_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detector_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "audio_detected_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "audio_detected_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "audio_detected_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "audio_detected_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "audio_profile__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "audio_profile__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_profile__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_profile__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "audio_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "audio_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostObjectTrackerClaimVideosQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_AudioEvent_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_audio_event_video_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Detection_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_detection_video_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Track_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_track_video_id__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "file_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "file_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "file_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "file_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "started_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "started_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "started_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "started_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "started_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "started_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "started_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "started_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "ended_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "ended_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "ended_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "ended_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "ended_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "ended_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "duration__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "duration__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__ne: Option<String>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "duration__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__gt: Option<String>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "duration__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__gte: Option<String>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "duration__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__lt: Option<String>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "duration__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__lte: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "duration__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "duration__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__notin: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "file_size__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "file_size__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "file_size__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "file_size__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "file_size__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "file_size__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "file_size__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "file_size__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__notin: Option<f64>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "thumbnail_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "thumbnail_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "thumbnail_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "thumbnail_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "status__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "status__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "status__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "status__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "object_detector_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "object_detector_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "object_detector_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "object_detector_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_detector_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_detector_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "object_tracker_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "object_tracker_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "object_tracker_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "object_tracker_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_tracker_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_tracker_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "camera_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "camera_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "camera_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "camera_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "camera_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "camera_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "detection_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "detection_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "detection_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "detection_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "is_low_res__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__eq: Option<bool>,
            /// SQL != comparison
                #[serde(rename = "is_low_res__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__ne: Option<bool>,
            /// SQL IS false comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__isfalse")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__isfalse: Option<String>,
            /// SQL IS true comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__istrue")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__istrue: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "track_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "track_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "overlay_offset_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "overlay_offset_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__notin: Option<i32>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "audio_detector_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "audio_detector_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "audio_detector_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "audio_detector_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detector_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detector_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "audio_detected_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "audio_detected_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "audio_detected_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "audio_detected_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "audio_profile__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "audio_profile__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_profile__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_profile__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "audio_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "audio_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostSegmentProducerClaimCamerasQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_AudioEvent_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_audio_event_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Detection_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_detection_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Track_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_track_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Video_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_video_camera_id__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "stream_url__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "stream_url__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "stream_url__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "stream_url__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_url__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_url__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "last_seen__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "last_seen__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "last_seen__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "last_seen__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "last_seen__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "last_seen__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "last_seen__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "last_seen__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "last_seen__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "last_seen__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "segment_producer_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "segment_producer_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "segment_producer_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "segment_producer_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "segment_producer_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "segment_producer_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "stream_producer_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "stream_producer_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "stream_producer_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "stream_producer_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_producer_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_producer_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "low_res_stream_url__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "low_res_stream_url__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "low_res_stream_url__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "low_res_stream_url__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "overlay_offset_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "overlay_offset_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__notin: Option<i32>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostStreamProducerClaimCamerasQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_AudioEvent_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_audio_event_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Detection_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_detection_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Track_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_track_camera_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Video_CameraID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_video_camera_id__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "stream_url__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "stream_url__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "stream_url__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "stream_url__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "stream_url__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_url__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_url__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_url__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "last_seen__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "last_seen__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "last_seen__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "last_seen__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "last_seen__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "last_seen__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "last_seen__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "last_seen__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "last_seen__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "last_seen__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub last_seen__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "segment_producer_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "segment_producer_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "segment_producer_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "segment_producer_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "segment_producer_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "segment_producer_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "segment_producer_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub segment_producer_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "stream_producer_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "stream_producer_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "stream_producer_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "stream_producer_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "stream_producer_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_producer_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "stream_producer_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub stream_producer_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "low_res_stream_url__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "low_res_stream_url__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "low_res_stream_url__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "low_res_stream_url__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "low_res_stream_url__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "low_res_stream_url__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub low_res_stream_url__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "overlay_offset_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "overlay_offset_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__notin: Option<i32>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteTrackPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteTrackQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetTrackPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetTrackQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetTracksQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera__load: Option<String>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "video__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Detection_TrackID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_detection_track_id__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "key__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "key__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "key__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "key__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "key__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "key__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "key__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "key__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "key__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "key__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub key__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "camera_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "camera_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "camera_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "camera_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "camera_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "camera_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "video_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "video_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "video_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "video_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "video_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "video_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "video_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "video_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "video_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "video_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub video_id__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "class_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "class_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "class_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "class_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "class_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "class_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "class_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "class_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__notin: Option<i32>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "class_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "class_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_id__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "class_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "class_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "class_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "class_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "class_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "class_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "class_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "class_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "class_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "class_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub class_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "started_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "started_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "started_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "started_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "started_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "started_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "started_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "started_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "ended_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "ended_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "ended_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "ended_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "ended_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "ended_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "frame_count__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "frame_count__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "frame_count__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "frame_count__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "frame_count__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "frame_count__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "frame_count__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "frame_count__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__notin: Option<i32>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "frame_count__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "frame_count__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub frame_count__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "mean_score__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "mean_score__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "mean_score__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "mean_score__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "mean_score__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "mean_score__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "mean_score__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "mean_score__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "mean_score__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "mean_score__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub mean_score__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "max_score__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "max_score__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "max_score__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "max_score__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "max_score__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "max_score__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "max_score__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "max_score__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__notin: Option<f64>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "max_score__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "max_score__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub max_score__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "trajectory__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub trajectory__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "trajectory__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub trajectory__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "trajectory__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub trajectory__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "trajectory__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub trajectory__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "is_static__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_static__eq: Option<bool>,
            /// SQL != comparison
                #[serde(rename = "is_static__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_static__ne: Option<bool>,
            /// SQL IS false comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_static__isfalse")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_static__isfalse: Option<String>,
            /// SQL IS true comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_static__istrue")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_static__istrue: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_static__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_static__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_static__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_static__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "status__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "status__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "status__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "status__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchTrackPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchTrackQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostTracksQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteVideoPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteVideoQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetVideoPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetVideoQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetVideosQueryParams {
            /// SQL LIMIT operator
                #[serde(rename = "limit")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<i32>,
            /// SQL OFFSET operator
                #[serde(rename = "offset")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub offset: Option<i32>,
            /// Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc)
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i32>,
            /// load the given outbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_AudioEvent_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_audio_event_video_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Detection_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_detection_video_id__load: Option<String>,
            /// load the given inbound relationship, value is ignored (presence of key is sufficient)
                #[serde(rename = "referenced_by_Track_VideoID__load")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub referenced_by_track_video_id__load: Option<String>,
            /// SQL = comparison
                #[serde(rename = "created_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "created_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "created_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "created_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "created_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "created_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "created_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "created_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "created_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub created_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "updated_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "updated_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "updated_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "updated_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "updated_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "updated_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "updated_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "updated_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub updated_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "deleted_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "deleted_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "deleted_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "deleted_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "deleted_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "deleted_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "deleted_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "deleted_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub deleted_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "file_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "file_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "file_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "file_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "file_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__notilike: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "started_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "started_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "started_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "started_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "started_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "started_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "started_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "started_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "started_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub started_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "ended_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "ended_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "ended_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "ended_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "ended_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "ended_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "ended_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "ended_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub ended_at__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "duration__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "duration__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__ne: Option<String>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "duration__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__gt: Option<String>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "duration__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__gte: Option<String>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "duration__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__lt: Option<String>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "duration__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__lte: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "duration__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "duration__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__notin: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "duration__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub duration__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "file_size__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__eq: Option<f64>,
            /// SQL != comparison
                #[serde(rename = "file_size__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__ne: Option<f64>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "file_size__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__gt: Option<f64>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "file_size__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__gte: Option<f64>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "file_size__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__lt: Option<f64>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "file_size__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__lte: Option<f64>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "file_size__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__in: Option<f64>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "file_size__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__notin: Option<f64>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "file_size__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub file_size__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "thumbnail_name__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "thumbnail_name__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "thumbnail_name__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "thumbnail_name__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "thumbnail_name__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "thumbnail_name__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub thumbnail_name__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "status__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__eq: Option<String>,
            /// SQL != comparison
                #[serde(rename = "status__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ne: Option<String>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "status__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__in: Option<String>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "status__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notin: Option<String>,
            /// SQL LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__like")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__like: Option<String>,
            /// SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notlike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notlike: Option<String>,
            /// SQL ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__ilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__ilike: Option<String>,
            /// SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with %
                #[serde(rename = "status__notilike")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__notilike: Option<String>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "status__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub status__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "object_detector_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "object_detector_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "object_detector_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "object_detector_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "object_detector_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_detector_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_detector_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_detector_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "object_tracker_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "object_tracker_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "object_tracker_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "object_tracker_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "object_tracker_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_tracker_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "object_tracker_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub object_tracker_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "camera_id__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__eq: Option<uuid::Uuid>,
            /// SQL != comparison
                #[serde(rename = "camera_id__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__ne: Option<uuid::Uuid>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "camera_id__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gt: Option<uuid::Uuid>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "camera_id__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__gte: Option<uuid::Uuid>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "camera_id__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lt: Option<uuid::Uuid>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "camera_id__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__lte: Option<uuid::Uuid>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__in: Option<uuid::Uuid>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "camera_id__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__notin: Option<uuid::Uuid>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "camera_id__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub camera_id__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "detection_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "detection_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "detection_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "detection_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub detection_summary__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "is_low_res__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__eq: Option<bool>,
            /// SQL != comparison
                #[serde(rename = "is_low_res__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__ne: Option<bool>,
            /// SQL IS false comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__isfalse")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__isfalse: Option<String>,
            /// SQL IS true comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__istrue")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__istrue: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "is_low_res__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub is_low_res__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "track_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "track_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "track_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub track_summary__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "overlay_offset_ms__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__eq: Option<i32>,
            /// SQL != comparison
                #[serde(rename = "overlay_offset_ms__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__ne: Option<i32>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gt: Option<i32>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__gte: Option<i32>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lt: Option<i32>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "overlay_offset_ms__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__lte: Option<i32>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__in: Option<i32>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "overlay_offset_ms__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__notin: Option<i32>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "overlay_offset_ms__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub overlay_offset_ms__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "audio_detector_claimed_until__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "audio_detector_claimed_until__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "audio_detector_claimed_until__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "audio_detector_claimed_until__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "audio_detector_claimed_until__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detector_claimed_until__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detector_claimed_until__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detector_claimed_until__desc: Option<String>,
            /// SQL = comparison
                #[serde(rename = "audio_detected_at__eq")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__eq: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL != comparison
                #[serde(rename = "audio_detected_at__ne")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__ne: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL > comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__gt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__gt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL >= comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__gte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__gte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL < comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__lt")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__lt: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL <= comparison, may not work with all column types
                #[serde(rename = "audio_detected_at__lte")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__lte: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IN comparison, permits comma-separated values
                #[serde(rename = "audio_detected_at__in")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__in: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL NOT IN comparison, permits comma-separated values
                #[serde(rename = "audio_detected_at__notin")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__notin: Option<chrono::DateTime::<chrono::Utc>>,
            /// SQL IS null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__isnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__isnull: Option<String>,
            /// SQL IS NOT null comparison, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__isnotnull")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__isnotnull: Option<String>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_detected_at__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_detected_at__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "audio_profile__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "audio_profile__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_profile__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_profile__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_profile__desc: Option<String>,
            /// SQL @> comparison
                #[serde(rename = "audio_summary__contains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__contains: Option<crate::types::Object>,
            /// SQL NOT @> comparison
                #[serde(rename = "audio_summary__notcontains")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__notcontains: Option<crate::types::Object>,
            /// SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_summary__asc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__asc: Option<String>,
            /// SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient)
                #[serde(rename = "audio_summary__desc")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub audio_summary__desc: Option<String>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchVideoPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PatchVideoQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostVideosQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostVideosAudioDetectorClaimPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostVideosAudioDetectorClaimQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostVideosObjectDetectorClaimPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostVideosObjectDetectorClaimQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostVideosObjectTrackerClaimPathParams {
            /// Path parameter primaryKey
                pub primary_key: uuid::Uuid,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostVideosObjectTrackerClaimQueryParams {
            /// Query parameter depth
                #[serde(rename = "depth")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub depth: Option<i64>,
    }



#[derive(Debug, Clone, PartialEq,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Any(pub crate::types::Object);

impl validator::Validate for Any {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<crate::types::Object> for Any {
    fn from(x: crate::types::Object) -> Self {
        Any(x)
    }
}

impl std::convert::From<Any> for crate::types::Object {
    fn from(x: Any) -> Self {
        x.0
    }
}

impl std::ops::Deref for Any {
    type Target = crate::types::Object;
    fn deref(&self) -> &crate::types::Object {
        &self.0
    }
}

impl std::ops::DerefMut for Any {
    fn deref_mut(&mut self) -> &mut crate::types::Object {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ArrayOfVec2Inner {
    #[serde(rename = "X")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub x: Option<f64>,

    #[serde(rename = "Y")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub y: Option<f64>,

}



impl ArrayOfVec2Inner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ArrayOfVec2Inner {
        ArrayOfVec2Inner {
 x: None,
 y: None,
        }
    }
}

/// Converts the ArrayOfVec2Inner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ArrayOfVec2Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.x.as_ref().map(|x| {
                [
                    "X".to_string(),
                    x.to_string(),
                ].join(",")
            }),


            self.y.as_ref().map(|y| {
                [
                    "Y".to_string(),
                    y.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ArrayOfVec2Inner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ArrayOfVec2Inner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub x: Vec<f64>,
            pub y: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ArrayOfVec2Inner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "X" => intermediate_rep.x.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "Y" => intermediate_rep.y.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ArrayOfVec2Inner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ArrayOfVec2Inner {
            x: intermediate_rep.x.into_iter().next(),
            y: intermediate_rep.y.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ArrayOfVec2Inner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ArrayOfVec2Inner>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ArrayOfVec2Inner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ArrayOfVec2Inner - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ArrayOfVec2Inner> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ArrayOfVec2Inner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ArrayOfVec2Inner - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AudioEvent {
    #[serde(rename = "camera_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id: Option<uuid::Uuid>,

    #[serde(rename = "camera_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id_object: Option<models::Camera>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "crest_db")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crest_db: Option<f64>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "duration_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration_ms: Option<i64>,

    #[serde(rename = "ended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ended_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "floor_dbfs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub floor_dbfs: Option<f64>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "kind")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kind: Option<String>,

    #[serde(rename = "offset_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset_ms: Option<i64>,

    #[serde(rename = "peak_dbfs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub peak_dbfs: Option<f64>,

    #[serde(rename = "prominence_db")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prominence_db: Option<f64>,

    #[serde(rename = "rms_dbfs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rms_dbfs: Option<f64>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<f64>,

    #[serde(rename = "seen_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seen_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "spectral")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub spectral: Option<crate::types::Object>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "video_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id: Option<uuid::Uuid>,

    #[serde(rename = "video_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id_object: Option<models::Video>,

}



impl AudioEvent {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AudioEvent {
        AudioEvent {
 camera_id: None,
 camera_id_object: None,
 created_at: None,
 crest_db: None,
 deleted_at: None,
 duration_ms: None,
 ended_at: None,
 floor_dbfs: None,
 id: None,
 kind: None,
 offset_ms: None,
 peak_dbfs: None,
 prominence_db: None,
 rms_dbfs: None,
 score: None,
 seen_at: None,
 spectral: None,
 updated_at: None,
 video_id: None,
 video_id_object: None,
        }
    }
}

/// Converts the AudioEvent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for AudioEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping camera_id in query parameter serialization

            // Skipping camera_id_object in query parameter serialization

            // Skipping created_at in query parameter serialization


            self.crest_db.as_ref().map(|crest_db| {
                [
                    "crest_db".to_string(),
                    crest_db.to_string(),
                ].join(",")
            }),

            // Skipping deleted_at in query parameter serialization


            self.duration_ms.as_ref().map(|duration_ms| {
                [
                    "duration_ms".to_string(),
                    duration_ms.to_string(),
                ].join(",")
            }),

            // Skipping ended_at in query parameter serialization


            self.floor_dbfs.as_ref().map(|floor_dbfs| {
                [
                    "floor_dbfs".to_string(),
                    floor_dbfs.to_string(),
                ].join(",")
            }),

            // Skipping id in query parameter serialization


            self.kind.as_ref().map(|kind| {
                [
                    "kind".to_string(),
                    kind.to_string(),
                ].join(",")
            }),


            self.offset_ms.as_ref().map(|offset_ms| {
                [
                    "offset_ms".to_string(),
                    offset_ms.to_string(),
                ].join(",")
            }),


            self.peak_dbfs.as_ref().map(|peak_dbfs| {
                [
                    "peak_dbfs".to_string(),
                    peak_dbfs.to_string(),
                ].join(",")
            }),


            self.prominence_db.as_ref().map(|prominence_db| {
                [
                    "prominence_db".to_string(),
                    prominence_db.to_string(),
                ].join(",")
            }),


            self.rms_dbfs.as_ref().map(|rms_dbfs| {
                [
                    "rms_dbfs".to_string(),
                    rms_dbfs.to_string(),
                ].join(",")
            }),


            self.score.as_ref().map(|score| {
                [
                    "score".to_string(),
                    score.to_string(),
                ].join(",")
            }),

            // Skipping seen_at in query parameter serialization

            // Skipping spectral in query parameter serialization

            // Skipping updated_at in query parameter serialization

            // Skipping video_id in query parameter serialization

            // Skipping video_id_object in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AudioEvent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AudioEvent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub camera_id: Vec<uuid::Uuid>,
            pub camera_id_object: Vec<models::Camera>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub crest_db: Vec<f64>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub duration_ms: Vec<i64>,
            pub ended_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub floor_dbfs: Vec<f64>,
            pub id: Vec<uuid::Uuid>,
            pub kind: Vec<String>,
            pub offset_ms: Vec<i64>,
            pub peak_dbfs: Vec<f64>,
            pub prominence_db: Vec<f64>,
            pub rms_dbfs: Vec<f64>,
            pub score: Vec<f64>,
            pub seen_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub spectral: Vec<crate::types::Object>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub video_id: Vec<uuid::Uuid>,
            pub video_id_object: Vec<models::Video>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AudioEvent".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "camera_id" => intermediate_rep.camera_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id_object" => intermediate_rep.camera_id_object.push(<models::Camera as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "crest_db" => intermediate_rep.crest_db.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "duration_ms" => intermediate_rep.duration_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ended_at" => intermediate_rep.ended_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "floor_dbfs" => intermediate_rep.floor_dbfs.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "offset_ms" => intermediate_rep.offset_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "peak_dbfs" => intermediate_rep.peak_dbfs.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prominence_db" => intermediate_rep.prominence_db.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "rms_dbfs" => intermediate_rep.rms_dbfs.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "seen_at" => intermediate_rep.seen_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spectral" => intermediate_rep.spectral.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id" => intermediate_rep.video_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id_object" => intermediate_rep.video_id_object.push(<models::Video as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AudioEvent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AudioEvent {
            camera_id: intermediate_rep.camera_id.into_iter().next(),
            camera_id_object: intermediate_rep.camera_id_object.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            crest_db: intermediate_rep.crest_db.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            duration_ms: intermediate_rep.duration_ms.into_iter().next(),
            ended_at: intermediate_rep.ended_at.into_iter().next(),
            floor_dbfs: intermediate_rep.floor_dbfs.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            kind: intermediate_rep.kind.into_iter().next(),
            offset_ms: intermediate_rep.offset_ms.into_iter().next(),
            peak_dbfs: intermediate_rep.peak_dbfs.into_iter().next(),
            prominence_db: intermediate_rep.prominence_db.into_iter().next(),
            rms_dbfs: intermediate_rep.rms_dbfs.into_iter().next(),
            score: intermediate_rep.score.into_iter().next(),
            seen_at: intermediate_rep.seen_at.into_iter().next(),
            spectral: intermediate_rep.spectral.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            video_id: intermediate_rep.video_id.into_iter().next(),
            video_id_object: intermediate_rep.video_id_object.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AudioEvent> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AudioEvent>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AudioEvent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for AudioEvent - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AudioEvent> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AudioEvent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into AudioEvent - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Camera {
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "last_seen")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_seen: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "low_res_stream_url")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub low_res_stream_url: Option<String>,

    #[serde(rename = "name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "overlay_offset_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub overlay_offset_ms: Option<i64>,

    #[serde(rename = "referenced_by_audio_event_camera_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_audio_event_camera_id_objects: Option<Vec<models::AudioEvent>>,

    #[serde(rename = "referenced_by_detection_camera_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_detection_camera_id_objects: Option<Vec<models::Detection>>,

    #[serde(rename = "referenced_by_track_camera_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_track_camera_id_objects: Option<Vec<models::Track>>,

    #[serde(rename = "referenced_by_video_camera_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_video_camera_id_objects: Option<Vec<models::Video>>,

    #[serde(rename = "segment_producer_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_producer_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "stream_producer_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stream_producer_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "stream_url")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stream_url: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

}



impl Camera {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Camera {
        Camera {
 created_at: None,
 deleted_at: None,
 id: None,
 last_seen: None,
 low_res_stream_url: None,
 name: None,
 overlay_offset_ms: None,
 referenced_by_audio_event_camera_id_objects: None,
 referenced_by_detection_camera_id_objects: None,
 referenced_by_track_camera_id_objects: None,
 referenced_by_video_camera_id_objects: None,
 segment_producer_claimed_until: None,
 stream_producer_claimed_until: None,
 stream_url: None,
 updated_at: None,
        }
    }
}

/// Converts the Camera value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping created_at in query parameter serialization

            // Skipping deleted_at in query parameter serialization

            // Skipping id in query parameter serialization

            // Skipping last_seen in query parameter serialization


            self.low_res_stream_url.as_ref().map(|low_res_stream_url| {
                [
                    "low_res_stream_url".to_string(),
                    low_res_stream_url.to_string(),
                ].join(",")
            }),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.overlay_offset_ms.as_ref().map(|overlay_offset_ms| {
                [
                    "overlay_offset_ms".to_string(),
                    overlay_offset_ms.to_string(),
                ].join(",")
            }),

            // Skipping referenced_by_audio_event_camera_id_objects in query parameter serialization

            // Skipping referenced_by_detection_camera_id_objects in query parameter serialization

            // Skipping referenced_by_track_camera_id_objects in query parameter serialization

            // Skipping referenced_by_video_camera_id_objects in query parameter serialization

            // Skipping segment_producer_claimed_until in query parameter serialization

            // Skipping stream_producer_claimed_until in query parameter serialization


            self.stream_url.as_ref().map(|stream_url| {
                [
                    "stream_url".to_string(),
                    stream_url.to_string(),
                ].join(",")
            }),

            // Skipping updated_at in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Camera value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Camera {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub id: Vec<uuid::Uuid>,
            pub last_seen: Vec<chrono::DateTime::<chrono::Utc>>,
            pub low_res_stream_url: Vec<String>,
            pub name: Vec<String>,
            pub overlay_offset_ms: Vec<i64>,
            pub referenced_by_audio_event_camera_id_objects: Vec<Vec<models::AudioEvent>>,
            pub referenced_by_detection_camera_id_objects: Vec<Vec<models::Detection>>,
            pub referenced_by_track_camera_id_objects: Vec<Vec<models::Track>>,
            pub referenced_by_video_camera_id_objects: Vec<Vec<models::Video>>,
            pub segment_producer_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub stream_producer_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub stream_url: Vec<String>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Camera".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last_seen" => intermediate_rep.last_seen.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "low_res_stream_url" => intermediate_rep.low_res_stream_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "overlay_offset_ms" => intermediate_rep.overlay_offset_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "referenced_by_audio_event_camera_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in Camera".to_string()),
                    "referenced_by_detection_camera_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in Camera".to_string()),
                    "referenced_by_track_camera_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in Camera".to_string()),
                    "referenced_by_video_camera_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in Camera".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "segment_producer_claimed_until" => intermediate_rep.segment_producer_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "stream_producer_claimed_until" => intermediate_rep.stream_producer_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "stream_url" => intermediate_rep.stream_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Camera".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Camera {
            created_at: intermediate_rep.created_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            last_seen: intermediate_rep.last_seen.into_iter().next(),
            low_res_stream_url: intermediate_rep.low_res_stream_url.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            overlay_offset_ms: intermediate_rep.overlay_offset_ms.into_iter().next(),
            referenced_by_audio_event_camera_id_objects: intermediate_rep.referenced_by_audio_event_camera_id_objects.into_iter().next(),
            referenced_by_detection_camera_id_objects: intermediate_rep.referenced_by_detection_camera_id_objects.into_iter().next(),
            referenced_by_track_camera_id_objects: intermediate_rep.referenced_by_track_camera_id_objects.into_iter().next(),
            referenced_by_video_camera_id_objects: intermediate_rep.referenced_by_video_camera_id_objects.into_iter().next(),
            segment_producer_claimed_until: intermediate_rep.segment_producer_claimed_until.into_iter().next(),
            stream_producer_claimed_until: intermediate_rep.stream_producer_claimed_until.into_iter().next(),
            stream_url: intermediate_rep.stream_url.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Camera> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Camera>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Camera>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Camera - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Camera> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Camera as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Camera - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CameraSegmentProducerClaimRequest {
    #[serde(rename = "timeout_seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeout_seconds: Option<f64>,

    #[serde(rename = "until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub until: Option<chrono::DateTime::<chrono::Utc>>,

}



impl CameraSegmentProducerClaimRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> CameraSegmentProducerClaimRequest {
        CameraSegmentProducerClaimRequest {
 timeout_seconds: None,
 until: None,
        }
    }
}

/// Converts the CameraSegmentProducerClaimRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CameraSegmentProducerClaimRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.timeout_seconds.as_ref().map(|timeout_seconds| {
                [
                    "timeout_seconds".to_string(),
                    timeout_seconds.to_string(),
                ].join(",")
            }),

            // Skipping until in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CameraSegmentProducerClaimRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CameraSegmentProducerClaimRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub timeout_seconds: Vec<f64>,
            pub until: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CameraSegmentProducerClaimRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "timeout_seconds" => intermediate_rep.timeout_seconds.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "until" => intermediate_rep.until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CameraSegmentProducerClaimRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CameraSegmentProducerClaimRequest {
            timeout_seconds: intermediate_rep.timeout_seconds.into_iter().next(),
            until: intermediate_rep.until.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CameraSegmentProducerClaimRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CameraSegmentProducerClaimRequest>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CameraSegmentProducerClaimRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for CameraSegmentProducerClaimRequest - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CameraSegmentProducerClaimRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CameraSegmentProducerClaimRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into CameraSegmentProducerClaimRequest - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CameraStreamProducerClaimRequest {
    #[serde(rename = "timeout_seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeout_seconds: Option<f64>,

    #[serde(rename = "until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub until: Option<chrono::DateTime::<chrono::Utc>>,

}



impl CameraStreamProducerClaimRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> CameraStreamProducerClaimRequest {
        CameraStreamProducerClaimRequest {
 timeout_seconds: None,
 until: None,
        }
    }
}

/// Converts the CameraStreamProducerClaimRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CameraStreamProducerClaimRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.timeout_seconds.as_ref().map(|timeout_seconds| {
                [
                    "timeout_seconds".to_string(),
                    timeout_seconds.to_string(),
                ].join(",")
            }),

            // Skipping until in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CameraStreamProducerClaimRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CameraStreamProducerClaimRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub timeout_seconds: Vec<f64>,
            pub until: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CameraStreamProducerClaimRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "timeout_seconds" => intermediate_rep.timeout_seconds.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "until" => intermediate_rep.until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CameraStreamProducerClaimRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CameraStreamProducerClaimRequest {
            timeout_seconds: intermediate_rep.timeout_seconds.into_iter().next(),
            until: intermediate_rep.until.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CameraStreamProducerClaimRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CameraStreamProducerClaimRequest>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CameraStreamProducerClaimRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for CameraStreamProducerClaimRequest - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CameraStreamProducerClaimRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CameraStreamProducerClaimRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into CameraStreamProducerClaimRequest - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Detection {
    #[serde(rename = "bounding_box")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bounding_box: Option<Vec<models::ArrayOfVec2Inner>>,

    #[serde(rename = "camera_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id: Option<uuid::Uuid>,

    #[serde(rename = "camera_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id_object: Option<models::Camera>,

    #[serde(rename = "centroid")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub centroid: Option<models::ArrayOfVec2Inner>,

    #[serde(rename = "class_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_id: Option<i64>,

    #[serde(rename = "class_name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_name: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<f64>,

    #[serde(rename = "seen_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seen_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "track_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub track_id: Option<uuid::Uuid>,

    #[serde(rename = "track_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub track_id_object: Option<models::Track>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "video_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id: Option<uuid::Uuid>,

    #[serde(rename = "video_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id_object: Option<models::Video>,

}



impl Detection {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Detection {
        Detection {
 bounding_box: None,
 camera_id: None,
 camera_id_object: None,
 centroid: None,
 class_id: None,
 class_name: None,
 created_at: None,
 deleted_at: None,
 id: None,
 score: None,
 seen_at: None,
 track_id: None,
 track_id_object: None,
 updated_at: None,
 video_id: None,
 video_id_object: None,
        }
    }
}

/// Converts the Detection value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Detection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping bounding_box in query parameter serialization

            // Skipping camera_id in query parameter serialization

            // Skipping camera_id_object in query parameter serialization

            // Skipping centroid in query parameter serialization


            self.class_id.as_ref().map(|class_id| {
                [
                    "class_id".to_string(),
                    class_id.to_string(),
                ].join(",")
            }),


            self.class_name.as_ref().map(|class_name| {
                [
                    "class_name".to_string(),
                    class_name.to_string(),
                ].join(",")
            }),

            // Skipping created_at in query parameter serialization

            // Skipping deleted_at in query parameter serialization

            // Skipping id in query parameter serialization


            self.score.as_ref().map(|score| {
                [
                    "score".to_string(),
                    score.to_string(),
                ].join(",")
            }),

            // Skipping seen_at in query parameter serialization

            // Skipping track_id in query parameter serialization

            // Skipping track_id_object in query parameter serialization

            // Skipping updated_at in query parameter serialization

            // Skipping video_id in query parameter serialization

            // Skipping video_id_object in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Detection value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Detection {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub bounding_box: Vec<Vec<models::ArrayOfVec2Inner>>,
            pub camera_id: Vec<uuid::Uuid>,
            pub camera_id_object: Vec<models::Camera>,
            pub centroid: Vec<models::ArrayOfVec2Inner>,
            pub class_id: Vec<i64>,
            pub class_name: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub id: Vec<uuid::Uuid>,
            pub score: Vec<f64>,
            pub seen_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub track_id: Vec<uuid::Uuid>,
            pub track_id_object: Vec<models::Track>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub video_id: Vec<uuid::Uuid>,
            pub video_id_object: Vec<models::Video>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Detection".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "bounding_box" => return std::result::Result::Err("Parsing a container in this style is not supported in Detection".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "camera_id" => intermediate_rep.camera_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id_object" => intermediate_rep.camera_id_object.push(<models::Camera as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "centroid" => intermediate_rep.centroid.push(<models::ArrayOfVec2Inner as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "class_id" => intermediate_rep.class_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "class_name" => intermediate_rep.class_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "seen_at" => intermediate_rep.seen_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "track_id" => intermediate_rep.track_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "track_id_object" => intermediate_rep.track_id_object.push(<models::Track as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id" => intermediate_rep.video_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id_object" => intermediate_rep.video_id_object.push(<models::Video as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Detection".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Detection {
            bounding_box: intermediate_rep.bounding_box.into_iter().next(),
            camera_id: intermediate_rep.camera_id.into_iter().next(),
            camera_id_object: intermediate_rep.camera_id_object.into_iter().next(),
            centroid: intermediate_rep.centroid.into_iter().next(),
            class_id: intermediate_rep.class_id.into_iter().next(),
            class_name: intermediate_rep.class_name.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            score: intermediate_rep.score.into_iter().next(),
            seen_at: intermediate_rep.seen_at.into_iter().next(),
            track_id: intermediate_rep.track_id.into_iter().next(),
            track_id_object: intermediate_rep.track_id_object.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            video_id: intermediate_rep.video_id.into_iter().next(),
            video_id_object: intermediate_rep.video_id_object.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Detection> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Detection>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Detection>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Detection - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Detection> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Detection as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Detection - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NullableAudioEvent {
    #[serde(rename = "camera_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id: Option<uuid::Uuid>,

    #[serde(rename = "camera_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id_object: Option<models::Camera>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "crest_db")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crest_db: Option<f64>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "duration_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration_ms: Option<i64>,

    #[serde(rename = "ended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ended_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "floor_dbfs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub floor_dbfs: Option<f64>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "kind")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kind: Option<String>,

    #[serde(rename = "offset_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset_ms: Option<i64>,

    #[serde(rename = "peak_dbfs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub peak_dbfs: Option<f64>,

    #[serde(rename = "prominence_db")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prominence_db: Option<f64>,

    #[serde(rename = "rms_dbfs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rms_dbfs: Option<f64>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<f64>,

    #[serde(rename = "seen_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seen_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "spectral")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub spectral: Option<crate::types::Object>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "video_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id: Option<uuid::Uuid>,

    #[serde(rename = "video_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id_object: Option<models::Video>,

}



impl NullableAudioEvent {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> NullableAudioEvent {
        NullableAudioEvent {
 camera_id: None,
 camera_id_object: None,
 created_at: None,
 crest_db: None,
 deleted_at: None,
 duration_ms: None,
 ended_at: None,
 floor_dbfs: None,
 id: None,
 kind: None,
 offset_ms: None,
 peak_dbfs: None,
 prominence_db: None,
 rms_dbfs: None,
 score: None,
 seen_at: None,
 spectral: None,
 updated_at: None,
 video_id: None,
 video_id_object: None,
        }
    }
}

/// Converts the NullableAudioEvent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NullableAudioEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping camera_id in query parameter serialization

            // Skipping camera_id_object in query parameter serialization

            // Skipping created_at in query parameter serialization


            self.crest_db.as_ref().map(|crest_db| {
                [
                    "crest_db".to_string(),
                    crest_db.to_string(),
                ].join(",")
            }),

            // Skipping deleted_at in query parameter serialization


            self.duration_ms.as_ref().map(|duration_ms| {
                [
                    "duration_ms".to_string(),
                    duration_ms.to_string(),
                ].join(",")
            }),

            // Skipping ended_at in query parameter serialization


            self.floor_dbfs.as_ref().map(|floor_dbfs| {
                [
                    "floor_dbfs".to_string(),
                    floor_dbfs.to_string(),
                ].join(",")
            }),

            // Skipping id in query parameter serialization


            self.kind.as_ref().map(|kind| {
                [
                    "kind".to_string(),
                    kind.to_string(),
                ].join(",")
            }),


            self.offset_ms.as_ref().map(|offset_ms| {
                [
                    "offset_ms".to_string(),
                    offset_ms.to_string(),
                ].join(",")
            }),


            self.peak_dbfs.as_ref().map(|peak_dbfs| {
                [
                    "peak_dbfs".to_string(),
                    peak_dbfs.to_string(),
                ].join(",")
            }),


            self.prominence_db.as_ref().map(|prominence_db| {
                [
                    "prominence_db".to_string(),
                    prominence_db.to_string(),
                ].join(",")
            }),


            self.rms_dbfs.as_ref().map(|rms_dbfs| {
                [
                    "rms_dbfs".to_string(),
                    rms_dbfs.to_string(),
                ].join(",")
            }),


            self.score.as_ref().map(|score| {
                [
                    "score".to_string(),
                    score.to_string(),
                ].join(",")
            }),

            // Skipping seen_at in query parameter serialization

            // Skipping spectral in query parameter serialization

            // Skipping updated_at in query parameter serialization

            // Skipping video_id in query parameter serialization

            // Skipping video_id_object in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NullableAudioEvent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NullableAudioEvent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub camera_id: Vec<uuid::Uuid>,
            pub camera_id_object: Vec<models::Camera>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub crest_db: Vec<f64>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub duration_ms: Vec<i64>,
            pub ended_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub floor_dbfs: Vec<f64>,
            pub id: Vec<uuid::Uuid>,
            pub kind: Vec<String>,
            pub offset_ms: Vec<i64>,
            pub peak_dbfs: Vec<f64>,
            pub prominence_db: Vec<f64>,
            pub rms_dbfs: Vec<f64>,
            pub score: Vec<f64>,
            pub seen_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub spectral: Vec<crate::types::Object>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub video_id: Vec<uuid::Uuid>,
            pub video_id_object: Vec<models::Video>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NullableAudioEvent".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "camera_id" => intermediate_rep.camera_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id_object" => intermediate_rep.camera_id_object.push(<models::Camera as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "crest_db" => intermediate_rep.crest_db.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "duration_ms" => intermediate_rep.duration_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ended_at" => intermediate_rep.ended_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "floor_dbfs" => intermediate_rep.floor_dbfs.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "offset_ms" => intermediate_rep.offset_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "peak_dbfs" => intermediate_rep.peak_dbfs.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prominence_db" => intermediate_rep.prominence_db.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "rms_dbfs" => intermediate_rep.rms_dbfs.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "seen_at" => intermediate_rep.seen_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spectral" => intermediate_rep.spectral.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id" => intermediate_rep.video_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id_object" => intermediate_rep.video_id_object.push(<models::Video as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NullableAudioEvent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NullableAudioEvent {
            camera_id: intermediate_rep.camera_id.into_iter().next(),
            camera_id_object: intermediate_rep.camera_id_object.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            crest_db: intermediate_rep.crest_db.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            duration_ms: intermediate_rep.duration_ms.into_iter().next(),
            ended_at: intermediate_rep.ended_at.into_iter().next(),
            floor_dbfs: intermediate_rep.floor_dbfs.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            kind: intermediate_rep.kind.into_iter().next(),
            offset_ms: intermediate_rep.offset_ms.into_iter().next(),
            peak_dbfs: intermediate_rep.peak_dbfs.into_iter().next(),
            prominence_db: intermediate_rep.prominence_db.into_iter().next(),
            rms_dbfs: intermediate_rep.rms_dbfs.into_iter().next(),
            score: intermediate_rep.score.into_iter().next(),
            seen_at: intermediate_rep.seen_at.into_iter().next(),
            spectral: intermediate_rep.spectral.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            video_id: intermediate_rep.video_id.into_iter().next(),
            video_id_object: intermediate_rep.video_id_object.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NullableAudioEvent> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NullableAudioEvent>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NullableAudioEvent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NullableAudioEvent - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NullableAudioEvent> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NullableAudioEvent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NullableAudioEvent - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NullableCamera {
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "last_seen")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_seen: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "low_res_stream_url")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub low_res_stream_url: Option<String>,

    #[serde(rename = "name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "overlay_offset_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub overlay_offset_ms: Option<i64>,

    #[serde(rename = "referenced_by_audio_event_camera_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_audio_event_camera_id_objects: Option<Vec<models::AudioEvent>>,

    #[serde(rename = "referenced_by_detection_camera_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_detection_camera_id_objects: Option<Vec<models::Detection>>,

    #[serde(rename = "referenced_by_track_camera_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_track_camera_id_objects: Option<Vec<models::Track>>,

    #[serde(rename = "referenced_by_video_camera_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_video_camera_id_objects: Option<Vec<models::Video>>,

    #[serde(rename = "segment_producer_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub segment_producer_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "stream_producer_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stream_producer_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "stream_url")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stream_url: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

}



impl NullableCamera {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> NullableCamera {
        NullableCamera {
 created_at: None,
 deleted_at: None,
 id: None,
 last_seen: None,
 low_res_stream_url: None,
 name: None,
 overlay_offset_ms: None,
 referenced_by_audio_event_camera_id_objects: None,
 referenced_by_detection_camera_id_objects: None,
 referenced_by_track_camera_id_objects: None,
 referenced_by_video_camera_id_objects: None,
 segment_producer_claimed_until: None,
 stream_producer_claimed_until: None,
 stream_url: None,
 updated_at: None,
        }
    }
}

/// Converts the NullableCamera value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NullableCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping created_at in query parameter serialization

            // Skipping deleted_at in query parameter serialization

            // Skipping id in query parameter serialization

            // Skipping last_seen in query parameter serialization


            self.low_res_stream_url.as_ref().map(|low_res_stream_url| {
                [
                    "low_res_stream_url".to_string(),
                    low_res_stream_url.to_string(),
                ].join(",")
            }),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.overlay_offset_ms.as_ref().map(|overlay_offset_ms| {
                [
                    "overlay_offset_ms".to_string(),
                    overlay_offset_ms.to_string(),
                ].join(",")
            }),

            // Skipping referenced_by_audio_event_camera_id_objects in query parameter serialization

            // Skipping referenced_by_detection_camera_id_objects in query parameter serialization

            // Skipping referenced_by_track_camera_id_objects in query parameter serialization

            // Skipping referenced_by_video_camera_id_objects in query parameter serialization

            // Skipping segment_producer_claimed_until in query parameter serialization

            // Skipping stream_producer_claimed_until in query parameter serialization


            self.stream_url.as_ref().map(|stream_url| {
                [
                    "stream_url".to_string(),
                    stream_url.to_string(),
                ].join(",")
            }),

            // Skipping updated_at in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NullableCamera value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NullableCamera {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub id: Vec<uuid::Uuid>,
            pub last_seen: Vec<chrono::DateTime::<chrono::Utc>>,
            pub low_res_stream_url: Vec<String>,
            pub name: Vec<String>,
            pub overlay_offset_ms: Vec<i64>,
            pub referenced_by_audio_event_camera_id_objects: Vec<Vec<models::AudioEvent>>,
            pub referenced_by_detection_camera_id_objects: Vec<Vec<models::Detection>>,
            pub referenced_by_track_camera_id_objects: Vec<Vec<models::Track>>,
            pub referenced_by_video_camera_id_objects: Vec<Vec<models::Video>>,
            pub segment_producer_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub stream_producer_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub stream_url: Vec<String>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NullableCamera".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last_seen" => intermediate_rep.last_seen.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "low_res_stream_url" => intermediate_rep.low_res_stream_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "overlay_offset_ms" => intermediate_rep.overlay_offset_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "referenced_by_audio_event_camera_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableCamera".to_string()),
                    "referenced_by_detection_camera_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableCamera".to_string()),
                    "referenced_by_track_camera_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableCamera".to_string()),
                    "referenced_by_video_camera_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableCamera".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "segment_producer_claimed_until" => intermediate_rep.segment_producer_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "stream_producer_claimed_until" => intermediate_rep.stream_producer_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "stream_url" => intermediate_rep.stream_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NullableCamera".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NullableCamera {
            created_at: intermediate_rep.created_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            last_seen: intermediate_rep.last_seen.into_iter().next(),
            low_res_stream_url: intermediate_rep.low_res_stream_url.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            overlay_offset_ms: intermediate_rep.overlay_offset_ms.into_iter().next(),
            referenced_by_audio_event_camera_id_objects: intermediate_rep.referenced_by_audio_event_camera_id_objects.into_iter().next(),
            referenced_by_detection_camera_id_objects: intermediate_rep.referenced_by_detection_camera_id_objects.into_iter().next(),
            referenced_by_track_camera_id_objects: intermediate_rep.referenced_by_track_camera_id_objects.into_iter().next(),
            referenced_by_video_camera_id_objects: intermediate_rep.referenced_by_video_camera_id_objects.into_iter().next(),
            segment_producer_claimed_until: intermediate_rep.segment_producer_claimed_until.into_iter().next(),
            stream_producer_claimed_until: intermediate_rep.stream_producer_claimed_until.into_iter().next(),
            stream_url: intermediate_rep.stream_url.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NullableCamera> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NullableCamera>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NullableCamera>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NullableCamera - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NullableCamera> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NullableCamera as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NullableCamera - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NullableDetection {
    #[serde(rename = "bounding_box")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bounding_box: Option<Vec<models::ArrayOfVec2Inner>>,

    #[serde(rename = "camera_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id: Option<uuid::Uuid>,

    #[serde(rename = "camera_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id_object: Option<models::Camera>,

    #[serde(rename = "centroid")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub centroid: Option<models::ArrayOfVec2Inner>,

    #[serde(rename = "class_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_id: Option<i64>,

    #[serde(rename = "class_name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_name: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<f64>,

    #[serde(rename = "seen_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seen_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "track_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub track_id: Option<uuid::Uuid>,

    #[serde(rename = "track_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub track_id_object: Option<models::Track>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "video_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id: Option<uuid::Uuid>,

    #[serde(rename = "video_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id_object: Option<models::Video>,

}



impl NullableDetection {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> NullableDetection {
        NullableDetection {
 bounding_box: None,
 camera_id: None,
 camera_id_object: None,
 centroid: None,
 class_id: None,
 class_name: None,
 created_at: None,
 deleted_at: None,
 id: None,
 score: None,
 seen_at: None,
 track_id: None,
 track_id_object: None,
 updated_at: None,
 video_id: None,
 video_id_object: None,
        }
    }
}

/// Converts the NullableDetection value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NullableDetection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping bounding_box in query parameter serialization

            // Skipping camera_id in query parameter serialization

            // Skipping camera_id_object in query parameter serialization

            // Skipping centroid in query parameter serialization


            self.class_id.as_ref().map(|class_id| {
                [
                    "class_id".to_string(),
                    class_id.to_string(),
                ].join(",")
            }),


            self.class_name.as_ref().map(|class_name| {
                [
                    "class_name".to_string(),
                    class_name.to_string(),
                ].join(",")
            }),

            // Skipping created_at in query parameter serialization

            // Skipping deleted_at in query parameter serialization

            // Skipping id in query parameter serialization


            self.score.as_ref().map(|score| {
                [
                    "score".to_string(),
                    score.to_string(),
                ].join(",")
            }),

            // Skipping seen_at in query parameter serialization

            // Skipping track_id in query parameter serialization

            // Skipping track_id_object in query parameter serialization

            // Skipping updated_at in query parameter serialization

            // Skipping video_id in query parameter serialization

            // Skipping video_id_object in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NullableDetection value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NullableDetection {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub bounding_box: Vec<Vec<models::ArrayOfVec2Inner>>,
            pub camera_id: Vec<uuid::Uuid>,
            pub camera_id_object: Vec<models::Camera>,
            pub centroid: Vec<models::ArrayOfVec2Inner>,
            pub class_id: Vec<i64>,
            pub class_name: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub id: Vec<uuid::Uuid>,
            pub score: Vec<f64>,
            pub seen_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub track_id: Vec<uuid::Uuid>,
            pub track_id_object: Vec<models::Track>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub video_id: Vec<uuid::Uuid>,
            pub video_id_object: Vec<models::Video>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NullableDetection".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "bounding_box" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableDetection".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "camera_id" => intermediate_rep.camera_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id_object" => intermediate_rep.camera_id_object.push(<models::Camera as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "centroid" => intermediate_rep.centroid.push(<models::ArrayOfVec2Inner as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "class_id" => intermediate_rep.class_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "class_name" => intermediate_rep.class_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "seen_at" => intermediate_rep.seen_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "track_id" => intermediate_rep.track_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "track_id_object" => intermediate_rep.track_id_object.push(<models::Track as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id" => intermediate_rep.video_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id_object" => intermediate_rep.video_id_object.push(<models::Video as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NullableDetection".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NullableDetection {
            bounding_box: intermediate_rep.bounding_box.into_iter().next(),
            camera_id: intermediate_rep.camera_id.into_iter().next(),
            camera_id_object: intermediate_rep.camera_id_object.into_iter().next(),
            centroid: intermediate_rep.centroid.into_iter().next(),
            class_id: intermediate_rep.class_id.into_iter().next(),
            class_name: intermediate_rep.class_name.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            score: intermediate_rep.score.into_iter().next(),
            seen_at: intermediate_rep.seen_at.into_iter().next(),
            track_id: intermediate_rep.track_id.into_iter().next(),
            track_id_object: intermediate_rep.track_id_object.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            video_id: intermediate_rep.video_id.into_iter().next(),
            video_id_object: intermediate_rep.video_id_object.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NullableDetection> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NullableDetection>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NullableDetection>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NullableDetection - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NullableDetection> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NullableDetection as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NullableDetection - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NullableTrack {
    #[serde(rename = "bounding_box")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bounding_box: Option<Vec<models::ArrayOfVec2Inner>>,

    #[serde(rename = "camera_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id: Option<uuid::Uuid>,

    #[serde(rename = "camera_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id_object: Option<models::Camera>,

    #[serde(rename = "centroid")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub centroid: Option<models::ArrayOfVec2Inner>,

    #[serde(rename = "class_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_id: Option<i64>,

    #[serde(rename = "class_name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_name: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "ended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ended_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "frame_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub frame_count: Option<i64>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "is_static")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_static: Option<bool>,

    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<uuid::Uuid>,

    #[serde(rename = "max_score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_score: Option<f64>,

    #[serde(rename = "mean_score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mean_score: Option<f64>,

    #[serde(rename = "referenced_by_detection_track_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_detection_track_id_objects: Option<Vec<models::Detection>>,

    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "status")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "trajectory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trajectory: Option<crate::types::Object>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "video_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id: Option<uuid::Uuid>,

    #[serde(rename = "video_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id_object: Option<models::Video>,

}



impl NullableTrack {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> NullableTrack {
        NullableTrack {
 bounding_box: None,
 camera_id: None,
 camera_id_object: None,
 centroid: None,
 class_id: None,
 class_name: None,
 created_at: None,
 deleted_at: None,
 ended_at: None,
 frame_count: None,
 id: None,
 is_static: None,
 key: None,
 max_score: None,
 mean_score: None,
 referenced_by_detection_track_id_objects: None,
 started_at: None,
 status: None,
 trajectory: None,
 updated_at: None,
 video_id: None,
 video_id_object: None,
        }
    }
}

/// Converts the NullableTrack value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NullableTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping bounding_box in query parameter serialization

            // Skipping camera_id in query parameter serialization

            // Skipping camera_id_object in query parameter serialization

            // Skipping centroid in query parameter serialization


            self.class_id.as_ref().map(|class_id| {
                [
                    "class_id".to_string(),
                    class_id.to_string(),
                ].join(",")
            }),


            self.class_name.as_ref().map(|class_name| {
                [
                    "class_name".to_string(),
                    class_name.to_string(),
                ].join(",")
            }),

            // Skipping created_at in query parameter serialization

            // Skipping deleted_at in query parameter serialization

            // Skipping ended_at in query parameter serialization


            self.frame_count.as_ref().map(|frame_count| {
                [
                    "frame_count".to_string(),
                    frame_count.to_string(),
                ].join(",")
            }),

            // Skipping id in query parameter serialization


            self.is_static.as_ref().map(|is_static| {
                [
                    "is_static".to_string(),
                    is_static.to_string(),
                ].join(",")
            }),

            // Skipping key in query parameter serialization


            self.max_score.as_ref().map(|max_score| {
                [
                    "max_score".to_string(),
                    max_score.to_string(),
                ].join(",")
            }),


            self.mean_score.as_ref().map(|mean_score| {
                [
                    "mean_score".to_string(),
                    mean_score.to_string(),
                ].join(",")
            }),

            // Skipping referenced_by_detection_track_id_objects in query parameter serialization

            // Skipping started_at in query parameter serialization


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),

            // Skipping trajectory in query parameter serialization

            // Skipping updated_at in query parameter serialization

            // Skipping video_id in query parameter serialization

            // Skipping video_id_object in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NullableTrack value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NullableTrack {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub bounding_box: Vec<Vec<models::ArrayOfVec2Inner>>,
            pub camera_id: Vec<uuid::Uuid>,
            pub camera_id_object: Vec<models::Camera>,
            pub centroid: Vec<models::ArrayOfVec2Inner>,
            pub class_id: Vec<i64>,
            pub class_name: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub ended_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub frame_count: Vec<i64>,
            pub id: Vec<uuid::Uuid>,
            pub is_static: Vec<bool>,
            pub key: Vec<uuid::Uuid>,
            pub max_score: Vec<f64>,
            pub mean_score: Vec<f64>,
            pub referenced_by_detection_track_id_objects: Vec<Vec<models::Detection>>,
            pub started_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub status: Vec<String>,
            pub trajectory: Vec<crate::types::Object>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub video_id: Vec<uuid::Uuid>,
            pub video_id_object: Vec<models::Video>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NullableTrack".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "bounding_box" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableTrack".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "camera_id" => intermediate_rep.camera_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id_object" => intermediate_rep.camera_id_object.push(<models::Camera as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "centroid" => intermediate_rep.centroid.push(<models::ArrayOfVec2Inner as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "class_id" => intermediate_rep.class_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "class_name" => intermediate_rep.class_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ended_at" => intermediate_rep.ended_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "frame_count" => intermediate_rep.frame_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_static" => intermediate_rep.is_static.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max_score" => intermediate_rep.max_score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "mean_score" => intermediate_rep.mean_score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "referenced_by_detection_track_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableTrack".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "started_at" => intermediate_rep.started_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "trajectory" => intermediate_rep.trajectory.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id" => intermediate_rep.video_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id_object" => intermediate_rep.video_id_object.push(<models::Video as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NullableTrack".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NullableTrack {
            bounding_box: intermediate_rep.bounding_box.into_iter().next(),
            camera_id: intermediate_rep.camera_id.into_iter().next(),
            camera_id_object: intermediate_rep.camera_id_object.into_iter().next(),
            centroid: intermediate_rep.centroid.into_iter().next(),
            class_id: intermediate_rep.class_id.into_iter().next(),
            class_name: intermediate_rep.class_name.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            ended_at: intermediate_rep.ended_at.into_iter().next(),
            frame_count: intermediate_rep.frame_count.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            is_static: intermediate_rep.is_static.into_iter().next(),
            key: intermediate_rep.key.into_iter().next(),
            max_score: intermediate_rep.max_score.into_iter().next(),
            mean_score: intermediate_rep.mean_score.into_iter().next(),
            referenced_by_detection_track_id_objects: intermediate_rep.referenced_by_detection_track_id_objects.into_iter().next(),
            started_at: intermediate_rep.started_at.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            trajectory: intermediate_rep.trajectory.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            video_id: intermediate_rep.video_id.into_iter().next(),
            video_id_object: intermediate_rep.video_id_object.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NullableTrack> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NullableTrack>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NullableTrack>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NullableTrack - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NullableTrack> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NullableTrack as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NullableTrack - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NullableVideo {
    #[serde(rename = "audio_detected_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_detected_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "audio_detector_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_detector_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "audio_profile")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_profile: Option<crate::types::Object>,

    #[serde(rename = "audio_summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_summary: Option<crate::types::Object>,

    #[serde(rename = "camera_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id: Option<uuid::Uuid>,

    #[serde(rename = "camera_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id_object: Option<models::Camera>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "detection_summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detection_summary: Option<crate::types::Object>,

    #[serde(rename = "duration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration: Option<i64>,

    #[serde(rename = "ended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ended_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "file_name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_name: Option<String>,

    #[serde(rename = "file_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_size: Option<f64>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "is_low_res")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_low_res: Option<bool>,

    #[serde(rename = "object_detector_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_detector_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "object_tracker_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_tracker_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "overlay_offset_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub overlay_offset_ms: Option<i64>,

    #[serde(rename = "referenced_by_audio_event_video_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_audio_event_video_id_objects: Option<Vec<models::AudioEvent>>,

    #[serde(rename = "referenced_by_detection_video_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_detection_video_id_objects: Option<Vec<models::Detection>>,

    #[serde(rename = "referenced_by_track_video_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_track_video_id_objects: Option<Vec<models::Track>>,

    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "status")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "thumbnail_name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumbnail_name: Option<String>,

    #[serde(rename = "track_summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub track_summary: Option<crate::types::Object>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

}



impl NullableVideo {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> NullableVideo {
        NullableVideo {
 audio_detected_at: None,
 audio_detector_claimed_until: None,
 audio_profile: None,
 audio_summary: None,
 camera_id: None,
 camera_id_object: None,
 created_at: None,
 deleted_at: None,
 detection_summary: None,
 duration: None,
 ended_at: None,
 file_name: None,
 file_size: None,
 id: None,
 is_low_res: None,
 object_detector_claimed_until: None,
 object_tracker_claimed_until: None,
 overlay_offset_ms: None,
 referenced_by_audio_event_video_id_objects: None,
 referenced_by_detection_video_id_objects: None,
 referenced_by_track_video_id_objects: None,
 started_at: None,
 status: None,
 thumbnail_name: None,
 track_summary: None,
 updated_at: None,
        }
    }
}

/// Converts the NullableVideo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NullableVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping audio_detected_at in query parameter serialization

            // Skipping audio_detector_claimed_until in query parameter serialization

            // Skipping audio_profile in query parameter serialization

            // Skipping audio_summary in query parameter serialization

            // Skipping camera_id in query parameter serialization

            // Skipping camera_id_object in query parameter serialization

            // Skipping created_at in query parameter serialization

            // Skipping deleted_at in query parameter serialization

            // Skipping detection_summary in query parameter serialization


            self.duration.as_ref().map(|duration| {
                [
                    "duration".to_string(),
                    duration.to_string(),
                ].join(",")
            }),

            // Skipping ended_at in query parameter serialization


            self.file_name.as_ref().map(|file_name| {
                [
                    "file_name".to_string(),
                    file_name.to_string(),
                ].join(",")
            }),


            self.file_size.as_ref().map(|file_size| {
                [
                    "file_size".to_string(),
                    file_size.to_string(),
                ].join(",")
            }),

            // Skipping id in query parameter serialization


            self.is_low_res.as_ref().map(|is_low_res| {
                [
                    "is_low_res".to_string(),
                    is_low_res.to_string(),
                ].join(",")
            }),

            // Skipping object_detector_claimed_until in query parameter serialization

            // Skipping object_tracker_claimed_until in query parameter serialization


            self.overlay_offset_ms.as_ref().map(|overlay_offset_ms| {
                [
                    "overlay_offset_ms".to_string(),
                    overlay_offset_ms.to_string(),
                ].join(",")
            }),

            // Skipping referenced_by_audio_event_video_id_objects in query parameter serialization

            // Skipping referenced_by_detection_video_id_objects in query parameter serialization

            // Skipping referenced_by_track_video_id_objects in query parameter serialization

            // Skipping started_at in query parameter serialization


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.thumbnail_name.as_ref().map(|thumbnail_name| {
                [
                    "thumbnail_name".to_string(),
                    thumbnail_name.to_string(),
                ].join(",")
            }),

            // Skipping track_summary in query parameter serialization

            // Skipping updated_at in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NullableVideo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NullableVideo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub audio_detected_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub audio_detector_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub audio_profile: Vec<crate::types::Object>,
            pub audio_summary: Vec<crate::types::Object>,
            pub camera_id: Vec<uuid::Uuid>,
            pub camera_id_object: Vec<models::Camera>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub detection_summary: Vec<crate::types::Object>,
            pub duration: Vec<i64>,
            pub ended_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub file_name: Vec<String>,
            pub file_size: Vec<f64>,
            pub id: Vec<uuid::Uuid>,
            pub is_low_res: Vec<bool>,
            pub object_detector_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub object_tracker_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub overlay_offset_ms: Vec<i64>,
            pub referenced_by_audio_event_video_id_objects: Vec<Vec<models::AudioEvent>>,
            pub referenced_by_detection_video_id_objects: Vec<Vec<models::Detection>>,
            pub referenced_by_track_video_id_objects: Vec<Vec<models::Track>>,
            pub started_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub status: Vec<String>,
            pub thumbnail_name: Vec<String>,
            pub track_summary: Vec<crate::types::Object>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NullableVideo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "audio_detected_at" => intermediate_rep.audio_detected_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "audio_detector_claimed_until" => intermediate_rep.audio_detector_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "audio_profile" => intermediate_rep.audio_profile.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "audio_summary" => intermediate_rep.audio_summary.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id" => intermediate_rep.camera_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id_object" => intermediate_rep.camera_id_object.push(<models::Camera as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "detection_summary" => intermediate_rep.detection_summary.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "duration" => intermediate_rep.duration.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ended_at" => intermediate_rep.ended_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file_name" => intermediate_rep.file_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file_size" => intermediate_rep.file_size.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_low_res" => intermediate_rep.is_low_res.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "object_detector_claimed_until" => intermediate_rep.object_detector_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "object_tracker_claimed_until" => intermediate_rep.object_tracker_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "overlay_offset_ms" => intermediate_rep.overlay_offset_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "referenced_by_audio_event_video_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableVideo".to_string()),
                    "referenced_by_detection_video_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableVideo".to_string()),
                    "referenced_by_track_video_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in NullableVideo".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "started_at" => intermediate_rep.started_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "thumbnail_name" => intermediate_rep.thumbnail_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "track_summary" => intermediate_rep.track_summary.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NullableVideo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NullableVideo {
            audio_detected_at: intermediate_rep.audio_detected_at.into_iter().next(),
            audio_detector_claimed_until: intermediate_rep.audio_detector_claimed_until.into_iter().next(),
            audio_profile: intermediate_rep.audio_profile.into_iter().next(),
            audio_summary: intermediate_rep.audio_summary.into_iter().next(),
            camera_id: intermediate_rep.camera_id.into_iter().next(),
            camera_id_object: intermediate_rep.camera_id_object.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            detection_summary: intermediate_rep.detection_summary.into_iter().next(),
            duration: intermediate_rep.duration.into_iter().next(),
            ended_at: intermediate_rep.ended_at.into_iter().next(),
            file_name: intermediate_rep.file_name.into_iter().next(),
            file_size: intermediate_rep.file_size.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            is_low_res: intermediate_rep.is_low_res.into_iter().next(),
            object_detector_claimed_until: intermediate_rep.object_detector_claimed_until.into_iter().next(),
            object_tracker_claimed_until: intermediate_rep.object_tracker_claimed_until.into_iter().next(),
            overlay_offset_ms: intermediate_rep.overlay_offset_ms.into_iter().next(),
            referenced_by_audio_event_video_id_objects: intermediate_rep.referenced_by_audio_event_video_id_objects.into_iter().next(),
            referenced_by_detection_video_id_objects: intermediate_rep.referenced_by_detection_video_id_objects.into_iter().next(),
            referenced_by_track_video_id_objects: intermediate_rep.referenced_by_track_video_id_objects.into_iter().next(),
            started_at: intermediate_rep.started_at.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            thumbnail_name: intermediate_rep.thumbnail_name.into_iter().next(),
            track_summary: intermediate_rep.track_summary.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NullableVideo> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NullableVideo>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NullableVideo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NullableVideo - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NullableVideo> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NullableVideo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NullableVideo - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostAudioDetectorClaimVideosDefaultResponse {
    #[serde(rename = "error")]
          #[validate(custom(function = "check_xss_vec_string"))]
    pub error: Vec<String>,

    #[serde(rename = "status")]
    pub status: i32,

    #[serde(rename = "success")]
    pub success: bool,

}



impl PostAudioDetectorClaimVideosDefaultResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(error: Vec<String>, status: i32, success: bool, ) -> PostAudioDetectorClaimVideosDefaultResponse {
        PostAudioDetectorClaimVideosDefaultResponse {
 error,
 status,
 success,
        }
    }
}

/// Converts the PostAudioDetectorClaimVideosDefaultResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PostAudioDetectorClaimVideosDefaultResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("error".to_string()),
            Some(self.error.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("status".to_string()),
            Some(self.status.to_string()),


            Some("success".to_string()),
            Some(self.success.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostAudioDetectorClaimVideosDefaultResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostAudioDetectorClaimVideosDefaultResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub error: Vec<Vec<String>>,
            pub status: Vec<i32>,
            pub success: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostAudioDetectorClaimVideosDefaultResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "error" => return std::result::Result::Err("Parsing a container in this style is not supported in PostAudioDetectorClaimVideosDefaultResponse".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostAudioDetectorClaimVideosDefaultResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostAudioDetectorClaimVideosDefaultResponse {
            error: intermediate_rep.error.into_iter().next().ok_or_else(|| "error missing in PostAudioDetectorClaimVideosDefaultResponse".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in PostAudioDetectorClaimVideosDefaultResponse".to_string())?,
            success: intermediate_rep.success.into_iter().next().ok_or_else(|| "success missing in PostAudioDetectorClaimVideosDefaultResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostAudioDetectorClaimVideosDefaultResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PostAudioDetectorClaimVideosDefaultResponse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostAudioDetectorClaimVideosDefaultResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for PostAudioDetectorClaimVideosDefaultResponse - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PostAudioDetectorClaimVideosDefaultResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostAudioDetectorClaimVideosDefaultResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into PostAudioDetectorClaimVideosDefaultResponse - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResponseWithGenericOfAudioEvent {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "error")]
          #[validate(custom(function = "check_xss_vec_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<Vec<String>>,

    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub objects: Option<Vec<models::AudioEvent>>,

    #[serde(rename = "offset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset: Option<i64>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i64>,

    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

}



impl ResponseWithGenericOfAudioEvent {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ResponseWithGenericOfAudioEvent {
        ResponseWithGenericOfAudioEvent {
 count: None,
 error: None,
 limit: None,
 objects: None,
 offset: None,
 status: None,
 success: None,
 total_count: None,
        }
    }
}

/// Converts the ResponseWithGenericOfAudioEvent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ResponseWithGenericOfAudioEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.count.as_ref().map(|count| {
                [
                    "count".to_string(),
                    count.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.limit.as_ref().map(|limit| {
                [
                    "limit".to_string(),
                    limit.to_string(),
                ].join(",")
            }),

            // Skipping objects in query parameter serialization


            self.offset.as_ref().map(|offset| {
                [
                    "offset".to_string(),
                    offset.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),


            self.total_count.as_ref().map(|total_count| {
                [
                    "total_count".to_string(),
                    total_count.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResponseWithGenericOfAudioEvent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResponseWithGenericOfAudioEvent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub count: Vec<i64>,
            pub error: Vec<Vec<String>>,
            pub limit: Vec<i64>,
            pub objects: Vec<Vec<models::AudioEvent>>,
            pub offset: Vec<i64>,
            pub status: Vec<i64>,
            pub success: Vec<bool>,
            pub total_count: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResponseWithGenericOfAudioEvent".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "error" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfAudioEvent".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "objects" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfAudioEvent".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "offset" => intermediate_rep.offset.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "total_count" => intermediate_rep.total_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResponseWithGenericOfAudioEvent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResponseWithGenericOfAudioEvent {
            count: intermediate_rep.count.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
            limit: intermediate_rep.limit.into_iter().next(),
            objects: intermediate_rep.objects.into_iter().next(),
            offset: intermediate_rep.offset.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            success: intermediate_rep.success.into_iter().next(),
            total_count: intermediate_rep.total_count.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResponseWithGenericOfAudioEvent> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ResponseWithGenericOfAudioEvent>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResponseWithGenericOfAudioEvent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ResponseWithGenericOfAudioEvent - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ResponseWithGenericOfAudioEvent> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResponseWithGenericOfAudioEvent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ResponseWithGenericOfAudioEvent - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResponseWithGenericOfCamera {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "error")]
          #[validate(custom(function = "check_xss_vec_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<Vec<String>>,

    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub objects: Option<Vec<models::Camera>>,

    #[serde(rename = "offset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset: Option<i64>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i64>,

    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

}



impl ResponseWithGenericOfCamera {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ResponseWithGenericOfCamera {
        ResponseWithGenericOfCamera {
 count: None,
 error: None,
 limit: None,
 objects: None,
 offset: None,
 status: None,
 success: None,
 total_count: None,
        }
    }
}

/// Converts the ResponseWithGenericOfCamera value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ResponseWithGenericOfCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.count.as_ref().map(|count| {
                [
                    "count".to_string(),
                    count.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.limit.as_ref().map(|limit| {
                [
                    "limit".to_string(),
                    limit.to_string(),
                ].join(",")
            }),

            // Skipping objects in query parameter serialization


            self.offset.as_ref().map(|offset| {
                [
                    "offset".to_string(),
                    offset.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),


            self.total_count.as_ref().map(|total_count| {
                [
                    "total_count".to_string(),
                    total_count.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResponseWithGenericOfCamera value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResponseWithGenericOfCamera {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub count: Vec<i64>,
            pub error: Vec<Vec<String>>,
            pub limit: Vec<i64>,
            pub objects: Vec<Vec<models::Camera>>,
            pub offset: Vec<i64>,
            pub status: Vec<i64>,
            pub success: Vec<bool>,
            pub total_count: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResponseWithGenericOfCamera".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "error" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfCamera".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "objects" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfCamera".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "offset" => intermediate_rep.offset.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "total_count" => intermediate_rep.total_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResponseWithGenericOfCamera".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResponseWithGenericOfCamera {
            count: intermediate_rep.count.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
            limit: intermediate_rep.limit.into_iter().next(),
            objects: intermediate_rep.objects.into_iter().next(),
            offset: intermediate_rep.offset.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            success: intermediate_rep.success.into_iter().next(),
            total_count: intermediate_rep.total_count.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResponseWithGenericOfCamera> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ResponseWithGenericOfCamera>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResponseWithGenericOfCamera>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ResponseWithGenericOfCamera - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ResponseWithGenericOfCamera> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResponseWithGenericOfCamera as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ResponseWithGenericOfCamera - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResponseWithGenericOfDetection {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "error")]
          #[validate(custom(function = "check_xss_vec_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<Vec<String>>,

    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub objects: Option<Vec<models::Detection>>,

    #[serde(rename = "offset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset: Option<i64>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i64>,

    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

}



impl ResponseWithGenericOfDetection {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ResponseWithGenericOfDetection {
        ResponseWithGenericOfDetection {
 count: None,
 error: None,
 limit: None,
 objects: None,
 offset: None,
 status: None,
 success: None,
 total_count: None,
        }
    }
}

/// Converts the ResponseWithGenericOfDetection value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ResponseWithGenericOfDetection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.count.as_ref().map(|count| {
                [
                    "count".to_string(),
                    count.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.limit.as_ref().map(|limit| {
                [
                    "limit".to_string(),
                    limit.to_string(),
                ].join(",")
            }),

            // Skipping objects in query parameter serialization


            self.offset.as_ref().map(|offset| {
                [
                    "offset".to_string(),
                    offset.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),


            self.total_count.as_ref().map(|total_count| {
                [
                    "total_count".to_string(),
                    total_count.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResponseWithGenericOfDetection value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResponseWithGenericOfDetection {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub count: Vec<i64>,
            pub error: Vec<Vec<String>>,
            pub limit: Vec<i64>,
            pub objects: Vec<Vec<models::Detection>>,
            pub offset: Vec<i64>,
            pub status: Vec<i64>,
            pub success: Vec<bool>,
            pub total_count: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResponseWithGenericOfDetection".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "error" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfDetection".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "objects" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfDetection".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "offset" => intermediate_rep.offset.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "total_count" => intermediate_rep.total_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResponseWithGenericOfDetection".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResponseWithGenericOfDetection {
            count: intermediate_rep.count.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
            limit: intermediate_rep.limit.into_iter().next(),
            objects: intermediate_rep.objects.into_iter().next(),
            offset: intermediate_rep.offset.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            success: intermediate_rep.success.into_iter().next(),
            total_count: intermediate_rep.total_count.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResponseWithGenericOfDetection> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ResponseWithGenericOfDetection>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResponseWithGenericOfDetection>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ResponseWithGenericOfDetection - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ResponseWithGenericOfDetection> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResponseWithGenericOfDetection as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ResponseWithGenericOfDetection - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResponseWithGenericOfTrack {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "error")]
          #[validate(custom(function = "check_xss_vec_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<Vec<String>>,

    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub objects: Option<Vec<models::Track>>,

    #[serde(rename = "offset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset: Option<i64>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i64>,

    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

}



impl ResponseWithGenericOfTrack {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ResponseWithGenericOfTrack {
        ResponseWithGenericOfTrack {
 count: None,
 error: None,
 limit: None,
 objects: None,
 offset: None,
 status: None,
 success: None,
 total_count: None,
        }
    }
}

/// Converts the ResponseWithGenericOfTrack value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ResponseWithGenericOfTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.count.as_ref().map(|count| {
                [
                    "count".to_string(),
                    count.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.limit.as_ref().map(|limit| {
                [
                    "limit".to_string(),
                    limit.to_string(),
                ].join(",")
            }),

            // Skipping objects in query parameter serialization


            self.offset.as_ref().map(|offset| {
                [
                    "offset".to_string(),
                    offset.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),


            self.total_count.as_ref().map(|total_count| {
                [
                    "total_count".to_string(),
                    total_count.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResponseWithGenericOfTrack value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResponseWithGenericOfTrack {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub count: Vec<i64>,
            pub error: Vec<Vec<String>>,
            pub limit: Vec<i64>,
            pub objects: Vec<Vec<models::Track>>,
            pub offset: Vec<i64>,
            pub status: Vec<i64>,
            pub success: Vec<bool>,
            pub total_count: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResponseWithGenericOfTrack".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "error" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfTrack".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "objects" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfTrack".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "offset" => intermediate_rep.offset.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "total_count" => intermediate_rep.total_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResponseWithGenericOfTrack".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResponseWithGenericOfTrack {
            count: intermediate_rep.count.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
            limit: intermediate_rep.limit.into_iter().next(),
            objects: intermediate_rep.objects.into_iter().next(),
            offset: intermediate_rep.offset.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            success: intermediate_rep.success.into_iter().next(),
            total_count: intermediate_rep.total_count.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResponseWithGenericOfTrack> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ResponseWithGenericOfTrack>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResponseWithGenericOfTrack>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ResponseWithGenericOfTrack - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ResponseWithGenericOfTrack> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResponseWithGenericOfTrack as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ResponseWithGenericOfTrack - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResponseWithGenericOfVideo {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "error")]
          #[validate(custom(function = "check_xss_vec_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<Vec<String>>,

    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub objects: Option<Vec<models::Video>>,

    #[serde(rename = "offset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset: Option<i64>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i64>,

    #[serde(rename = "success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

}



impl ResponseWithGenericOfVideo {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ResponseWithGenericOfVideo {
        ResponseWithGenericOfVideo {
 count: None,
 error: None,
 limit: None,
 objects: None,
 offset: None,
 status: None,
 success: None,
 total_count: None,
        }
    }
}

/// Converts the ResponseWithGenericOfVideo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ResponseWithGenericOfVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.count.as_ref().map(|count| {
                [
                    "count".to_string(),
                    count.to_string(),
                ].join(",")
            }),


            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.limit.as_ref().map(|limit| {
                [
                    "limit".to_string(),
                    limit.to_string(),
                ].join(",")
            }),

            // Skipping objects in query parameter serialization


            self.offset.as_ref().map(|offset| {
                [
                    "offset".to_string(),
                    offset.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.success.as_ref().map(|success| {
                [
                    "success".to_string(),
                    success.to_string(),
                ].join(",")
            }),


            self.total_count.as_ref().map(|total_count| {
                [
                    "total_count".to_string(),
                    total_count.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResponseWithGenericOfVideo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResponseWithGenericOfVideo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub count: Vec<i64>,
            pub error: Vec<Vec<String>>,
            pub limit: Vec<i64>,
            pub objects: Vec<Vec<models::Video>>,
            pub offset: Vec<i64>,
            pub status: Vec<i64>,
            pub success: Vec<bool>,
            pub total_count: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResponseWithGenericOfVideo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "error" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfVideo".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "objects" => return std::result::Result::Err("Parsing a container in this style is not supported in ResponseWithGenericOfVideo".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "offset" => intermediate_rep.offset.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "success" => intermediate_rep.success.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "total_count" => intermediate_rep.total_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResponseWithGenericOfVideo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResponseWithGenericOfVideo {
            count: intermediate_rep.count.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
            limit: intermediate_rep.limit.into_iter().next(),
            objects: intermediate_rep.objects.into_iter().next(),
            offset: intermediate_rep.offset.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            success: intermediate_rep.success.into_iter().next(),
            total_count: intermediate_rep.total_count.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResponseWithGenericOfVideo> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ResponseWithGenericOfVideo>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResponseWithGenericOfVideo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ResponseWithGenericOfVideo - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ResponseWithGenericOfVideo> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResponseWithGenericOfVideo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ResponseWithGenericOfVideo - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Track {
    #[serde(rename = "bounding_box")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bounding_box: Option<Vec<models::ArrayOfVec2Inner>>,

    #[serde(rename = "camera_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id: Option<uuid::Uuid>,

    #[serde(rename = "camera_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id_object: Option<models::Camera>,

    #[serde(rename = "centroid")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub centroid: Option<models::ArrayOfVec2Inner>,

    #[serde(rename = "class_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_id: Option<i64>,

    #[serde(rename = "class_name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_name: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "ended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ended_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "frame_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub frame_count: Option<i64>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "is_static")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_static: Option<bool>,

    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<uuid::Uuid>,

    #[serde(rename = "max_score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_score: Option<f64>,

    #[serde(rename = "mean_score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mean_score: Option<f64>,

    #[serde(rename = "referenced_by_detection_track_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_detection_track_id_objects: Option<Vec<models::Detection>>,

    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "status")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "trajectory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trajectory: Option<crate::types::Object>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "video_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id: Option<uuid::Uuid>,

    #[serde(rename = "video_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub video_id_object: Option<models::Video>,

}



impl Track {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Track {
        Track {
 bounding_box: None,
 camera_id: None,
 camera_id_object: None,
 centroid: None,
 class_id: None,
 class_name: None,
 created_at: None,
 deleted_at: None,
 ended_at: None,
 frame_count: None,
 id: None,
 is_static: None,
 key: None,
 max_score: None,
 mean_score: None,
 referenced_by_detection_track_id_objects: None,
 started_at: None,
 status: None,
 trajectory: None,
 updated_at: None,
 video_id: None,
 video_id_object: None,
        }
    }
}

/// Converts the Track value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping bounding_box in query parameter serialization

            // Skipping camera_id in query parameter serialization

            // Skipping camera_id_object in query parameter serialization

            // Skipping centroid in query parameter serialization


            self.class_id.as_ref().map(|class_id| {
                [
                    "class_id".to_string(),
                    class_id.to_string(),
                ].join(",")
            }),


            self.class_name.as_ref().map(|class_name| {
                [
                    "class_name".to_string(),
                    class_name.to_string(),
                ].join(",")
            }),

            // Skipping created_at in query parameter serialization

            // Skipping deleted_at in query parameter serialization

            // Skipping ended_at in query parameter serialization


            self.frame_count.as_ref().map(|frame_count| {
                [
                    "frame_count".to_string(),
                    frame_count.to_string(),
                ].join(",")
            }),

            // Skipping id in query parameter serialization


            self.is_static.as_ref().map(|is_static| {
                [
                    "is_static".to_string(),
                    is_static.to_string(),
                ].join(",")
            }),

            // Skipping key in query parameter serialization


            self.max_score.as_ref().map(|max_score| {
                [
                    "max_score".to_string(),
                    max_score.to_string(),
                ].join(",")
            }),


            self.mean_score.as_ref().map(|mean_score| {
                [
                    "mean_score".to_string(),
                    mean_score.to_string(),
                ].join(",")
            }),

            // Skipping referenced_by_detection_track_id_objects in query parameter serialization

            // Skipping started_at in query parameter serialization


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),

            // Skipping trajectory in query parameter serialization

            // Skipping updated_at in query parameter serialization

            // Skipping video_id in query parameter serialization

            // Skipping video_id_object in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Track value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Track {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub bounding_box: Vec<Vec<models::ArrayOfVec2Inner>>,
            pub camera_id: Vec<uuid::Uuid>,
            pub camera_id_object: Vec<models::Camera>,
            pub centroid: Vec<models::ArrayOfVec2Inner>,
            pub class_id: Vec<i64>,
            pub class_name: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub ended_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub frame_count: Vec<i64>,
            pub id: Vec<uuid::Uuid>,
            pub is_static: Vec<bool>,
            pub key: Vec<uuid::Uuid>,
            pub max_score: Vec<f64>,
            pub mean_score: Vec<f64>,
            pub referenced_by_detection_track_id_objects: Vec<Vec<models::Detection>>,
            pub started_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub status: Vec<String>,
            pub trajectory: Vec<crate::types::Object>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub video_id: Vec<uuid::Uuid>,
            pub video_id_object: Vec<models::Video>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Track".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "bounding_box" => return std::result::Result::Err("Parsing a container in this style is not supported in Track".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "camera_id" => intermediate_rep.camera_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id_object" => intermediate_rep.camera_id_object.push(<models::Camera as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "centroid" => intermediate_rep.centroid.push(<models::ArrayOfVec2Inner as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "class_id" => intermediate_rep.class_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "class_name" => intermediate_rep.class_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ended_at" => intermediate_rep.ended_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "frame_count" => intermediate_rep.frame_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_static" => intermediate_rep.is_static.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max_score" => intermediate_rep.max_score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "mean_score" => intermediate_rep.mean_score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "referenced_by_detection_track_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in Track".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "started_at" => intermediate_rep.started_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "trajectory" => intermediate_rep.trajectory.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id" => intermediate_rep.video_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "video_id_object" => intermediate_rep.video_id_object.push(<models::Video as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Track".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Track {
            bounding_box: intermediate_rep.bounding_box.into_iter().next(),
            camera_id: intermediate_rep.camera_id.into_iter().next(),
            camera_id_object: intermediate_rep.camera_id_object.into_iter().next(),
            centroid: intermediate_rep.centroid.into_iter().next(),
            class_id: intermediate_rep.class_id.into_iter().next(),
            class_name: intermediate_rep.class_name.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            ended_at: intermediate_rep.ended_at.into_iter().next(),
            frame_count: intermediate_rep.frame_count.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            is_static: intermediate_rep.is_static.into_iter().next(),
            key: intermediate_rep.key.into_iter().next(),
            max_score: intermediate_rep.max_score.into_iter().next(),
            mean_score: intermediate_rep.mean_score.into_iter().next(),
            referenced_by_detection_track_id_objects: intermediate_rep.referenced_by_detection_track_id_objects.into_iter().next(),
            started_at: intermediate_rep.started_at.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            trajectory: intermediate_rep.trajectory.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
            video_id: intermediate_rep.video_id.into_iter().next(),
            video_id_object: intermediate_rep.video_id_object.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Track> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Track>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Track>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Track - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Track> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Track as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Track - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Vec2 {
    #[serde(rename = "X")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub x: Option<f64>,

    #[serde(rename = "Y")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub y: Option<f64>,

}



impl Vec2 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Vec2 {
        Vec2 {
 x: None,
 y: None,
        }
    }
}

/// Converts the Vec2 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.x.as_ref().map(|x| {
                [
                    "X".to_string(),
                    x.to_string(),
                ].join(",")
            }),


            self.y.as_ref().map(|y| {
                [
                    "Y".to_string(),
                    y.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Vec2 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Vec2 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub x: Vec<f64>,
            pub y: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Vec2".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "X" => intermediate_rep.x.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "Y" => intermediate_rep.y.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Vec2".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Vec2 {
            x: intermediate_rep.x.into_iter().next(),
            y: intermediate_rep.y.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Vec2> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec2>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Vec2>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Vec2 - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Vec2> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Vec2 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Vec2 - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Video {
    #[serde(rename = "audio_detected_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_detected_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "audio_detector_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_detector_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "audio_profile")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_profile: Option<crate::types::Object>,

    #[serde(rename = "audio_summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub audio_summary: Option<crate::types::Object>,

    #[serde(rename = "camera_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id: Option<uuid::Uuid>,

    #[serde(rename = "camera_id_object")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub camera_id_object: Option<models::Camera>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "detection_summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detection_summary: Option<crate::types::Object>,

    #[serde(rename = "duration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration: Option<i64>,

    #[serde(rename = "ended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ended_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "file_name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_name: Option<String>,

    #[serde(rename = "file_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_size: Option<f64>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "is_low_res")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_low_res: Option<bool>,

    #[serde(rename = "object_detector_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_detector_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "object_tracker_claimed_until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_tracker_claimed_until: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "overlay_offset_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub overlay_offset_ms: Option<i64>,

    #[serde(rename = "referenced_by_audio_event_video_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_audio_event_video_id_objects: Option<Vec<models::AudioEvent>>,

    #[serde(rename = "referenced_by_detection_video_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_detection_video_id_objects: Option<Vec<models::Detection>>,

    #[serde(rename = "referenced_by_track_video_id_objects")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referenced_by_track_video_id_objects: Option<Vec<models::Track>>,

    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "status")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "thumbnail_name")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thumbnail_name: Option<String>,

    #[serde(rename = "track_summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub track_summary: Option<crate::types::Object>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime::<chrono::Utc>>,

}



impl Video {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Video {
        Video {
 audio_detected_at: None,
 audio_detector_claimed_until: None,
 audio_profile: None,
 audio_summary: None,
 camera_id: None,
 camera_id_object: None,
 created_at: None,
 deleted_at: None,
 detection_summary: None,
 duration: None,
 ended_at: None,
 file_name: None,
 file_size: None,
 id: None,
 is_low_res: None,
 object_detector_claimed_until: None,
 object_tracker_claimed_until: None,
 overlay_offset_ms: None,
 referenced_by_audio_event_video_id_objects: None,
 referenced_by_detection_video_id_objects: None,
 referenced_by_track_video_id_objects: None,
 started_at: None,
 status: None,
 thumbnail_name: None,
 track_summary: None,
 updated_at: None,
        }
    }
}

/// Converts the Video value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Video {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping audio_detected_at in query parameter serialization

            // Skipping audio_detector_claimed_until in query parameter serialization

            // Skipping audio_profile in query parameter serialization

            // Skipping audio_summary in query parameter serialization

            // Skipping camera_id in query parameter serialization

            // Skipping camera_id_object in query parameter serialization

            // Skipping created_at in query parameter serialization

            // Skipping deleted_at in query parameter serialization

            // Skipping detection_summary in query parameter serialization


            self.duration.as_ref().map(|duration| {
                [
                    "duration".to_string(),
                    duration.to_string(),
                ].join(",")
            }),

            // Skipping ended_at in query parameter serialization


            self.file_name.as_ref().map(|file_name| {
                [
                    "file_name".to_string(),
                    file_name.to_string(),
                ].join(",")
            }),


            self.file_size.as_ref().map(|file_size| {
                [
                    "file_size".to_string(),
                    file_size.to_string(),
                ].join(",")
            }),

            // Skipping id in query parameter serialization


            self.is_low_res.as_ref().map(|is_low_res| {
                [
                    "is_low_res".to_string(),
                    is_low_res.to_string(),
                ].join(",")
            }),

            // Skipping object_detector_claimed_until in query parameter serialization

            // Skipping object_tracker_claimed_until in query parameter serialization


            self.overlay_offset_ms.as_ref().map(|overlay_offset_ms| {
                [
                    "overlay_offset_ms".to_string(),
                    overlay_offset_ms.to_string(),
                ].join(",")
            }),

            // Skipping referenced_by_audio_event_video_id_objects in query parameter serialization

            // Skipping referenced_by_detection_video_id_objects in query parameter serialization

            // Skipping referenced_by_track_video_id_objects in query parameter serialization

            // Skipping started_at in query parameter serialization


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.thumbnail_name.as_ref().map(|thumbnail_name| {
                [
                    "thumbnail_name".to_string(),
                    thumbnail_name.to_string(),
                ].join(",")
            }),

            // Skipping track_summary in query parameter serialization

            // Skipping updated_at in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Video value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Video {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub audio_detected_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub audio_detector_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub audio_profile: Vec<crate::types::Object>,
            pub audio_summary: Vec<crate::types::Object>,
            pub camera_id: Vec<uuid::Uuid>,
            pub camera_id_object: Vec<models::Camera>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deleted_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub detection_summary: Vec<crate::types::Object>,
            pub duration: Vec<i64>,
            pub ended_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub file_name: Vec<String>,
            pub file_size: Vec<f64>,
            pub id: Vec<uuid::Uuid>,
            pub is_low_res: Vec<bool>,
            pub object_detector_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub object_tracker_claimed_until: Vec<chrono::DateTime::<chrono::Utc>>,
            pub overlay_offset_ms: Vec<i64>,
            pub referenced_by_audio_event_video_id_objects: Vec<Vec<models::AudioEvent>>,
            pub referenced_by_detection_video_id_objects: Vec<Vec<models::Detection>>,
            pub referenced_by_track_video_id_objects: Vec<Vec<models::Track>>,
            pub started_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub status: Vec<String>,
            pub thumbnail_name: Vec<String>,
            pub track_summary: Vec<crate::types::Object>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Video".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "audio_detected_at" => intermediate_rep.audio_detected_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "audio_detector_claimed_until" => intermediate_rep.audio_detector_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "audio_profile" => intermediate_rep.audio_profile.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "audio_summary" => intermediate_rep.audio_summary.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id" => intermediate_rep.camera_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "camera_id_object" => intermediate_rep.camera_id_object.push(<models::Camera as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "deleted_at" => intermediate_rep.deleted_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "detection_summary" => intermediate_rep.detection_summary.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "duration" => intermediate_rep.duration.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ended_at" => intermediate_rep.ended_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file_name" => intermediate_rep.file_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file_size" => intermediate_rep.file_size.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_low_res" => intermediate_rep.is_low_res.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "object_detector_claimed_until" => intermediate_rep.object_detector_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "object_tracker_claimed_until" => intermediate_rep.object_tracker_claimed_until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "overlay_offset_ms" => intermediate_rep.overlay_offset_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "referenced_by_audio_event_video_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in Video".to_string()),
                    "referenced_by_detection_video_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in Video".to_string()),
                    "referenced_by_track_video_id_objects" => return std::result::Result::Err("Parsing a container in this style is not supported in Video".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "started_at" => intermediate_rep.started_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "thumbnail_name" => intermediate_rep.thumbnail_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "track_summary" => intermediate_rep.track_summary.push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Video".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Video {
            audio_detected_at: intermediate_rep.audio_detected_at.into_iter().next(),
            audio_detector_claimed_until: intermediate_rep.audio_detector_claimed_until.into_iter().next(),
            audio_profile: intermediate_rep.audio_profile.into_iter().next(),
            audio_summary: intermediate_rep.audio_summary.into_iter().next(),
            camera_id: intermediate_rep.camera_id.into_iter().next(),
            camera_id_object: intermediate_rep.camera_id_object.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            deleted_at: intermediate_rep.deleted_at.into_iter().next(),
            detection_summary: intermediate_rep.detection_summary.into_iter().next(),
            duration: intermediate_rep.duration.into_iter().next(),
            ended_at: intermediate_rep.ended_at.into_iter().next(),
            file_name: intermediate_rep.file_name.into_iter().next(),
            file_size: intermediate_rep.file_size.into_iter().next(),
            id: intermediate_rep.id.into_iter().next(),
            is_low_res: intermediate_rep.is_low_res.into_iter().next(),
            object_detector_claimed_until: intermediate_rep.object_detector_claimed_until.into_iter().next(),
            object_tracker_claimed_until: intermediate_rep.object_tracker_claimed_until.into_iter().next(),
            overlay_offset_ms: intermediate_rep.overlay_offset_ms.into_iter().next(),
            referenced_by_audio_event_video_id_objects: intermediate_rep.referenced_by_audio_event_video_id_objects.into_iter().next(),
            referenced_by_detection_video_id_objects: intermediate_rep.referenced_by_detection_video_id_objects.into_iter().next(),
            referenced_by_track_video_id_objects: intermediate_rep.referenced_by_track_video_id_objects.into_iter().next(),
            started_at: intermediate_rep.started_at.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            thumbnail_name: intermediate_rep.thumbnail_name.into_iter().next(),
            track_summary: intermediate_rep.track_summary.into_iter().next(),
            updated_at: intermediate_rep.updated_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Video> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Video>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Video>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Video - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Video> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Video as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Video - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VideoAudioDetectorClaimRequest {
    #[serde(rename = "timeout_seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeout_seconds: Option<f64>,

    #[serde(rename = "until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub until: Option<chrono::DateTime::<chrono::Utc>>,

}



impl VideoAudioDetectorClaimRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> VideoAudioDetectorClaimRequest {
        VideoAudioDetectorClaimRequest {
 timeout_seconds: None,
 until: None,
        }
    }
}

/// Converts the VideoAudioDetectorClaimRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for VideoAudioDetectorClaimRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.timeout_seconds.as_ref().map(|timeout_seconds| {
                [
                    "timeout_seconds".to_string(),
                    timeout_seconds.to_string(),
                ].join(",")
            }),

            // Skipping until in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VideoAudioDetectorClaimRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VideoAudioDetectorClaimRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub timeout_seconds: Vec<f64>,
            pub until: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VideoAudioDetectorClaimRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "timeout_seconds" => intermediate_rep.timeout_seconds.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "until" => intermediate_rep.until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VideoAudioDetectorClaimRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VideoAudioDetectorClaimRequest {
            timeout_seconds: intermediate_rep.timeout_seconds.into_iter().next(),
            until: intermediate_rep.until.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VideoAudioDetectorClaimRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<VideoAudioDetectorClaimRequest>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VideoAudioDetectorClaimRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for VideoAudioDetectorClaimRequest - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<VideoAudioDetectorClaimRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VideoAudioDetectorClaimRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into VideoAudioDetectorClaimRequest - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VideoObjectDetectorClaimRequest {
    #[serde(rename = "timeout_seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeout_seconds: Option<f64>,

    #[serde(rename = "until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub until: Option<chrono::DateTime::<chrono::Utc>>,

}



impl VideoObjectDetectorClaimRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> VideoObjectDetectorClaimRequest {
        VideoObjectDetectorClaimRequest {
 timeout_seconds: None,
 until: None,
        }
    }
}

/// Converts the VideoObjectDetectorClaimRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for VideoObjectDetectorClaimRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.timeout_seconds.as_ref().map(|timeout_seconds| {
                [
                    "timeout_seconds".to_string(),
                    timeout_seconds.to_string(),
                ].join(",")
            }),

            // Skipping until in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VideoObjectDetectorClaimRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VideoObjectDetectorClaimRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub timeout_seconds: Vec<f64>,
            pub until: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VideoObjectDetectorClaimRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "timeout_seconds" => intermediate_rep.timeout_seconds.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "until" => intermediate_rep.until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VideoObjectDetectorClaimRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VideoObjectDetectorClaimRequest {
            timeout_seconds: intermediate_rep.timeout_seconds.into_iter().next(),
            until: intermediate_rep.until.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VideoObjectDetectorClaimRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<VideoObjectDetectorClaimRequest>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VideoObjectDetectorClaimRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for VideoObjectDetectorClaimRequest - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<VideoObjectDetectorClaimRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VideoObjectDetectorClaimRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into VideoObjectDetectorClaimRequest - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VideoObjectTrackerClaimRequest {
    #[serde(rename = "timeout_seconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeout_seconds: Option<f64>,

    #[serde(rename = "until")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub until: Option<chrono::DateTime::<chrono::Utc>>,

}



impl VideoObjectTrackerClaimRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> VideoObjectTrackerClaimRequest {
        VideoObjectTrackerClaimRequest {
 timeout_seconds: None,
 until: None,
        }
    }
}

/// Converts the VideoObjectTrackerClaimRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for VideoObjectTrackerClaimRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.timeout_seconds.as_ref().map(|timeout_seconds| {
                [
                    "timeout_seconds".to_string(),
                    timeout_seconds.to_string(),
                ].join(",")
            }),

            // Skipping until in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VideoObjectTrackerClaimRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VideoObjectTrackerClaimRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub timeout_seconds: Vec<f64>,
            pub until: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VideoObjectTrackerClaimRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "timeout_seconds" => intermediate_rep.timeout_seconds.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "until" => intermediate_rep.until.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VideoObjectTrackerClaimRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VideoObjectTrackerClaimRequest {
            timeout_seconds: intermediate_rep.timeout_seconds.into_iter().next(),
            until: intermediate_rep.until.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VideoObjectTrackerClaimRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<VideoObjectTrackerClaimRequest>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VideoObjectTrackerClaimRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for VideoObjectTrackerClaimRequest - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<VideoObjectTrackerClaimRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VideoObjectTrackerClaimRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into VideoObjectTrackerClaimRequest - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}


