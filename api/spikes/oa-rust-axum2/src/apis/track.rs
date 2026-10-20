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
pub enum DeleteTrackResponse {
    /// DeleteTrack success
    Status204_DeleteTrackSuccess
    ,
    /// DeleteTrack failure
    Status0_DeleteTrackFailure
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTrackResponse {
    /// GetTrack success
    Status200_GetTrackSuccess
    (models::ResponseWithGenericOfTrack)
    ,
    /// GetTrack failure
    Status0_GetTrackFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTracksResponse {
    /// GetTracks success
    Status200_GetTracksSuccess
    (models::ResponseWithGenericOfTrack)
    ,
    /// GetTracks failure
    Status0_GetTracksFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PatchTrackResponse {
    /// PatchTrack success
    Status200_PatchTrackSuccess
    (models::ResponseWithGenericOfTrack)
    ,
    /// PatchTrack failure
    Status0_PatchTrackFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostTracksResponse {
    /// PostTracks success
    Status201_PostTracksSuccess
    (models::ResponseWithGenericOfTrack)
    ,
    /// PostTracks failure
    Status0_PostTracksFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// Track
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Track<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// DeleteTrack - DELETE /api/tracks/{primaryKey}
    async fn delete_track(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::DeleteTrackPathParams,
      query_params: &models::DeleteTrackQueryParams,
    ) -> Result<DeleteTrackResponse, E>;

    /// GetTrack - GET /api/tracks/{primaryKey}
    async fn get_track(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetTrackPathParams,
      query_params: &models::GetTrackQueryParams,
    ) -> Result<GetTrackResponse, E>;

    /// GetTracks - GET /api/tracks
    async fn get_tracks(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetTracksQueryParams,
    ) -> Result<GetTracksResponse, E>;

    /// PatchTrack - PATCH /api/tracks/{primaryKey}
    async fn patch_track(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PatchTrackPathParams,
      query_params: &models::PatchTrackQueryParams,
            body: &models::Track,
    ) -> Result<PatchTrackResponse, E>;

    /// PostTracks - POST /api/tracks
    async fn post_tracks(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostTracksQueryParams,
            body: &Vec<models::Track>,
    ) -> Result<PostTracksResponse, E>;
}
