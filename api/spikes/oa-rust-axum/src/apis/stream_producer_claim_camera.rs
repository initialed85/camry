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
pub enum PostStreamProducerClaimCamerasResponse {
    /// PostStreamProducerClaimCameras success
    Status200_PostStreamProducerClaimCamerasSuccess
    (models::ResponseWithGenericOfCamera)
    ,
    /// PostStreamProducerClaimCameras failure
    Status0_PostStreamProducerClaimCamerasFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// StreamProducerClaimCamera
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait StreamProducerClaimCamera<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// PostStreamProducerClaimCameras - POST /api/stream-producer-claim-camera
    async fn post_stream_producer_claim_cameras(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostStreamProducerClaimCamerasQueryParams,
            body: &models::CameraStreamProducerClaimRequest,
    ) -> Result<PostStreamProducerClaimCamerasResponse, E>;
}
