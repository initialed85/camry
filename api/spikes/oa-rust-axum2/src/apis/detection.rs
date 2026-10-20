use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteDetectionResponse {
    /// DeleteDetection success
    Status204_DeleteDetectionSuccess
    ,
    /// DeleteDetection failure
    Status0_DeleteDetectionFailure
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetDetectionResponse {
    /// GetDetection success
    Status200_GetDetectionSuccess
    (models::ResponseWithGenericOfDetection)
    ,
    /// GetDetection failure
    Status0_GetDetectionFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetDetectionsResponse {
    /// GetDetections success
    Status200_GetDetectionsSuccess
    (models::ResponseWithGenericOfDetection)
    ,
    /// GetDetections failure
    Status0_GetDetectionsFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PatchDetectionResponse {
    /// PatchDetection success
    Status200_PatchDetectionSuccess
    (models::ResponseWithGenericOfDetection)
    ,
    /// PatchDetection failure
    Status0_PatchDetectionFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDetectionsResponse {
    /// PostDetections success
    Status201_PostDetectionsSuccess
    (models::ResponseWithGenericOfDetection)
    ,
    /// PostDetections failure
    Status0_PostDetectionsFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// Detection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Detection<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// DeleteDetection - DELETE /api/detections/{primaryKey}
    async fn delete_detection(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::DeleteDetectionPathParams,
      query_params: &models::DeleteDetectionQueryParams,
    ) -> Result<DeleteDetectionResponse, E>;

    /// GetDetection - GET /api/detections/{primaryKey}
    async fn get_detection(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetDetectionPathParams,
      query_params: &models::GetDetectionQueryParams,
    ) -> Result<GetDetectionResponse, E>;

    /// GetDetections - GET /api/detections
    async fn get_detections(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetDetectionsQueryParams,
    ) -> Result<GetDetectionsResponse, E>;

    /// PatchDetection - PATCH /api/detections/{primaryKey}
    async fn patch_detection(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PatchDetectionPathParams,
      query_params: &models::PatchDetectionQueryParams,
            body: &models::Detection,
    ) -> Result<PatchDetectionResponse, E>;

    /// PostDetections - POST /api/detections
    async fn post_detections(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostDetectionsQueryParams,
            body: &Vec<models::Detection>,
    ) -> Result<PostDetectionsResponse, E>;
}
