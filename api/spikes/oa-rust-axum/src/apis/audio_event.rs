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
pub enum DeleteAudioEventResponse {
    /// DeleteAudioEvent success
    Status204_DeleteAudioEventSuccess
    ,
    /// DeleteAudioEvent failure
    Status0_DeleteAudioEventFailure
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAudioEventResponse {
    /// GetAudioEvent success
    Status200_GetAudioEventSuccess
    (models::ResponseWithGenericOfAudioEvent)
    ,
    /// GetAudioEvent failure
    Status0_GetAudioEventFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAudioEventsResponse {
    /// GetAudioEvents success
    Status200_GetAudioEventsSuccess
    (models::ResponseWithGenericOfAudioEvent)
    ,
    /// GetAudioEvents failure
    Status0_GetAudioEventsFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PatchAudioEventResponse {
    /// PatchAudioEvent success
    Status200_PatchAudioEventSuccess
    (models::ResponseWithGenericOfAudioEvent)
    ,
    /// PatchAudioEvent failure
    Status0_PatchAudioEventFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostAudioEventsResponse {
    /// PostAudioEvents success
    Status201_PostAudioEventsSuccess
    (models::ResponseWithGenericOfAudioEvent)
    ,
    /// PostAudioEvents failure
    Status0_PostAudioEventsFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// AudioEvent
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait AudioEvent<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// DeleteAudioEvent - DELETE /api/audio-events/{primaryKey}
    async fn delete_audio_event(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::DeleteAudioEventPathParams,
      query_params: &models::DeleteAudioEventQueryParams,
    ) -> Result<DeleteAudioEventResponse, E>;

    /// GetAudioEvent - GET /api/audio-events/{primaryKey}
    async fn get_audio_event(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetAudioEventPathParams,
      query_params: &models::GetAudioEventQueryParams,
    ) -> Result<GetAudioEventResponse, E>;

    /// GetAudioEvents - GET /api/audio-events
    async fn get_audio_events(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetAudioEventsQueryParams,
    ) -> Result<GetAudioEventsResponse, E>;

    /// PatchAudioEvent - PATCH /api/audio-events/{primaryKey}
    async fn patch_audio_event(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::PatchAudioEventPathParams,
      query_params: &models::PatchAudioEventQueryParams,
            body: &models::AudioEvent,
    ) -> Result<PatchAudioEventResponse, E>;

    /// PostAudioEvents - POST /api/audio-events
    async fn post_audio_events(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostAudioEventsQueryParams,
            body: &models::ArrayOfNullableAudioEvent,
    ) -> Result<PostAudioEventsResponse, E>;
}
