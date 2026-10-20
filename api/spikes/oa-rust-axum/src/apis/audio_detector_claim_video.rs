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
pub enum PostAudioDetectorClaimVideosResponse {
    /// PostAudioDetectorClaimVideos success
    Status200_PostAudioDetectorClaimVideosSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// PostAudioDetectorClaimVideos failure
    Status0_PostAudioDetectorClaimVideosFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// AudioDetectorClaimVideo
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait AudioDetectorClaimVideo<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// PostAudioDetectorClaimVideos - POST /api/audio-detector-claim-video
    async fn post_audio_detector_claim_videos(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostAudioDetectorClaimVideosQueryParams,
            body: &models::VideoAudioDetectorClaimRequest,
    ) -> Result<PostAudioDetectorClaimVideosResponse, E>;
}
