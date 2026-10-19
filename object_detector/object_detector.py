import glob
import json
import math
import os
import re
import time
import datetime
import traceback
import uuid

import cv2
import numpy as np

from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Tuple, cast
from pytz import UTC
from ultralytics import YOLO  # type: ignore
from ultralytics.models.yolo.detect.predict import Results  # type: ignore

from .api.openapi_client import (
    ApiClient,
    Configuration,
    VideoApi,
    Video,
    DetectionApi,
    Detection,
    ArrayOfVec2Inner as DetectionBoundingBoxInner,
    ObjectDetectorClaimVideoApi,
    VideoObjectDetectorClaimRequest,
    TrackApi,
    Track,
    CameraApi,
)

debug = os.getenv("DEBUG", "") == "1"

# A sighting of the same object in the same place gets the same id, so a segment
# that is processed twice writes the same tracks instead of duplicates.
TRACK_NAMESPACE = uuid.UUID("4f2c8a0c-8f0e-4c6b-9c8a-3f9d6b1c7e24")

TRACKABLE_CLASSES = [
    c.strip()
    for c in os.getenv("TRACKABLE_CLASSES", "person,car,dog,cat,motorcycle,truck,bus,bicycle").split(",")
    if c.strip()
]

# A box seen for fewer frames than this is temporal noise - the one-frame
# giraffe - so it never becomes an object and never stamps a track on it.
TRACK_MIN_HITS = int(os.getenv("TRACK_MIN_HITS", "4"))

# A track is dropped once it goes this many frames without a matching box.
TRACK_MAX_AGE = int(os.getenv("TRACK_MAX_AGE", "10"))

# How many recent tracks a segment loads as candidates for continuity.
TRACK_SEED_LIMIT = int(os.getenv("TRACK_SEED_LIMIT", "200"))

# How much IoU plus closeness a box needs to continue an existing track.
TRACK_MATCH_SCORE = float(os.getenv("TRACK_MATCH_SCORE", "0.45"))

# Tracks seen this recently are carried across a segment boundary, so a person
# who walks across two segments keeps one identity.
TRACK_CONTINUITY_SECONDS = int(os.getenv("TRACK_CONTINUITY_SECONDS", "120"))

# The high-resolution recording and the low-resolution recording it is detected
# from are two encoders on two paths, so their frames are not the same instant.
# The detector aligns them by correlating the first seconds of both files and
# stores the offset on the row. A scene with nothing moving in it says nothing,
# so the measurement is gated on how much the best alignment beats zero offset.
OVERLAY_MEASURE_SECONDS = int(os.getenv("OVERLAY_MEASURE_SECONDS", "12"))
OVERLAY_MEASURE_FPS = int(os.getenv("OVERLAY_MEASURE_FPS", "10"))
OVERLAY_MEASURE_MAX_OFFSET_SECONDS = int(os.getenv("OVERLAY_MEASURE_MAX_OFFSET_SECONDS", "2"))
OVERLAY_MEASURE_MIN_RATIO = float(os.getenv("OVERLAY_MEASURE_MIN_RATIO", "0.85"))
OVERLAY_MEASURE_MIN_SCORE = float(os.getenv("OVERLAY_MEASURE_MIN_SCORE", "0.3"))
OVERLAY_MEASURE_MIN_OVERLAP_FRAMES = int(os.getenv("OVERLAY_MEASURE_MIN_OVERLAP_FRAMES", "30"))
OVERLAY_MEASURE_MIN_MOVING_FRAMES = int(os.getenv("OVERLAY_MEASURE_MIN_MOVING_FRAMES", "20"))
# The paired streams are frame-aligned, so a real overlay offset is a fraction of
# a frame. Anything larger is the camera's own clock lag, which both streams share
# and the overlay cannot correct, so it must not be written into the row.
OVERLAY_MEASURE_MAX_ACCEPT_MS = int(os.getenv("OVERLAY_MEASURE_MAX_ACCEPT_MS", "500"))
OVERLAY_MEASURE_WIDTH = 160
OVERLAY_MEASURE_HEIGHT = 90

# A track that moves less than this fraction of the frame diagonal is static.
TRACK_STATIC_DISPLACEMENT = float(os.getenv("TRACK_STATIC_DISPLACEMENT", "0.06"))

# The API writes the JSONB summaries on a 66k-row table under concurrent load, so the
# per-segment PATCH regularly needs more than ten seconds.
API_TIMEOUT = int(os.getenv("API_TIMEOUT", "60"))

# A claim is a lease: while it holds, no other worker can take the row. The
# lease has to outlast one segment of processing, so a worker on a slow (CPU)
# segment is not interrupted by a second worker starting the same row.
CLAIM_LEASE_SECONDS = int(os.getenv("CLAIM_LEASE_SECONDS", "1200"))

# A segment whose file cannot be decoded is not work, and re-claiming it every
# lease expiry is a loop, not a retry. It is quarantined as `failed` with a
# lease this far out: `failed` is not claimable, fix.sql does not requeue it,
# and prune deletes the row together with the file at 7 days.
UNOPENABLE_LEASE_SECONDS = int(os.getenv("UNOPENABLE_LEASE_SECONDS", str(7 * 24 * 60 * 60)))


class UnopenableSegment(ValueError):
    """The claimed segment's file is there and will not decode (ffmpeg killed mid-write: no moov atom)."""


class Sink:
    """Where a handled segment's results go. Subclasses decide: the API, or a local fixture directory."""

    def find_high_res_video(self, video: Video, file_name: str) -> Optional[Video]:
        return None

    def patch_video(self, video: Video, update: Video) -> None:
        pass

    def post_detections(self, detections: list[Detection]) -> None:
        pass

    def post_tracks(self, video: Video, tracks: list["TrackedObject"]) -> dict[str, str]:
        """Write tracks and return the mapping from our track key to the id the rows actually have."""
        return {}

    def recent_tracks(self, video: Video, window_seconds: int) -> list["Seed"]:
        return []

    def camera_overlay_offset_ms(self, camera_id: Optional[str]) -> Optional[int]:
        """The camera's standing offset, used when this segment could not be measured."""
        return None


class ApiSink(Sink):
    def __init__(
        self,
        video_api: VideoApi,
        detection_api: DetectionApi,
        track_api: TrackApi,
        camera_api: Optional[CameraApi] = None,
    ) -> None:
        self.video_api = video_api
        self.detection_api = detection_api
        self.track_api = track_api
        self.camera_api = camera_api
        self.camera_overlay_offsets: dict[str, Optional[int]] = {}

    def find_high_res_video(self, video: Video, file_name: str) -> Optional[Video]:
        # The detector intentionally claims the low-resolution recording, but
        # the frontend displays the paired high-resolution recording. Keep the
        # high-resolution row's status/summary in sync after inference so the
        # normal video list still shows what was detected.
        candidates: list[tuple[Video, datetime.datetime]] = []
        if video.camera_id and video.started_at:
            # The two segment writers can cross a wall-clock second, so
            # their filenames are not always exact siblings. Use the camera
            # plus a minute-level filename prefix, then choose the nearest
            # start time from the small candidate set. Avoid the generated
            # datetime query parameters here: they serialize offsets as +0800
            # while the API parser requires RFC3339.
            start = video.started_at
            parts = file_name.split("_")
            minute_prefix = (
                f"Segment_{parts[1].rsplit(':', 1)[0]}" if len(parts) >= 3 and ":" in parts[1] else None
            )
            if minute_prefix is None:
                return None

            high_res_videos_response = self.video_api.get_videos(
                camera_id__eq=str(video.camera_id),
                is_low_res__eq=False,
                file_name__ilike=minute_prefix,
                limit=10,
                _request_timeout=API_TIMEOUT,
            )
            for candidate in high_res_videos_response.objects or []:
                if not candidate or not candidate.id:
                    continue
                candidate_started_at = candidate.started_at
                if candidate_started_at is not None:
                    candidates.append((candidate, candidate_started_at))

            if candidates:
                closest_candidate, _ = min(
                    candidates,
                    key=lambda candidate_and_start: abs(
                        (candidate_and_start[1] - start).total_seconds()
                    ),
                )
                return closest_candidate

        # Keep the exact filename lookup as a fallback for older or
        # manually-created rows that lack usable timing metadata.
        if not file_name.endswith("_low_res.mp4"):
            return None
        high_res_file_name = f"{file_name[:-len('_low_res.mp4')]}.mp4"
        high_res_videos_response = self.video_api.get_videos(
            file_name__eq=high_res_file_name,
            limit=1,
            _request_timeout=API_TIMEOUT,
        )
        return next(
            (
                candidate
                for candidate in (high_res_videos_response.objects or [])
                if candidate and candidate.id
            ),
            None,
        )

    def patch_video(self, video: Video, update: Video) -> None:
        if not video.id:
            return

        self.video_api.patch_video(
            video.id,
            update,
            _request_timeout=API_TIMEOUT,
        )

    def post_detections(self, detections: list[Detection]) -> None:
        if not detections:
            print("no detections to post")
            return

        print(f"posting {len(detections)} detections")

        self.detection_api.post_detections(
            detections,
            _request_timeout=API_TIMEOUT,
        )

    def post_tracks(self, video: Video, tracks: list["TrackedObject"]) -> dict[str, str]:
        if not tracks:
            return {}

        models = [track_model(t) for t in tracks]

        print(f"posting {len(models)} tracks")

        keys = [str(model.key) for model in models if model.key]
        known = self.track_ids_by_keys(keys)
        missing = [model for model in models if model.key and str(model.key) not in known]

        if missing:
            try:
                self.track_api.post_tracks(
                    missing,
                    _request_timeout=API_TIMEOUT,
                )
            except Exception as e:
                # A second worker can claim the same key between our lookup and
                # this insert, and the bulk write fails as a whole. Write what is
                # left one at a time - whoever's key landed wins, and the
                # read-back below resolves the row id either way.
                print(f"bulk track post failed ({e}); writing tracks individually")

                for model in missing:
                    try:
                        self.track_api.post_tracks(
                            [model],
                            _request_timeout=API_TIMEOUT,
                        )
                    except Exception:
                        pass

        return self.track_ids_by_keys(keys)

    def track_ids_by_keys(self, keys: list[str]) -> dict[str, str]:
        """Tracker keys mapped to the ids of the rows that already hold them."""
        if not keys:
            return {}

        try:
            response = self.track_api.get_tracks(
                key__in=",".join(keys),
                limit=min(len(keys), 500),
                _request_timeout=API_TIMEOUT,
            )
        except Exception as e:
            print(f"could not load track ids: {e}")
            return {}

        return {
            str(track.key): str(track.id)
            for track in (response.objects or [])
            if track.key and track.id
        }

    def camera_overlay_offset_ms(self, camera_id: Optional[str]) -> Optional[int]:
        if not camera_id:
            return None

        if camera_id in self.camera_overlay_offsets:
            return self.camera_overlay_offsets[camera_id]

        offset_ms: Optional[int] = None

        if self.camera_api is not None:
            try:
                response = self.camera_api.get_camera(primary_key=camera_id, _request_timeout=API_TIMEOUT)
                cameras = getattr(response, "objects", None) or []
                offset_ms = cameras[0].overlay_offset_ms if cameras else None
            except Exception as e:
                print(f"could not load overlay offset for camera {camera_id}: {e}")

        self.camera_overlay_offsets[camera_id] = offset_ms

        return offset_ms

    def recent_tracks(self, video: Video, window_seconds: int) -> list["Seed"]:
        if not video.camera_id or not video.started_at:
            return []

        since = video.started_at - datetime.timedelta(seconds=window_seconds)

        try:
            response = self.track_api.get_tracks(
                camera_id__eq=str(video.camera_id),
                # No datetime filter: the generated client serializes offsets as
                # +0800 and the API parser rejects that, so the window is applied
                # here after pulling the most recent tracks for the camera.
                ended_at__desc="",
                limit=TRACK_SEED_LIMIT,
                _request_timeout=API_TIMEOUT,
            )
        except Exception as e:
            print(f"could not load recent tracks for {video.file_name}: {e}")
            return []

        return [
            seed
            for track in (response.objects or [])
            if track.ended_at and track.ended_at >= since
            and (seed := seed_from_track(track)) is not None
        ]


class FixtureSink(Sink):
    """Writes everything a handled segment would send to the API into `<video id>.json`."""

    def __init__(self, out_dir: str) -> None:
        self.out_dir = out_dir
        os.makedirs(out_dir, exist_ok=True)

    def _write(self, video_id: str, record: dict[str, Any]) -> None:
        with open(os.path.join(self.out_dir, f"{video_id}.json"), "w") as file:
            json.dump(record, file, indent=1)

    def _record(self, video_id: str) -> dict[str, Any]:
        path = os.path.join(self.out_dir, f"{video_id}.json")
        if not os.path.exists(path):
            return {}
        with open(path) as file:
            return json.load(file)

    def patch_video(self, video: Video, update: Video) -> None:
        if not video.id:
            return

        record = self._record(video.id)
        record.update(update.model_dump(mode="json", exclude_none=True))
        record["file_name"] = video.file_name
        record["camera_id"] = video.camera_id
        self._write(video.id, record)

    def post_detections(self, detections: list[Detection]) -> None:
        video_id = str(detections[0].video_id) if detections else "no-detections"

        record = self._record(video_id)
        record["detections"] = [
            detection.model_dump(mode="json", exclude_none=True) for detection in detections
        ]
        self._write(video_id, record)

    def post_tracks(self, video: Video, tracks: list["TrackedObject"]) -> dict[str, str]:
        if not tracks:
            return {}

        record = self._record(str(video.id))
        record["tracks"] = [track_model(t).model_dump(mode="json", exclude_none=True) for t in tracks]
        self._write(str(video.id), record)

        return {track.id: track.id for track in tracks}

    def recent_tracks(self, video: Video, window_seconds: int) -> list["Seed"]:
        if not video.started_at:
            return []

        since = video.started_at - datetime.timedelta(seconds=window_seconds)

        seeds: list[Seed] = []

        for path in glob.glob(os.path.join(self.out_dir, "*.json")):
            if path.endswith(f"{video.id}.json"):
                continue

            with open(path) as file:
                record = json.load(file)

            if record.get("camera_id") != video.camera_id:
                continue

            for item in record.get("tracks", []):
                seed = seed_from_track(item)

                if seed and seed.last_seen >= since:
                    seeds.append(seed)

        return seeds


model: Optional[YOLO] = None


def build_predict_options() -> dict[str, Any]:
    """Inference knobs from the environment: CONF, IMGSZ (640 or 384x640), DEVICE, HALF.

    Defaults are the measured dev configuration: CUDA, native substream geometry, and
    the confidence that cleared the side-gate temporal noise.
    """

    options: dict[str, Any] = {}

    conf = os.getenv("CONF", "0.33").strip()
    if conf:
        options["conf"] = float(conf)

    imgsz = os.getenv("IMGSZ", "384x640").strip()
    if imgsz:
        match = re.fullmatch(r"(\d+)x(\d+)", imgsz)
        options["imgsz"] = [int(match.group(1)), int(match.group(2))] if match else int(imgsz)

    device = os.getenv("DEVICE", "cuda").strip()
    if device:
        options["device"] = device

    half = os.getenv("HALF", "").strip()
    if half:
        options["half"] = half == "1"

    batch = os.getenv("BATCH", "").strip()
    if batch:
        options["batch"] = int(batch)

    return options


predict_options_cache: dict[str, Any] | None = None


def predict_options() -> dict[str, Any]:
    # Resolved when a run starts, not at import: tests set their own environment and
    # the container image must not bake the knobs in at build time.
    global predict_options_cache

    if predict_options_cache is None:
        predict_options_cache = build_predict_options()

    return predict_options_cache


def get_model() -> YOLO:
    """Load the weights once per process and reuse them for every claimed segment."""

    global model

    if model is None:
        model = YOLO(os.getenv("MODEL_PATH", "/srv/yolo26n.pt"))

    return model


def do(
    sink: Sink,
    videos: list[Video],
    source_path: str,
):
    for video in videos:
        if not video or not video.id:
            continue

        handle_video(sink, video, source_path)


def quarantine_unopenable(sink: Sink, video: Video, high_res_video: Optional[Video]) -> None:
    """Take an undecodable segment out of the claim pool without hiding why.

    The caller re-raises, so the failure stays in the log. `failed` is the status
    the UI already shows as an error and the detector never claims, and the lease
    stamp is the quarantine: fix.sql leaves `failed` alone, so the pair stays
    visible until prune deletes the rows with the files.
    """

    update = Video(
        status="failed",
        object_detector_claimed_until=datetime.datetime.now().replace(tzinfo=UTC)
        + datetime.timedelta(seconds=UNOPENABLE_LEASE_SECONDS),
    )

    sink.patch_video(video, update)

    if high_res_video and high_res_video.id:
        sink.patch_video(high_res_video, update)


def handle_video(sink: Sink, video: Video, source_path: str) -> None:
    print(f"claimed {video.file_name} - {video.status} - {video.started_at} - {video.duration}")

    before = datetime.datetime.now()

    if video.file_name is None:
        return

    video_file_name: str = cast(str, video.file_name)

    high_res_video: Optional[Video] = sink.find_high_res_video(video, video_file_name)
    high_res_file_name: Optional[str] = (
        f"{video_file_name[:-len('_low_res.mp4')]}.mp4" if video_file_name.endswith("_low_res.mp4") else None
    )

    overlay_offset_ms: Optional[int] = None
    high_res_path = os.path.join(source_path, high_res_file_name) if high_res_file_name is not None else None
    high_res_exists = high_res_path is not None and os.path.exists(high_res_path)

    sink.patch_video(
        video,
        Video(status="detecting"),
    )

    try:
        detections_by_frame, handled_frame_count, frame_width, frame_height = infer(
            video, os.path.join(source_path, video_file_name)
        )
    except UnopenableSegment:
        # Loud in the log (the raise continues) and loud in the UI (`failed`
        # renders as an error), and out of the pool the claim filter reads.
        quarantine_unopenable(sink, video, high_res_video)
        raise

    tracker = Tracker(
        width=frame_width,
        height=frame_height,
        min_hits=TRACK_MIN_HITS,
        max_age=TRACK_MAX_AGE,
        match_score=TRACK_MATCH_SCORE,
        seed=sink.recent_tracks(video, TRACK_CONTINUITY_SECONDS),
    )

    for frame_ms, frame_detections in detections_by_frame:
        tracker.update(video, frame_ms, frame_detections)

    tracks = tracker.emit()

    # Measured after tracking so the window can sit on the moving objects: a
    # static stretch of two encoders carries no alignment signal at all.
    if high_res_exists and high_res_path is not None:
        fps = handled_frame_count / (video.duration / 1_000_000_000) if video.duration else 0.0
        start_frame = pick_measure_window(detections_by_frame, tracks, fps)

        if start_frame is not None:
            overlay_offset_ms = measure_overlay_offset_ms(
                high_res_path, os.path.join(source_path, video_file_name), start_frame
            )

    if overlay_offset_ms is None:
        overlay_offset_ms = sink.camera_overlay_offset_ms(video.camera_id)

    detections = [detection for _, frame_detections in detections_by_frame for detection in frame_detections]

    # A box that never joined a track long enough to survive the noise gate is
    # left as a bare detection: the raw detections stay honest, the objects do not.
    tracked_ids = {track.id for track in tracks}

    # The API assigns each track row its own id, so detections are stamped with
    # the row ids that the keys resolve to, not with the tracker's own ids.
    track_ids = sink.post_tracks(video, tracks)

    for detection in detections:
        detection.track_id = track_ids.get(detection.track_id) if detection.track_id in tracked_ids else None

    sink.post_detections(detections)

    before_summarise = datetime.datetime.now()

    detection_summary = summarise(detections, handled_frame_count)
    track_summary = summarise_tracks(tracks)

    before_request = datetime.datetime.now()

    # The low-res segment can close just before its high-res sibling. Retry
    # the lookup after inference so the final status/summary update also
    # repairs that small race.
    if high_res_file_name is not None and high_res_video is None:
        high_res_video = sink.find_high_res_video(video, video_file_name)

    detection_video_update = Video(
        status="needs tracking",
        detection_summary=detection_summary,
        track_summary=track_summary,
        overlay_offset_ms=overlay_offset_ms,
        object_tracker_claimed_until=datetime.datetime.now().replace(tzinfo=UTC),
    )

    sink.patch_video(video, detection_video_update)

    if high_res_video and high_res_video.id:
        sink.patch_video(high_res_video, detection_video_update)

    after = datetime.datetime.now()

    print(
        f"{video.file_name} handled in {after - before} (summarise took {after - before_summarise}, request took {after - before_request}, overlay offset {overlay_offset_ms})"
    )


def frame_stream(cap: cv2.VideoCapture, timestamps: list[float]) -> Any:
    """Yield decoded frames for ultralytics to batch, recording each frame's media timestamp."""

    while cap.isOpened():
        success, frame = cap.read()
        if not success:
            break

        timestamps.append(cap.get(cv2.CAP_PROP_POS_MSEC))

        yield frame


def infer(
    video: Video, filename: str
) -> tuple[list[tuple[float, list[Detection]]], int, int, int]:
    """Turn each decoded frame into its detections immediately, then drop the frame.

    Returns the frames (media timestamp, detections), the frame count, and the frame
    size, so tracking can associate boxes across frames.

    BATCH defaults to frame-by-frame inference; BATCH>1 hands the decoder to
    ultralytics as a generator so it can batch frames. Only CUDA should decide
    whether that is worth it.
    """

    detections_by_frame: list[tuple[float, list[Detection]]] = []
    handled_frame_count = 0

    batch = int(os.getenv("BATCH", "0"))

    cap: cv2.VideoCapture = cv2.VideoCapture(filename)

    if not cap.isOpened():
        # A segment that cannot be decoded is not a segment with no detections.
        # Marking it "needs tracking" hides the failure, so fail loudly instead.
        # The weights load after this check: an undecodable file should not pay
        # for the model it never uses.
        raise UnopenableSegment(f"cannot open {filename}")

    model = get_model()

    try:
        frame_width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
        frame_height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))

        if batch > 1:
            timestamps: list[float] = []

            for result in cast(
                Any,
                model(
                    frame_stream(cap, timestamps),
                    stream=True,
                    verbose=debug,
                    **predict_options(),
                ),
            ):
                frame_detections: list[Detection] = []

                add_detections(video, result, timestamps[handled_frame_count], frame_detections)

                detections_by_frame.append((timestamps[handled_frame_count], frame_detections))

                handled_frame_count += 1
        else:
            while cap.isOpened():
                success, frame = cap.read()
                if not success:
                    break

                raw_timedelta = cap.get(cv2.CAP_PROP_POS_MSEC)

                # The detector reads the low-resolution recording, so infer every
                # decoded frame rather than decimating the temporal signal again.
                results = cast(
                    list[Results],
                    model(
                        frame,
                        verbose=debug,
                        **predict_options(),
                    ),
                )

                frame_detections = []

                for result in results:
                    add_detections(video, result, raw_timedelta, frame_detections)

                detections_by_frame.append((raw_timedelta, frame_detections))

                handled_frame_count += 1
    finally:
        cap.release()

    return detections_by_frame, handled_frame_count, frame_width, frame_height


def measure_sample(path: str, start_frame: int = 0) -> Optional[np.ndarray]:
    capture = cv2.VideoCapture(path)

    if not capture.isOpened():
        return None

    source_fps = capture.get(cv2.CAP_PROP_FPS)

    if not source_fps or source_fps <= 0:
        source_fps = float(OVERLAY_MEASURE_FPS)

    step = max(1, int(round(source_fps / OVERLAY_MEASURE_FPS)))
    frames: List[np.ndarray] = []

    # Frames are counted out, not seeked to: frame seeking is not dependable
    # across decoders, and decoding the skipped frames is cheap.
    for index in range(start_frame + OVERLAY_MEASURE_SECONDS * int(source_fps) + step):
        ok, frame = capture.read()

        if not ok:
            break

        if index < start_frame:
            continue

        if (index - start_frame) % step == 0:
            resized = cv2.resize(frame, (OVERLAY_MEASURE_WIDTH, OVERLAY_MEASURE_HEIGHT))
            frames.append(cv2.cvtColor(resized, cv2.COLOR_BGR2GRAY).astype(np.float64))

    capture.release()

    if len(frames) < OVERLAY_MEASURE_SECONDS * OVERLAY_MEASURE_FPS // 2:
        return None

    return np.array(frames)


def correlation_score(high: np.ndarray, low: np.ndarray, offset_frames: int) -> float:
    start = max(0, -offset_frames)
    end = min(high.shape[0], low.shape[0] - offset_frames)

    if end - start < OVERLAY_MEASURE_MIN_OVERLAP_FRAMES:
        return float("inf")

    return float(np.mean([abs(high[i] - low[i + offset_frames]).mean() for i in range(start, end)]))


def measure_content_offset_ms(high: np.ndarray, low: np.ndarray) -> Optional[int]:
    max_frames = OVERLAY_MEASURE_MAX_OFFSET_SECONDS * OVERLAY_MEASURE_FPS
    scores = [correlation_score(high, low, offset) for offset in range(-max_frames, max_frames + 1)]
    best_index = int(np.argmin(scores))
    best = scores[best_index]
    direct = scores[max_frames]

    # A best at the edge of the search is the search running out of room, not a
    # match: the real alignment is outside what this measures.
    if best_index == 0 or best_index == len(scores) - 1:
        return None

    if direct < OVERLAY_MEASURE_MIN_SCORE or best >= direct * OVERLAY_MEASURE_MIN_RATIO:
        return None

    offset_ms = int(round((best_index - max_frames) * 1000 / OVERLAY_MEASURE_FPS))
    if abs(offset_ms) > OVERLAY_MEASURE_MAX_ACCEPT_MS:
        return None

    return offset_ms


def measure_overlay_offset_ms(high_path: str, low_path: str, start_frame: int = 0) -> Optional[int]:
    high = measure_sample(high_path, start_frame)
    low = measure_sample(low_path, start_frame)

    if high is None or low is None:
        return None

    return measure_content_offset_ms(high, low)


def pick_measure_window(
    detections_by_frame: list[tuple[float, list[Detection]]],
    tracks: list["TrackedObject"],
    fps: float,
) -> Optional[int]:
    # An empty scene has no correlation to find, so the window is chosen where
    # the tracked moving objects actually are, not where the file starts.
    moving_ids = {track.id for track in tracks if not track.is_static}

    if not moving_ids or fps <= 0:
        return None

    moving = [
        1 if any(detection.track_id in moving_ids for detection in frame_detections) else 0
        for _, frame_detections in detections_by_frame
    ]

    window = int(OVERLAY_MEASURE_SECONDS * fps)

    if window <= 0 or window > len(moving):
        window = len(moving)

    if window <= 0:
        return None

    total = sum(moving[:window])
    best_start = 0
    running = total

    for start in range(1, len(moving) - window + 1):
        running += moving[start + window - 1] - moving[start - 1]

        if running > total:
            total = running
            best_start = start

    if total < OVERLAY_MEASURE_MIN_MOVING_FRAMES:
        return None

    return best_start


def add_detections(
    video: Video, result: Results, raw_timedelta: float, detections: list[Detection]
) -> None:
    if not result.boxes:
        return

    timedelta = datetime.timedelta(milliseconds=raw_timedelta)

    for box in result.boxes:  # should be one box per result (because stream=True)
        class_id = [int(v) for v in box.cls][0]  # type: ignore
        class_name = cast(str, result.names[class_id])
        confidence = [float(v) for v in box.conf][0]  # type: ignore

        for raw_xyxy in box.xyxy:  # type: ignore
            ltx, lty, rbx, rby = [float(v) for v in raw_xyxy]

            polygon = [
                (ltx, lty),  # left top
                (rbx, lty),  # right top
                (rbx, rby),  # right bottom
                (ltx, rby),  # left bottom
                (
                    ltx,
                    lty,
                ),  # left top again (postgres likes its polygons closed off)
            ]

            bounding_box = [
                DetectionBoundingBoxInner(
                    X=p[0],
                    Y=p[1],
                )
                for p in polygon
            ]

            cx, cy, _, _ = [float(v) for v in box.xywh[0]]  # type: ignore

            centroid = DetectionBoundingBoxInner(
                X=cx,
                Y=cy,
            )

            if debug:
                print(f"{class_name} ({class_id}) @ {confidence} {[(ltx, lty), (rbx, rby)]} {(cx, cy)}")

            detection = Detection(
                class_id=class_id,
                class_name=class_name,
                score=confidence,
                seen_at=video.started_at + timedelta if video.started_at else None,
                bounding_box=bounding_box,
                centroid=centroid,
                video_id=video.id,
                camera_id=video.camera_id,
            )

            detections.append(detection)


@dataclass
class Seed:
    """The last known state of a track from an earlier segment."""

    id: str
    class_id: int
    class_name: str
    bbox: Tuple[float, float, float, float]
    centroid: Tuple[float, float]
    first_seen: datetime.datetime
    last_seen: datetime.datetime
    mean_score: float = 0.0
    max_score: float = 0.0


@dataclass
class TrackedObject:
    id: str
    camera_id: str
    video_id: str
    class_id: int
    class_name: str
    first_seen: datetime.datetime
    last_seen: datetime.datetime
    hits: int
    age: int
    bbox: Tuple[float, float, float, float]
    centroid: Tuple[float, float]
    first_centroid: Tuple[float, float]
    scores: list[float] = field(default_factory=list)
    inherited: Optional[Tuple[float, float]] = None
    path: list[dict[str, Any]] = field(default_factory=list)
    widest_bbox: Tuple[float, float, float, float] = (0.0, 0.0, 0.0, 0.0)
    is_static: bool = False


def xy(point: Any) -> Tuple[float, float]:
    """Read a stored point from a generated model, an API JSON dict, or a fixture JSON dict."""

    if isinstance(point, dict):
        x = point.get("x", point.get("X"))
        y = point.get("y", point.get("Y"))

        if x is None or y is None:
            raise ValueError(f"stored point has no coordinates: {point}")

        return float(x), float(y)

    if point.x is None or point.y is None:
        raise ValueError(f"stored point has no coordinates: {point}")

    return float(point.x), float(point.y)


def box_of(polygon: Any) -> Optional[Tuple[float, float, float, float]]:
    """Axis-aligned box from a stored closed polygon ring."""

    points = [xy(p) for p in polygon]

    if len(points) < 4:
        return None

    return (
        min(p[0] for p in points),
        min(p[1] for p in points),
        max(p[0] for p in points),
        max(p[1] for p in points),
    )


def box_iou(a: Tuple[float, float, float, float], b: Tuple[float, float, float, float]) -> float:
    ix0, iy0 = max(a[0], b[0]), max(a[1], b[1])
    ix1, iy1 = min(a[2], b[2]), min(a[3], b[3])

    intersection = max(0.0, ix1 - ix0) * max(0.0, iy1 - iy0)

    area_a = max(0.0, a[2] - a[0]) * max(0.0, a[3] - a[1])
    area_b = max(0.0, b[2] - b[0]) * max(0.0, b[3] - b[1])

    union = area_a + area_b - intersection

    return intersection / union if union > 0 else 0.0


def match_score(
    a_bbox: Tuple[float, float, float, float],
    a_centroid: Tuple[float, float],
    b_bbox: Tuple[float, float, float, float],
    b_centroid: Tuple[float, float],
    width: int,
    height: int,
) -> float:
    """IoU plus how close the centres are, scaled by the frame diagonal.

    Closeness matters on a fixed camera because a box can move a few pixels per
    frame while IoU barely changes, and IoU alone lets a fast object hand its
    identity over to a neighbour.
    """

    diagonal = math.hypot(width, height)

    if not diagonal:
        return 0.0

    distance = math.hypot(a_centroid[0] - b_centroid[0], a_centroid[1] - b_centroid[1]) / diagonal

    closeness = max(0.0, 1.0 - distance / 0.25)

    return box_iou(a_bbox, b_bbox) + 0.5 * closeness


def track_id_for(camera_id: Optional[str], class_name: str, first_seen: datetime.datetime) -> str:
    return str(uuid.uuid5(TRACK_NAMESPACE, f"{camera_id}|{class_name}|{first_seen.isoformat()}"))


def seed_from_track(track: Any) -> Optional[Seed]:
    """Rebuild a track's last state from an API row or a fixture record."""

    def get_field(name: str) -> Any:
        return getattr(track, name, None) if not isinstance(track, dict) else track.get(name)

    track_id = get_field("key") or get_field("id")
    bounding_box = get_field("bounding_box")
    centroid = get_field("centroid")
    class_name = get_field("class_name")
    started_at = get_field("started_at")
    ended_at = get_field("ended_at")

    if not track_id or not bounding_box or centroid is None or not class_name:
        return None

    bbox = box_of(bounding_box)

    if bbox is None:
        return None

    if isinstance(started_at, str):
        started_at = datetime.datetime.fromisoformat(started_at)
    if isinstance(ended_at, str):
        ended_at = datetime.datetime.fromisoformat(ended_at)

    if not started_at or not ended_at:
        return None

    return Seed(
        id=str(track_id),
        class_id=int(get_field("class_id") or 0),
        class_name=str(class_name),
        bbox=bbox,
        centroid=xy(centroid),
        first_seen=started_at,
        last_seen=ended_at,
        mean_score=float(get_field("mean_score") or 0.0),
        max_score=float(get_field("max_score") or 0.0),
    )


def track_from_seed(seed: Seed) -> TrackedObject:
    """Carry a previous segment's track into this one so identity survives the cut."""

    return TrackedObject(
        id=seed.id,
        camera_id="",
        video_id="",
        class_id=seed.class_id,
        class_name=seed.class_name,
        first_seen=seed.first_seen,
        last_seen=seed.last_seen,
        hits=TRACK_MIN_HITS,
        age=0,
        bbox=seed.bbox,
        centroid=seed.centroid,
        first_centroid=seed.centroid,
        inherited=(seed.mean_score, seed.max_score),
        widest_bbox=seed.bbox,
    )


class Tracker:
    """Greedy IoU-plus-closeness tracker (SORT-shaped) over the frames of one segment."""

    def __init__(
        self,
        width: int,
        height: int,
        min_hits: int,
        max_age: int,
        match_score: float,
        seed: Optional[list[Seed]] = None,
    ) -> None:
        self.width = width
        self.height = height
        self.min_hits = min_hits
        self.max_age = max_age
        self.match_score = match_score
        self.tracks: list[TrackedObject] = [track_from_seed(seed) for seed in (seed or [])]

        # A track that stops matching mid-segment is finished, not garbage: it
        # has to survive to the end of the segment to be written.
        self.retired: list[TrackedObject] = []

    def update(self, video: Video, frame_ms: float, detections: list[Detection]) -> None:
        candidates = [
            detection
            for detection in detections
            if detection.class_name in TRACKABLE_CLASSES and detection.bounding_box and detection.centroid
        ]

        pairs: list[tuple[float, int, Detection]] = []

        for index, track in enumerate(self.tracks):
            for detection in candidates:
                if detection.class_name != track.class_name:
                    continue

                score = match_score(
                    track.bbox,
                    track.centroid,
                    cast(Tuple[float, float, float, float], box_of(detection.bounding_box)),
                    xy(detection.centroid),
                    self.width,
                    self.height,
                )

                if score >= self.match_score:
                    pairs.append((score, index, detection))

        pairs.sort(key=lambda pair: -pair[0])

        matched_tracks: set[int] = set()
        matched_detections: set[int] = set()

        for _score, index, detection in pairs:
            if index in matched_tracks or id(detection) in matched_detections:
                continue

            matched_tracks.add(index)
            matched_detections.add(id(detection))

            self._continue(video, self.tracks[index], detection, frame_ms)

        for detection in candidates:
            if id(detection) not in matched_detections:
                self.tracks.append(self._spawn(video, detection, frame_ms))

        for index, track in enumerate(self.tracks):
            if index not in matched_tracks:
                track.age += 1

        alive: list[TrackedObject] = []

        for track in self.tracks:
            if track.age <= self.max_age:
                alive.append(track)
            else:
                # A track that stops matching mid-segment is finished, not
                # garbage: it has to survive to the end of the segment to be written.
                self.retired.append(track)

        self.tracks = alive

    def emit(self) -> list[TrackedObject]:
        # A seeded track that was never seen again in this segment is not this
        # segment's to write; its own segment already recorded it.
        tracks = [
            track
            for track in self.tracks + self.retired
            if track.hits >= self.min_hits and track.video_id
        ]

        diagonal = math.hypot(self.width, self.height)

        for track in tracks:
            displacement = math.hypot(
                track.centroid[0] - track.first_centroid[0],
                track.centroid[1] - track.first_centroid[1],
            )

            track.is_static = bool(diagonal) and displacement <= TRACK_STATIC_DISPLACEMENT * diagonal

        return tracks

    def _continue(self, video: Video, track: TrackedObject, detection: Detection, frame_ms: float) -> None:
        bbox = cast(Tuple[float, float, float, float], box_of(detection.bounding_box))
        centroid = xy(detection.centroid)

        track.bbox = bbox
        track.centroid = centroid
        track.hits += 1
        track.age = 0

        # A track carried in from an earlier segment belongs to this segment's
        # record as soon as it is seen here.
        track.video_id = str(video.id)
        track.camera_id = str(video.camera_id)

        if detection.score is not None:
            track.scores.append(float(detection.score))

        if detection.seen_at:
            track.last_seen = detection.seen_at

        if (bbox[2] - bbox[0]) * (bbox[3] - bbox[1]) > (track.widest_bbox[2] - track.widest_bbox[0]) * (
            track.widest_bbox[3] - track.widest_bbox[1]
        ):
            track.widest_bbox = bbox

        track.path.append(
            {
                "seen_at": detection.seen_at.isoformat() if detection.seen_at else None,
                "x": centroid[0],
                "y": centroid[1],
                "bbox": list(bbox),
            }
        )

        detection.track_id = track.id

    def _spawn(self, video: Video, detection: Detection, frame_ms: float) -> TrackedObject:
        bbox = cast(Tuple[float, float, float, float], box_of(detection.bounding_box))
        centroid = xy(detection.centroid)

        seen_at = detection.seen_at or video.started_at or datetime.datetime.now(tz=UTC)

        track = TrackedObject(
            id=track_id_for(video.camera_id, str(detection.class_name), seen_at),
            camera_id=str(video.camera_id),
            video_id=str(video.id),
            class_id=int(detection.class_id or 0),
            class_name=str(detection.class_name),
            first_seen=seen_at,
            last_seen=seen_at,
            hits=1,
            age=0,
            bbox=bbox,
            centroid=centroid,
            first_centroid=centroid,
            scores=[float(detection.score)] if detection.score is not None else [],
            path=[
                {
                    "seen_at": seen_at.isoformat(),
                    "x": centroid[0],
                    "y": centroid[1],
                    "bbox": list(bbox),
                }
            ],
            widest_bbox=bbox,
        )

        detection.track_id = track.id

        return track


def polygon_of(bbox: Tuple[float, float, float, float]) -> list[DetectionBoundingBoxInner]:
    ltx, lty, rbx, rby = bbox

    return [
        DetectionBoundingBoxInner(X=x, Y=y)
        for x, y in [
            (ltx, lty),  # left top
            (rbx, lty),  # right top
            (rbx, rby),  # right bottom
            (ltx, rby),  # left bottom
            (ltx, lty),  # left top again (postgres likes its polygons closed off)
        ]
    ]


def track_mean_score(track: TrackedObject) -> float:
    """Average score over this segment's hits, blended with what earlier segments saw."""

    if not track.scores:
        return track.inherited[0] if track.inherited else 0.0

    here = sum(track.scores) / len(track.scores)

    return (here + track.inherited[0]) / 2 if track.inherited else here


def track_max_score(track: TrackedObject) -> float:
    here = max(track.scores) if track.scores else 0.0

    return max(here, track.inherited[1]) if track.inherited else here


def track_model(track: TrackedObject) -> Track:
    # `key` is the deterministic identity the tracker derives (camera + class +
    # first sighting), so a re-run of a segment re-issues the same key. The API
    # generates `id` itself, so the row's id is looked up by key afterwards.
    return Track(
        key=track.id,
        camera_id=track.camera_id,
        video_id=track.video_id,
        class_id=track.class_id,
        class_name=track.class_name,
        started_at=track.first_seen,
        ended_at=track.last_seen,
        frame_count=track.hits,
        mean_score=track_mean_score(track),
        max_score=track_max_score(track),
        centroid=DetectionBoundingBoxInner(X=track.centroid[0], Y=track.centroid[1]),
        bounding_box=polygon_of(track.widest_bbox),
        trajectory=track.path,
        is_static=track.is_static,
        status="tracking",
    )


def summarise_tracks(tracks: list[TrackedObject]) -> list[dict[str, Any]]:
    """Per-class object counts for the video list, the way detection_summary is per-class counts of frames."""

    if not tracks:
        return []

    by_class: Dict[str, list[TrackedObject]] = {}

    for track in tracks:
        by_class.setdefault(track.class_name, []).append(track)

    summary = [
        {
            "class_name": class_name,
            "count": len(these),
            "moving_count": sum(1 for t in these if not t.is_static),
            "mean_score": round(sum(track_mean_score(t) for t in these) / len(these), 2),
            "max_score": round(max(track_max_score(t) for t in these), 2),
            "frame_count": sum(t.hits for t in these),
        }
        for class_name, these in by_class.items()
    ]

    return sorted(summary, reverse=True, key=lambda item: item["count"])


def summarise(detections: list[Detection], handled_frame_count: int) -> list[dict[str, Any]]:
    detection_summary: list[dict[str, Any]] = []

    if not detections:
        return detection_summary

    detections_by_class_name: Dict[str, List[Detection]] = dict()
    for detection in detections:
        if not detection.class_name:
            continue

        detections_by_class_name.setdefault(detection.class_name, []).append(detection)

    detection_summary_by_class_name: Dict[str, Tuple[float, float, int]] = dict()
    for class_name, these_detections in detections_by_class_name.items():
        scores = [x.score for x in these_detections if x.score is not None]
        average_score = sum(scores) / len(scores) if scores else 0.0
        frame_count = len(these_detections)

        weighted_score = (
            ((average_score * frame_count) / float(handled_frame_count))
            if handled_frame_count != 0
            else 0.0
        )

        item = (weighted_score, average_score, frame_count)

        # Keep every class the model detected in the summary. Filtering
        # low-confidence or sparse classes here made the summary disagree with
        # the raw detections and hid legitimate person detections from the UI
        # search. Temporal noise belongs in tracking, not here.
        print(f"keeping   : {class_name} {item}")
        detection_summary_by_class_name[class_name] = item

    raw_detection_summary = sorted(
        list(detection_summary_by_class_name.items()), reverse=True, key=lambda x: x[1][0]
    )

    detection_summary.extend(
        [
            {
                "class_name": class_name,
                "weighted_score": round(weighted_score, 2),
                "average_score": round(average_score, 2),
                "detected_frame_count": frame_count,
                "handled_frame_count": handled_frame_count,
            }
            for (class_name, (weighted_score, average_score, frame_count)) in raw_detection_summary
        ]
    )

    return detection_summary



def claim_videos(
    object_detector_claim_video_api: ObjectDetectorClaimVideoApi,
) -> list[Video]:
    print("waiting to claim a video... ", end="")

    now = datetime.datetime.now().astimezone(UTC)

    until = now + datetime.timedelta(seconds=CLAIM_LEASE_SECONDS)

    try:
        resp = object_detector_claim_video_api.post_object_detector_claim_videos(
            video_object_detector_claim_request=VideoObjectDetectorClaimRequest(until=until, timeout_seconds=10),
            is_low_res__eq=True,
            # Segment producers leave files as "recording" until ffmpeg
            # closes them and writes the MP4 moov atom.
            #
            # "detecting" is claimable too: the endpoint only hands out rows whose
            # lease has expired, so this recovers work abandoned mid-inference
            # within CLAIM_LEASE_SECONDS instead of waiting for prune's daily fix.
            status__in="needs detection,detecting",
            started_at__asc="",
            _request_timeout=70,
        )
        return resp.objects or []
    except Exception as e:
        if "wanted exactly 1 unclaimed video, got 0" in str(e):
            print("no videos available to claim.")
            return []

        print("")
        raise


def videos_by_file_name(video_api: VideoApi, file_name: str) -> list[Video]:
    videos_response = video_api.get_videos(
        file_name__eq=file_name,
        started_at__desc="",
        _request_timeout=API_TIMEOUT,
    )

    return videos_response.objects or []


def load_fixture_videos(fixture_dir: str) -> list[Video]:
    """Load `video-N.json` rows from a fixture directory, in segment order."""

    paths = [
        path
        for path in glob.glob(os.path.join(fixture_dir, "video-*.json"))
        if re.search(r"video-\d+\.json$", path)
    ]

    def index(path: str) -> int:
        match = re.search(r"video-(\d+)\.json$", path)
        return int(match.group(1)) if match else 0

    videos: list[Video] = []

    for path in sorted(paths, key=index):
        with open(path) as file:
            video = Video.from_dict(json.load(file))
        if not video:
            continue
        # The fixture holds the segment as video-N.mp4, so point the row at
        # the local file rather than the name it has on the server.
        video.file_name = os.path.basename(re.sub(r"\.json$", ".mp4", path))
        videos.append(video)

    return videos


def run():
    api_url = os.getenv("API_URL")
    source_path = os.getenv("SOURCE_PATH")

    one_shot_file_name = os.getenv("ONE_SHOT_FILE_NAME", "").strip() or None

    fixture_dir = os.getenv("FIXTURE_DIR", "").strip() or None

    if fixture_dir:
        # SOURCE_PATH is the deployment's media directory; a fixture brings its
        # own path, and FIXTURE_SOURCE_PATH overrides it for a fixture whose
        # mp4s live elsewhere.
        source_path = os.getenv("FIXTURE_SOURCE_PATH", "").strip() or fixture_dir

        sink: Sink = FixtureSink(os.getenv("FIXTURE_OUT_DIR", "").strip() or os.path.join(fixture_dir, "out"))

        videos = load_fixture_videos(fixture_dir)

        if not videos:
            raise ValueError(f"no video-N.json rows found in {fixture_dir}")

        do(sink, videos, cast(str, source_path))

        return

    if not api_url:
        raise ValueError("API_URL env var empty or unset")
    if not source_path:
        raise ValueError("SOURCE_PATH env var empty or unset")

    configuration = Configuration(host=api_url, debug=debug)
    api_client = ApiClient(configuration)
    # The API closes idle keep-alive connections and urllib3 reuses them
    # regardless, so a request written to a half-closed socket waits out its
    # entire timeout. One connection per call costs milliseconds here.
    api_client.set_default_header("Connection", "close")
    video_api = VideoApi(api_client)
    detection_api = DetectionApi(api_client)
    track_api = TrackApi(api_client)
    custom_api = ObjectDetectorClaimVideoApi(api_client)
    camera_api = CameraApi(api_client)

    sink = ApiSink(video_api, detection_api, track_api, camera_api)

    while 1:
        try:
            if one_shot_file_name:
                videos = videos_by_file_name(video_api, one_shot_file_name)
            else:
                videos = claim_videos(custom_api)

            if videos:
                do(sink, videos, source_path)

            if one_shot_file_name:
                break
        except Exception as e:
            print(f"attempt to execute {do} raised {e}; traceback follows:\n{traceback.format_exc()}")
        except KeyboardInterrupt:
            print("exiting...")
            return

        if not one_shot_file_name:
            time.sleep(1)


if __name__ == "__main__":
    pass
