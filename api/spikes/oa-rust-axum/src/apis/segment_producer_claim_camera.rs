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
pub enum PostSegmentProducerClaimCamerasResponse {
    /// PostSegmentProducerClaimCameras success
    Status200_PostSegmentProducerClaimCamerasSuccess
    (models::ResponseWithGenericOfCamera)
    ,
    /// PostSegmentProducerClaimCameras failure
    Status0_PostSegmentProducerClaimCamerasFailure
    (models::PostAudioDetectorClaimVideosDefaultResponse)
}




/// SegmentProducerClaimCamera
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SegmentProducerClaimCamera<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// PostSegmentProducerClaimCameras - POST /api/segment-producer-claim-camera
    async fn post_segment_producer_claim_cameras(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::PostSegmentProducerClaimCamerasQueryParams,
            body: &models::CameraSegmentProducerClaimRequest,
    ) -> Result<PostSegmentProducerClaimCamerasResponse, E>;
}
