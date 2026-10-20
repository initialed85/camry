use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::{
    TypedHeader,
    extract::{CookieJar, Query as QueryExtra},
};
use bytes::Bytes;
use headers::Host;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};
use tracing::error;
use validator::{Validate, ValidationErrors};

#[allow(unused_imports)]
use crate::{apis, models};
use crate::{header, types::*};
#[allow(unused_imports)]
use crate::{
    models::check_xss_map, models::check_xss_map_nested, models::check_xss_map_string,
    models::check_xss_string, models::check_xss_vec_string,
};


/// Setup API Server.
pub fn new<I, A, E>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::audio_detector_claim_video::AudioDetectorClaimVideo<E> + apis::audio_event::AudioEvent<E> + apis::camera::Camera<E> + apis::detection::Detection<E> + apis::object_detector_claim_video::ObjectDetectorClaimVideo<E> + apis::object_tracker_claim_video::ObjectTrackerClaimVideo<E> + apis::segment_producer_claim_camera::SegmentProducerClaimCamera<E> + apis::stream_producer_claim_camera::StreamProducerClaimCamera<E> + apis::track::Track<E> + apis::video::Video<E> + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
    
{
    // build our application with a route
    Router::new()
        .route("/api/audio-detector-claim-video",
            post(post_audio_detector_claim_videos::<I, A, E>)
        )
        .route("/api/audio-events",
            get(get_audio_events::<I, A, E>).post(post_audio_events::<I, A, E>)
        )
        .route("/api/audio-events/{primary_key}",
            delete(delete_audio_event::<I, A, E>).get(get_audio_event::<I, A, E>).patch(patch_audio_event::<I, A, E>)
        )
        .route("/api/cameras",
            get(get_cameras::<I, A, E>).post(post_cameras::<I, A, E>)
        )
        .route("/api/cameras/{primary_key}",
            delete(delete_camera::<I, A, E>).get(get_camera::<I, A, E>).patch(patch_camera::<I, A, E>)
        )
        .route("/api/cameras/{primary_key}/segment-producer-claim",
            post(post_cameras_segment_producer_claim::<I, A, E>)
        )
        .route("/api/cameras/{primary_key}/stream-producer-claim",
            post(post_cameras_stream_producer_claim::<I, A, E>)
        )
        .route("/api/detections",
            get(get_detections::<I, A, E>).post(post_detections::<I, A, E>)
        )
        .route("/api/detections/{primary_key}",
            delete(delete_detection::<I, A, E>).get(get_detection::<I, A, E>).patch(patch_detection::<I, A, E>)
        )
        .route("/api/object-detector-claim-video",
            post(post_object_detector_claim_videos::<I, A, E>)
        )
        .route("/api/object-tracker-claim-video",
            post(post_object_tracker_claim_videos::<I, A, E>)
        )
        .route("/api/segment-producer-claim-camera",
            post(post_segment_producer_claim_cameras::<I, A, E>)
        )
        .route("/api/stream-producer-claim-camera",
            post(post_stream_producer_claim_cameras::<I, A, E>)
        )
        .route("/api/tracks",
            get(get_tracks::<I, A, E>).post(post_tracks::<I, A, E>)
        )
        .route("/api/tracks/{primary_key}",
            delete(delete_track::<I, A, E>).get(get_track::<I, A, E>).patch(patch_track::<I, A, E>)
        )
        .route("/api/videos",
            get(get_videos::<I, A, E>).post(post_videos::<I, A, E>)
        )
        .route("/api/videos/{primary_key}",
            delete(delete_video::<I, A, E>).get(get_video::<I, A, E>).patch(patch_video::<I, A, E>)
        )
        .route("/api/videos/{primary_key}/audio-detector-claim",
            post(post_videos_audio_detector_claim::<I, A, E>)
        )
        .route("/api/videos/{primary_key}/object-detector-claim",
            post(post_videos_object_detector_claim::<I, A, E>)
        )
        .route("/api/videos/{primary_key}/object-tracker-claim",
            post(post_videos_object_tracker_claim::<I, A, E>)
        )
        .with_state(api_impl)
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostAudioDetectorClaimVideosBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::VideoAudioDetectorClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_audio_detector_claim_videos_validation(
  query_params: models::PostAudioDetectorClaimVideosQueryParams,
        body: models::VideoAudioDetectorClaimRequest,
) -> std::result::Result<(
  models::PostAudioDetectorClaimVideosQueryParams,
        models::VideoAudioDetectorClaimRequest,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostAudioDetectorClaimVideosBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostAudioDetectorClaimVideos - POST /api/audio-detector-claim-video
#[tracing::instrument(skip_all)]
async fn post_audio_detector_claim_videos<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostAudioDetectorClaimVideosQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::VideoAudioDetectorClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::audio_detector_claim_video::AudioDetectorClaimVideo<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_audio_detector_claim_videos_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_audio_detector_claim_videos(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::audio_detector_claim_video::PostAudioDetectorClaimVideosResponse::Status200_PostAudioDetectorClaimVideosSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::audio_detector_claim_video::PostAudioDetectorClaimVideosResponse::Status0_PostAudioDetectorClaimVideosFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn delete_audio_event_validation(
  path_params: models::DeleteAudioEventPathParams,
  query_params: models::DeleteAudioEventQueryParams,
) -> std::result::Result<(
  models::DeleteAudioEventPathParams,
  models::DeleteAudioEventQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// DeleteAudioEvent - DELETE /api/audio-events/{primaryKey}
#[tracing::instrument(skip_all)]
async fn delete_audio_event<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::DeleteAudioEventPathParams>,
  QueryExtra(query_params): QueryExtra<models::DeleteAudioEventQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::audio_event::AudioEvent<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    delete_audio_event_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().delete_audio_event(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::audio_event::DeleteAudioEventResponse::Status204_DeleteAudioEventSuccess
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::audio_event::DeleteAudioEventResponse::Status0_DeleteAudioEventFailure
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_audio_event_validation(
  path_params: models::GetAudioEventPathParams,
  query_params: models::GetAudioEventQueryParams,
) -> std::result::Result<(
  models::GetAudioEventPathParams,
  models::GetAudioEventQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// GetAudioEvent - GET /api/audio-events/{primaryKey}
#[tracing::instrument(skip_all)]
async fn get_audio_event<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::GetAudioEventPathParams>,
  QueryExtra(query_params): QueryExtra<models::GetAudioEventQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::audio_event::AudioEvent<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_audio_event_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_audio_event(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::audio_event::GetAudioEventResponse::Status200_GetAudioEventSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::audio_event::GetAudioEventResponse::Status0_GetAudioEventFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_audio_events_validation(
  query_params: models::GetAudioEventsQueryParams,
) -> std::result::Result<(
  models::GetAudioEventsQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// GetAudioEvents - GET /api/audio-events
#[tracing::instrument(skip_all)]
async fn get_audio_events<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::GetAudioEventsQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::audio_event::AudioEvent<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_audio_events_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_audio_events(
      
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::audio_event::GetAudioEventsResponse::Status200_GetAudioEventsSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::audio_event::GetAudioEventsResponse::Status0_GetAudioEventsFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PatchAudioEventBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::AudioEvent,
    }


#[tracing::instrument(skip_all)]
fn patch_audio_event_validation(
  path_params: models::PatchAudioEventPathParams,
  query_params: models::PatchAudioEventQueryParams,
        body: models::AudioEvent,
) -> std::result::Result<(
  models::PatchAudioEventPathParams,
  models::PatchAudioEventQueryParams,
        models::AudioEvent,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PatchAudioEventBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PatchAudioEvent - PATCH /api/audio-events/{primaryKey}
#[tracing::instrument(skip_all)]
async fn patch_audio_event<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PatchAudioEventPathParams>,
  QueryExtra(query_params): QueryExtra<models::PatchAudioEventQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::AudioEvent>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::audio_event::AudioEvent<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    patch_audio_event_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().patch_audio_event(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::audio_event::PatchAudioEventResponse::Status200_PatchAudioEventSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::audio_event::PatchAudioEventResponse::Status0_PatchAudioEventFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostAudioEventsBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::ArrayOfNullableAudioEvent,
    }


#[tracing::instrument(skip_all)]
fn post_audio_events_validation(
  query_params: models::PostAudioEventsQueryParams,
        body: models::ArrayOfNullableAudioEvent,
) -> std::result::Result<(
  models::PostAudioEventsQueryParams,
        models::ArrayOfNullableAudioEvent,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostAudioEventsBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostAudioEvents - POST /api/audio-events
#[tracing::instrument(skip_all)]
async fn post_audio_events<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostAudioEventsQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::ArrayOfNullableAudioEvent>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::audio_event::AudioEvent<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_audio_events_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_audio_events(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::audio_event::PostAudioEventsResponse::Status201_PostAudioEventsSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::audio_event::PostAudioEventsResponse::Status0_PostAudioEventsFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn delete_camera_validation(
  path_params: models::DeleteCameraPathParams,
  query_params: models::DeleteCameraQueryParams,
) -> std::result::Result<(
  models::DeleteCameraPathParams,
  models::DeleteCameraQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// DeleteCamera - DELETE /api/cameras/{primaryKey}
#[tracing::instrument(skip_all)]
async fn delete_camera<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::DeleteCameraPathParams>,
  QueryExtra(query_params): QueryExtra<models::DeleteCameraQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::camera::Camera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    delete_camera_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().delete_camera(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::camera::DeleteCameraResponse::Status204_DeleteCameraSuccess
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::camera::DeleteCameraResponse::Status0_DeleteCameraFailure
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_camera_validation(
  path_params: models::GetCameraPathParams,
  query_params: models::GetCameraQueryParams,
) -> std::result::Result<(
  models::GetCameraPathParams,
  models::GetCameraQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// GetCamera - GET /api/cameras/{primaryKey}
#[tracing::instrument(skip_all)]
async fn get_camera<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::GetCameraPathParams>,
  QueryExtra(query_params): QueryExtra<models::GetCameraQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::camera::Camera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_camera_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_camera(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::camera::GetCameraResponse::Status200_GetCameraSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::camera::GetCameraResponse::Status0_GetCameraFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_cameras_validation(
  query_params: models::GetCamerasQueryParams,
) -> std::result::Result<(
  models::GetCamerasQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// GetCameras - GET /api/cameras
#[tracing::instrument(skip_all)]
async fn get_cameras<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::GetCamerasQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::camera::Camera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_cameras_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_cameras(
      
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::camera::GetCamerasResponse::Status200_GetCamerasSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::camera::GetCamerasResponse::Status0_GetCamerasFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PatchCameraBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::Camera,
    }


#[tracing::instrument(skip_all)]
fn patch_camera_validation(
  path_params: models::PatchCameraPathParams,
  query_params: models::PatchCameraQueryParams,
        body: models::Camera,
) -> std::result::Result<(
  models::PatchCameraPathParams,
  models::PatchCameraQueryParams,
        models::Camera,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PatchCameraBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PatchCamera - PATCH /api/cameras/{primaryKey}
#[tracing::instrument(skip_all)]
async fn patch_camera<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PatchCameraPathParams>,
  QueryExtra(query_params): QueryExtra<models::PatchCameraQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::Camera>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::camera::Camera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    patch_camera_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().patch_camera(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::camera::PatchCameraResponse::Status200_PatchCameraSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::camera::PatchCameraResponse::Status0_PatchCameraFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostCamerasBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::ArrayOfNullableCamera,
    }


#[tracing::instrument(skip_all)]
fn post_cameras_validation(
  query_params: models::PostCamerasQueryParams,
        body: models::ArrayOfNullableCamera,
) -> std::result::Result<(
  models::PostCamerasQueryParams,
        models::ArrayOfNullableCamera,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostCamerasBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostCameras - POST /api/cameras
#[tracing::instrument(skip_all)]
async fn post_cameras<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostCamerasQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::ArrayOfNullableCamera>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::camera::Camera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_cameras_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_cameras(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::camera::PostCamerasResponse::Status201_PostCamerasSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::camera::PostCamerasResponse::Status0_PostCamerasFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostCamerasSegmentProducerClaimBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::CameraSegmentProducerClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_cameras_segment_producer_claim_validation(
  path_params: models::PostCamerasSegmentProducerClaimPathParams,
  query_params: models::PostCamerasSegmentProducerClaimQueryParams,
        body: models::CameraSegmentProducerClaimRequest,
) -> std::result::Result<(
  models::PostCamerasSegmentProducerClaimPathParams,
  models::PostCamerasSegmentProducerClaimQueryParams,
        models::CameraSegmentProducerClaimRequest,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PostCamerasSegmentProducerClaimBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PostCamerasSegmentProducerClaim - POST /api/cameras/{primaryKey}/segment-producer-claim
#[tracing::instrument(skip_all)]
async fn post_cameras_segment_producer_claim<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PostCamerasSegmentProducerClaimPathParams>,
  QueryExtra(query_params): QueryExtra<models::PostCamerasSegmentProducerClaimQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::CameraSegmentProducerClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::camera::Camera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_cameras_segment_producer_claim_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_cameras_segment_producer_claim(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::camera::PostCamerasSegmentProducerClaimResponse::Status200_PostCamerasSegmentProducerClaimSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::camera::PostCamerasSegmentProducerClaimResponse::Status0_PostCamerasSegmentProducerClaimFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostCamerasStreamProducerClaimBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::CameraStreamProducerClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_cameras_stream_producer_claim_validation(
  path_params: models::PostCamerasStreamProducerClaimPathParams,
  query_params: models::PostCamerasStreamProducerClaimQueryParams,
        body: models::CameraStreamProducerClaimRequest,
) -> std::result::Result<(
  models::PostCamerasStreamProducerClaimPathParams,
  models::PostCamerasStreamProducerClaimQueryParams,
        models::CameraStreamProducerClaimRequest,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PostCamerasStreamProducerClaimBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PostCamerasStreamProducerClaim - POST /api/cameras/{primaryKey}/stream-producer-claim
#[tracing::instrument(skip_all)]
async fn post_cameras_stream_producer_claim<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PostCamerasStreamProducerClaimPathParams>,
  QueryExtra(query_params): QueryExtra<models::PostCamerasStreamProducerClaimQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::CameraStreamProducerClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::camera::Camera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_cameras_stream_producer_claim_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_cameras_stream_producer_claim(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::camera::PostCamerasStreamProducerClaimResponse::Status200_PostCamerasStreamProducerClaimSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::camera::PostCamerasStreamProducerClaimResponse::Status0_PostCamerasStreamProducerClaimFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn delete_detection_validation(
  path_params: models::DeleteDetectionPathParams,
  query_params: models::DeleteDetectionQueryParams,
) -> std::result::Result<(
  models::DeleteDetectionPathParams,
  models::DeleteDetectionQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// DeleteDetection - DELETE /api/detections/{primaryKey}
#[tracing::instrument(skip_all)]
async fn delete_detection<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::DeleteDetectionPathParams>,
  QueryExtra(query_params): QueryExtra<models::DeleteDetectionQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::detection::Detection<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    delete_detection_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().delete_detection(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::detection::DeleteDetectionResponse::Status204_DeleteDetectionSuccess
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::detection::DeleteDetectionResponse::Status0_DeleteDetectionFailure
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_detection_validation(
  path_params: models::GetDetectionPathParams,
  query_params: models::GetDetectionQueryParams,
) -> std::result::Result<(
  models::GetDetectionPathParams,
  models::GetDetectionQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// GetDetection - GET /api/detections/{primaryKey}
#[tracing::instrument(skip_all)]
async fn get_detection<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::GetDetectionPathParams>,
  QueryExtra(query_params): QueryExtra<models::GetDetectionQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::detection::Detection<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_detection_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_detection(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::detection::GetDetectionResponse::Status200_GetDetectionSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::detection::GetDetectionResponse::Status0_GetDetectionFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_detections_validation(
  query_params: models::GetDetectionsQueryParams,
) -> std::result::Result<(
  models::GetDetectionsQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// GetDetections - GET /api/detections
#[tracing::instrument(skip_all)]
async fn get_detections<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::GetDetectionsQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::detection::Detection<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_detections_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_detections(
      
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::detection::GetDetectionsResponse::Status200_GetDetectionsSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::detection::GetDetectionsResponse::Status0_GetDetectionsFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PatchDetectionBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::Detection,
    }


#[tracing::instrument(skip_all)]
fn patch_detection_validation(
  path_params: models::PatchDetectionPathParams,
  query_params: models::PatchDetectionQueryParams,
        body: models::Detection,
) -> std::result::Result<(
  models::PatchDetectionPathParams,
  models::PatchDetectionQueryParams,
        models::Detection,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PatchDetectionBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PatchDetection - PATCH /api/detections/{primaryKey}
#[tracing::instrument(skip_all)]
async fn patch_detection<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PatchDetectionPathParams>,
  QueryExtra(query_params): QueryExtra<models::PatchDetectionQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::Detection>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::detection::Detection<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    patch_detection_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().patch_detection(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::detection::PatchDetectionResponse::Status200_PatchDetectionSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::detection::PatchDetectionResponse::Status0_PatchDetectionFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostDetectionsBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::ArrayOfNullableDetection,
    }


#[tracing::instrument(skip_all)]
fn post_detections_validation(
  query_params: models::PostDetectionsQueryParams,
        body: models::ArrayOfNullableDetection,
) -> std::result::Result<(
  models::PostDetectionsQueryParams,
        models::ArrayOfNullableDetection,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostDetectionsBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostDetections - POST /api/detections
#[tracing::instrument(skip_all)]
async fn post_detections<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostDetectionsQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::ArrayOfNullableDetection>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::detection::Detection<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_detections_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_detections(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::detection::PostDetectionsResponse::Status201_PostDetectionsSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::detection::PostDetectionsResponse::Status0_PostDetectionsFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostObjectDetectorClaimVideosBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::VideoObjectDetectorClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_object_detector_claim_videos_validation(
  query_params: models::PostObjectDetectorClaimVideosQueryParams,
        body: models::VideoObjectDetectorClaimRequest,
) -> std::result::Result<(
  models::PostObjectDetectorClaimVideosQueryParams,
        models::VideoObjectDetectorClaimRequest,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostObjectDetectorClaimVideosBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostObjectDetectorClaimVideos - POST /api/object-detector-claim-video
#[tracing::instrument(skip_all)]
async fn post_object_detector_claim_videos<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostObjectDetectorClaimVideosQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::VideoObjectDetectorClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::object_detector_claim_video::ObjectDetectorClaimVideo<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_object_detector_claim_videos_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_object_detector_claim_videos(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::object_detector_claim_video::PostObjectDetectorClaimVideosResponse::Status200_PostObjectDetectorClaimVideosSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::object_detector_claim_video::PostObjectDetectorClaimVideosResponse::Status0_PostObjectDetectorClaimVideosFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostObjectTrackerClaimVideosBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::VideoObjectTrackerClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_object_tracker_claim_videos_validation(
  query_params: models::PostObjectTrackerClaimVideosQueryParams,
        body: models::VideoObjectTrackerClaimRequest,
) -> std::result::Result<(
  models::PostObjectTrackerClaimVideosQueryParams,
        models::VideoObjectTrackerClaimRequest,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostObjectTrackerClaimVideosBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostObjectTrackerClaimVideos - POST /api/object-tracker-claim-video
#[tracing::instrument(skip_all)]
async fn post_object_tracker_claim_videos<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostObjectTrackerClaimVideosQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::VideoObjectTrackerClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::object_tracker_claim_video::ObjectTrackerClaimVideo<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_object_tracker_claim_videos_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_object_tracker_claim_videos(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::object_tracker_claim_video::PostObjectTrackerClaimVideosResponse::Status200_PostObjectTrackerClaimVideosSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::object_tracker_claim_video::PostObjectTrackerClaimVideosResponse::Status0_PostObjectTrackerClaimVideosFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostSegmentProducerClaimCamerasBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::CameraSegmentProducerClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_segment_producer_claim_cameras_validation(
  query_params: models::PostSegmentProducerClaimCamerasQueryParams,
        body: models::CameraSegmentProducerClaimRequest,
) -> std::result::Result<(
  models::PostSegmentProducerClaimCamerasQueryParams,
        models::CameraSegmentProducerClaimRequest,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostSegmentProducerClaimCamerasBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostSegmentProducerClaimCameras - POST /api/segment-producer-claim-camera
#[tracing::instrument(skip_all)]
async fn post_segment_producer_claim_cameras<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostSegmentProducerClaimCamerasQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::CameraSegmentProducerClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::segment_producer_claim_camera::SegmentProducerClaimCamera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_segment_producer_claim_cameras_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_segment_producer_claim_cameras(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::segment_producer_claim_camera::PostSegmentProducerClaimCamerasResponse::Status200_PostSegmentProducerClaimCamerasSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::segment_producer_claim_camera::PostSegmentProducerClaimCamerasResponse::Status0_PostSegmentProducerClaimCamerasFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostStreamProducerClaimCamerasBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::CameraStreamProducerClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_stream_producer_claim_cameras_validation(
  query_params: models::PostStreamProducerClaimCamerasQueryParams,
        body: models::CameraStreamProducerClaimRequest,
) -> std::result::Result<(
  models::PostStreamProducerClaimCamerasQueryParams,
        models::CameraStreamProducerClaimRequest,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostStreamProducerClaimCamerasBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostStreamProducerClaimCameras - POST /api/stream-producer-claim-camera
#[tracing::instrument(skip_all)]
async fn post_stream_producer_claim_cameras<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostStreamProducerClaimCamerasQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::CameraStreamProducerClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::stream_producer_claim_camera::StreamProducerClaimCamera<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_stream_producer_claim_cameras_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_stream_producer_claim_cameras(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::stream_producer_claim_camera::PostStreamProducerClaimCamerasResponse::Status200_PostStreamProducerClaimCamerasSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::stream_producer_claim_camera::PostStreamProducerClaimCamerasResponse::Status0_PostStreamProducerClaimCamerasFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn delete_track_validation(
  path_params: models::DeleteTrackPathParams,
  query_params: models::DeleteTrackQueryParams,
) -> std::result::Result<(
  models::DeleteTrackPathParams,
  models::DeleteTrackQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// DeleteTrack - DELETE /api/tracks/{primaryKey}
#[tracing::instrument(skip_all)]
async fn delete_track<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::DeleteTrackPathParams>,
  QueryExtra(query_params): QueryExtra<models::DeleteTrackQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::track::Track<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    delete_track_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().delete_track(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::track::DeleteTrackResponse::Status204_DeleteTrackSuccess
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::track::DeleteTrackResponse::Status0_DeleteTrackFailure
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_track_validation(
  path_params: models::GetTrackPathParams,
  query_params: models::GetTrackQueryParams,
) -> std::result::Result<(
  models::GetTrackPathParams,
  models::GetTrackQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// GetTrack - GET /api/tracks/{primaryKey}
#[tracing::instrument(skip_all)]
async fn get_track<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::GetTrackPathParams>,
  QueryExtra(query_params): QueryExtra<models::GetTrackQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::track::Track<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_track_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_track(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::track::GetTrackResponse::Status200_GetTrackSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::track::GetTrackResponse::Status0_GetTrackFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_tracks_validation(
  query_params: models::GetTracksQueryParams,
) -> std::result::Result<(
  models::GetTracksQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// GetTracks - GET /api/tracks
#[tracing::instrument(skip_all)]
async fn get_tracks<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::GetTracksQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::track::Track<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_tracks_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_tracks(
      
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::track::GetTracksResponse::Status200_GetTracksSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::track::GetTracksResponse::Status0_GetTracksFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PatchTrackBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::Track,
    }


#[tracing::instrument(skip_all)]
fn patch_track_validation(
  path_params: models::PatchTrackPathParams,
  query_params: models::PatchTrackQueryParams,
        body: models::Track,
) -> std::result::Result<(
  models::PatchTrackPathParams,
  models::PatchTrackQueryParams,
        models::Track,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PatchTrackBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PatchTrack - PATCH /api/tracks/{primaryKey}
#[tracing::instrument(skip_all)]
async fn patch_track<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PatchTrackPathParams>,
  QueryExtra(query_params): QueryExtra<models::PatchTrackQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::Track>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::track::Track<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    patch_track_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().patch_track(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::track::PatchTrackResponse::Status200_PatchTrackSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::track::PatchTrackResponse::Status0_PatchTrackFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostTracksBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::ArrayOfNullableTrack,
    }


#[tracing::instrument(skip_all)]
fn post_tracks_validation(
  query_params: models::PostTracksQueryParams,
        body: models::ArrayOfNullableTrack,
) -> std::result::Result<(
  models::PostTracksQueryParams,
        models::ArrayOfNullableTrack,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostTracksBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostTracks - POST /api/tracks
#[tracing::instrument(skip_all)]
async fn post_tracks<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostTracksQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::ArrayOfNullableTrack>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::track::Track<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_tracks_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_tracks(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::track::PostTracksResponse::Status201_PostTracksSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::track::PostTracksResponse::Status0_PostTracksFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn delete_video_validation(
  path_params: models::DeleteVideoPathParams,
  query_params: models::DeleteVideoQueryParams,
) -> std::result::Result<(
  models::DeleteVideoPathParams,
  models::DeleteVideoQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// DeleteVideo - DELETE /api/videos/{primaryKey}
#[tracing::instrument(skip_all)]
async fn delete_video<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::DeleteVideoPathParams>,
  QueryExtra(query_params): QueryExtra<models::DeleteVideoQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::video::Video<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    delete_video_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().delete_video(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::video::DeleteVideoResponse::Status204_DeleteVideoSuccess
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::video::DeleteVideoResponse::Status0_DeleteVideoFailure
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_video_validation(
  path_params: models::GetVideoPathParams,
  query_params: models::GetVideoQueryParams,
) -> std::result::Result<(
  models::GetVideoPathParams,
  models::GetVideoQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// GetVideo - GET /api/videos/{primaryKey}
#[tracing::instrument(skip_all)]
async fn get_video<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::GetVideoPathParams>,
  QueryExtra(query_params): QueryExtra<models::GetVideoQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::video::Video<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_video_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_video(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::video::GetVideoResponse::Status200_GetVideoSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::video::GetVideoResponse::Status0_GetVideoFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_videos_validation(
  query_params: models::GetVideosQueryParams,
) -> std::result::Result<(
  models::GetVideosQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// GetVideos - GET /api/videos
#[tracing::instrument(skip_all)]
async fn get_videos<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::GetVideosQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::video::Video<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_videos_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().get_videos(
      
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::video::GetVideosResponse::Status200_GetVideosSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::video::GetVideosResponse::Status0_GetVideosFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PatchVideoBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::Video,
    }


#[tracing::instrument(skip_all)]
fn patch_video_validation(
  path_params: models::PatchVideoPathParams,
  query_params: models::PatchVideoQueryParams,
        body: models::Video,
) -> std::result::Result<(
  models::PatchVideoPathParams,
  models::PatchVideoQueryParams,
        models::Video,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PatchVideoBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PatchVideo - PATCH /api/videos/{primaryKey}
#[tracing::instrument(skip_all)]
async fn patch_video<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PatchVideoPathParams>,
  QueryExtra(query_params): QueryExtra<models::PatchVideoQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::Video>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::video::Video<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    patch_video_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().patch_video(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::video::PatchVideoResponse::Status200_PatchVideoSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::video::PatchVideoResponse::Status0_PatchVideoFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostVideosBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::ArrayOfNullableVideo,
    }


#[tracing::instrument(skip_all)]
fn post_videos_validation(
  query_params: models::PostVideosQueryParams,
        body: models::ArrayOfNullableVideo,
) -> std::result::Result<(
  models::PostVideosQueryParams,
        models::ArrayOfNullableVideo,
), ValidationErrors>
{
  query_params.validate()?;
              let b = PostVideosBodyValidator { body: &body };
              b.validate()?;

Ok((
  query_params,
    body,
))
}
/// PostVideos - POST /api/videos
#[tracing::instrument(skip_all)]
async fn post_videos<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::PostVideosQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::ArrayOfNullableVideo>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::video::Video<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_videos_validation(
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_videos(
      
      &method,
      &host,
      &cookies,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::video::PostVideosResponse::Status201_PostVideosSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::video::PostVideosResponse::Status0_PostVideosFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostVideosAudioDetectorClaimBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::VideoAudioDetectorClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_videos_audio_detector_claim_validation(
  path_params: models::PostVideosAudioDetectorClaimPathParams,
  query_params: models::PostVideosAudioDetectorClaimQueryParams,
        body: models::VideoAudioDetectorClaimRequest,
) -> std::result::Result<(
  models::PostVideosAudioDetectorClaimPathParams,
  models::PostVideosAudioDetectorClaimQueryParams,
        models::VideoAudioDetectorClaimRequest,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PostVideosAudioDetectorClaimBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PostVideosAudioDetectorClaim - POST /api/videos/{primaryKey}/audio-detector-claim
#[tracing::instrument(skip_all)]
async fn post_videos_audio_detector_claim<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PostVideosAudioDetectorClaimPathParams>,
  QueryExtra(query_params): QueryExtra<models::PostVideosAudioDetectorClaimQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::VideoAudioDetectorClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::video::Video<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_videos_audio_detector_claim_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_videos_audio_detector_claim(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::video::PostVideosAudioDetectorClaimResponse::Status200_PostVideosAudioDetectorClaimSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::video::PostVideosAudioDetectorClaimResponse::Status0_PostVideosAudioDetectorClaimFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostVideosObjectDetectorClaimBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::VideoObjectDetectorClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_videos_object_detector_claim_validation(
  path_params: models::PostVideosObjectDetectorClaimPathParams,
  query_params: models::PostVideosObjectDetectorClaimQueryParams,
        body: models::VideoObjectDetectorClaimRequest,
) -> std::result::Result<(
  models::PostVideosObjectDetectorClaimPathParams,
  models::PostVideosObjectDetectorClaimQueryParams,
        models::VideoObjectDetectorClaimRequest,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PostVideosObjectDetectorClaimBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PostVideosObjectDetectorClaim - POST /api/videos/{primaryKey}/object-detector-claim
#[tracing::instrument(skip_all)]
async fn post_videos_object_detector_claim<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PostVideosObjectDetectorClaimPathParams>,
  QueryExtra(query_params): QueryExtra<models::PostVideosObjectDetectorClaimQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::VideoObjectDetectorClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::video::Video<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_videos_object_detector_claim_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_videos_object_detector_claim(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::video::PostVideosObjectDetectorClaimResponse::Status200_PostVideosObjectDetectorClaimSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::video::PostVideosObjectDetectorClaimResponse::Status0_PostVideosObjectDetectorClaimFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostVideosObjectTrackerClaimBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::VideoObjectTrackerClaimRequest,
    }


#[tracing::instrument(skip_all)]
fn post_videos_object_tracker_claim_validation(
  path_params: models::PostVideosObjectTrackerClaimPathParams,
  query_params: models::PostVideosObjectTrackerClaimQueryParams,
        body: models::VideoObjectTrackerClaimRequest,
) -> std::result::Result<(
  models::PostVideosObjectTrackerClaimPathParams,
  models::PostVideosObjectTrackerClaimQueryParams,
        models::VideoObjectTrackerClaimRequest,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;
              let b = PostVideosObjectTrackerClaimBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
  query_params,
    body,
))
}
/// PostVideosObjectTrackerClaim - POST /api/videos/{primaryKey}/object-tracker-claim
#[tracing::instrument(skip_all)]
async fn post_videos_object_tracker_claim<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::PostVideosObjectTrackerClaimPathParams>,
  QueryExtra(query_params): QueryExtra<models::PostVideosObjectTrackerClaimQueryParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::VideoObjectTrackerClaimRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::video::Video<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_videos_object_tracker_claim_validation(
        path_params,
        query_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



  let result = api_impl.as_ref().post_videos_object_tracker_claim(
      
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
              &body,
  ).await;

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::video::PostVideosObjectTrackerClaimResponse::Status200_PostVideosObjectTrackerClaimSuccess
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::video::PostVideosObjectTrackerClaimResponse::Status0_PostVideosObjectTrackerClaimFailure
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[allow(dead_code)]
#[inline]
fn response_with_status_code_only(code: StatusCode) -> Result<Response, StatusCode> {
   Response::builder()
          .status(code)
          .body(Body::empty())
          .map_err(|_| code)
}
