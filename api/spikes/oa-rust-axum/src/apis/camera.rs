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
pub enum DeleteCameraResponse {
    /// DeleteCamera success
    Status204_DeleteCameraSuccess
    ,
    /// DeleteCamera failure
    Status0_DeleteCameraFailure
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetCameraResponse {
    /// GetCamera success
    Status200_GetCameraSuccess
    (models::ResponseWithGenericOfCamera)
    ,
    /// GetCamera failure
    Status0_GetCameraFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetCamerasResponse {
    /// GetCameras success
    Status200_GetCamerasSuccess
    (models::ResponseWithGenericOfCamera)
    ,
    /// GetCameras failure
    Status0_GetCamerasFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PatchCameraResponse {
    /// PatchCamera success
    Status200_PatchCameraSuccess
    (models::ResponseWithGenericOfCamera)
    ,
    /// PatchCamera failure
    Status0_PatchCameraFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCamerasResponse {
    /// PostCameras success
    Status201_PostCamerasSuccess
    (models::ResponseWithGenericOfCamera)
    ,
    /// PostCameras failure
    Status0_PostCamerasFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCamerasSegmentProducerClaimResponse {
    /// PostCamerasSegmentProducerClaim success
    Status200_PostCamerasSegmentProducerClaimSuccess
    (models::ResponseWithGenericOfCamera)
    ,
    /// PostCamerasSegmentProducerClaim failure
    Status0_PostCamerasSegmentProducerClaimFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCamerasStreamProducerClaimResponse {
    /// PostCamerasStreamProducerClaim success
    Status200_PostCamerasStreamProducerClaimSuccess
    (models::ResponseWithGenericOfCamera)
    ,
    /// PostCamerasStreamProducerClaim failure
    Status0_PostCamerasStreamProducerClaimFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// Camera
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Camera<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// DeleteCamera - DELETE /api/cameras/{primaryKey}
    async fn delete_camera(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::DeleteCameraPathParams,
      query_params: &models::DeleteCameraQueryParams,
    ) -> Result<DeleteCameraResponse, E>;

    /// GetCamera - GET /api/cameras/{primaryKey}
    async fn get_camera(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetCameraPathParams,
      query_params: &models::GetCameraQueryParams,
    ) -> Result<GetCameraResponse, E>;

    /// GetCameras - GET /api/cameras
    async fn get_cameras(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetCamerasQueryParams,
    ) -> Result<GetCamerasResponse, E>;

    /// PatchCamera - PATCH /api/cameras/{primaryKey}
    async fn patch_camera(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PatchCameraPathParams,
      query_params: &models::PatchCameraQueryParams,
            body: &models::Camera,
    ) -> Result<PatchCameraResponse, E>;

    /// PostCameras - POST /api/cameras
    async fn post_cameras(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostCamerasQueryParams,
            body: &models::ArrayOfNullableCamera,
    ) -> Result<PostCamerasResponse, E>;

    /// PostCamerasSegmentProducerClaim - POST /api/cameras/{primaryKey}/segment-producer-claim
    async fn post_cameras_segment_producer_claim(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PostCamerasSegmentProducerClaimPathParams,
      query_params: &models::PostCamerasSegmentProducerClaimQueryParams,
            body: &models::CameraSegmentProducerClaimRequest,
    ) -> Result<PostCamerasSegmentProducerClaimResponse, E>;

    /// PostCamerasStreamProducerClaim - POST /api/cameras/{primaryKey}/stream-producer-claim
    async fn post_cameras_stream_producer_claim(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PostCamerasStreamProducerClaimPathParams,
      query_params: &models::PostCamerasStreamProducerClaimQueryParams,
            body: &models::CameraStreamProducerClaimRequest,
    ) -> Result<PostCamerasStreamProducerClaimResponse, E>;
}
