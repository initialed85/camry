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
pub enum DeleteVideoResponse {
    /// DeleteVideo success
    Status204_DeleteVideoSuccess
    ,
    /// DeleteVideo failure
    Status0_DeleteVideoFailure
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetVideoResponse {
    /// GetVideo success
    Status200_GetVideoSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// GetVideo failure
    Status0_GetVideoFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetVideosResponse {
    /// GetVideos success
    Status200_GetVideosSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// GetVideos failure
    Status0_GetVideosFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PatchVideoResponse {
    /// PatchVideo success
    Status200_PatchVideoSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// PatchVideo failure
    Status0_PatchVideoFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostVideosResponse {
    /// PostVideos success
    Status201_PostVideosSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// PostVideos failure
    Status0_PostVideosFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostVideosAudioDetectorClaimResponse {
    /// PostVideosAudioDetectorClaim success
    Status200_PostVideosAudioDetectorClaimSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// PostVideosAudioDetectorClaim failure
    Status0_PostVideosAudioDetectorClaimFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostVideosObjectDetectorClaimResponse {
    /// PostVideosObjectDetectorClaim success
    Status200_PostVideosObjectDetectorClaimSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// PostVideosObjectDetectorClaim failure
    Status0_PostVideosObjectDetectorClaimFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostVideosObjectTrackerClaimResponse {
    /// PostVideosObjectTrackerClaim success
    Status200_PostVideosObjectTrackerClaimSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// PostVideosObjectTrackerClaim failure
    Status0_PostVideosObjectTrackerClaimFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// Video
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Video<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// DeleteVideo - DELETE /api/videos/{primaryKey}
    async fn delete_video(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::DeleteVideoPathParams,
      query_params: &models::DeleteVideoQueryParams,
    ) -> Result<DeleteVideoResponse, E>;

    /// GetVideo - GET /api/videos/{primaryKey}
    async fn get_video(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetVideoPathParams,
      query_params: &models::GetVideoQueryParams,
    ) -> Result<GetVideoResponse, E>;

    /// GetVideos - GET /api/videos
    async fn get_videos(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetVideosQueryParams,
    ) -> Result<GetVideosResponse, E>;

    /// PatchVideo - PATCH /api/videos/{primaryKey}
    async fn patch_video(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PatchVideoPathParams,
      query_params: &models::PatchVideoQueryParams,
            body: &models::Video,
    ) -> Result<PatchVideoResponse, E>;

    /// PostVideos - POST /api/videos
    async fn post_videos(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostVideosQueryParams,
            body: &Vec<models::Video>,
    ) -> Result<PostVideosResponse, E>;

    /// PostVideosAudioDetectorClaim - POST /api/videos/{primaryKey}/audio-detector-claim
    async fn post_videos_audio_detector_claim(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PostVideosAudioDetectorClaimPathParams,
      query_params: &models::PostVideosAudioDetectorClaimQueryParams,
            body: &models::VideoAudioDetectorClaimRequest,
    ) -> Result<PostVideosAudioDetectorClaimResponse, E>;

    /// PostVideosObjectDetectorClaim - POST /api/videos/{primaryKey}/object-detector-claim
    async fn post_videos_object_detector_claim(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PostVideosObjectDetectorClaimPathParams,
      query_params: &models::PostVideosObjectDetectorClaimQueryParams,
            body: &models::VideoObjectDetectorClaimRequest,
    ) -> Result<PostVideosObjectDetectorClaimResponse, E>;

    /// PostVideosObjectTrackerClaim - POST /api/videos/{primaryKey}/object-tracker-claim
    async fn post_videos_object_tracker_claim(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PostVideosObjectTrackerClaimPathParams,
      query_params: &models::PostVideosObjectTrackerClaimQueryParams,
            body: &models::VideoObjectTrackerClaimRequest,
    ) -> Result<PostVideosObjectTrackerClaimResponse, E>;
}
