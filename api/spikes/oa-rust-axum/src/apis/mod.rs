pub mod audio_detector_claim_video;
pub mod audio_event;
pub mod camera;
pub mod detection;
pub mod object_detector_claim_video;
pub mod object_tracker_claim_video;
pub mod segment_producer_claim_camera;
pub mod stream_producer_claim_camera;
pub mod track;
pub mod video;




// Error handler for unhandled errors.
#[async_trait::async_trait]
pub trait ErrorHandler<E: std::fmt::Debug + Send + Sync + 'static = ()> {
    #[allow(unused_variables)]
    #[tracing::instrument(skip_all)]
    async fn handle_error(
        &self,
        method: &::http::Method,
        host: &headers::Host,
        cookies: &axum_extra::extract::CookieJar,
        error: E
    ) -> Result<axum::response::Response, http::StatusCode> {
        tracing::error!("Unhandled error: {:?}", error);
        axum::response::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::empty())
            .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
