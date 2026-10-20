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
pub enum PostObjectTrackerClaimVideosResponse {
    /// PostObjectTrackerClaimVideos success
    Status200_PostObjectTrackerClaimVideosSuccess
    (models::ResponseWithGenericOfVideo)
    ,
    /// PostObjectTrackerClaimVideos failure
    Status0_PostObjectTrackerClaimVideosFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// ObjectTrackerClaimVideo
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ObjectTrackerClaimVideo<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// PostObjectTrackerClaimVideos - POST /api/object-tracker-claim-video
    async fn post_object_tracker_claim_videos(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostObjectTrackerClaimVideosQueryParams,
            body: &models::VideoObjectTrackerClaimRequest,
    ) -> Result<PostObjectTrackerClaimVideosResponse, E>;
}
