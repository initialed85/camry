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
pub enum PostObjectDetectorClaimVideosResponse {
    /// PostObjectDetectorClaimVideos success
    Status200_PostObjectDetectorClaimVideosSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// PostObjectDetectorClaimVideos failure
    Status0_PostObjectDetectorClaimVideosFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// ObjectDetectorClaimVideo
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ObjectDetectorClaimVideo<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// PostObjectDetectorClaimVideos - POST /api/object-detector-claim-video
    async fn post_object_detector_claim_videos(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostObjectDetectorClaimVideosQueryParams,
            body: &models::VideoObjectDetectorClaimRequest,
    ) -> Result<PostObjectDetectorClaimVideosResponse, E>;
}
