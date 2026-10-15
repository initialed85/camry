"""Regression checks for the detector's result sinks.

Run from the repo root:
  object_detector/.venv/bin/python -m unittest object_detector.tests
"""

import datetime
import glob
import json
import os
import sys
import tempfile
import unittest
from unittest.mock import MagicMock, patch

import cv2
import numpy as np
from pytz import UTC

# The generated client imports itself as `openapi_client`, the same path
# `object_detector/__main__.py` adds before importing the package.
sys.path.append(os.path.join(os.path.dirname(os.path.abspath(__file__)), "api"))

from .api.openapi_client import ArrayOfVec2Inner, Detection, Track, Video
from .object_detector import (
    OVERLAY_MEASURE_FPS,
    OVERLAY_MEASURE_SECONDS,
    TRACK_SEED_LIMIT,
    ApiSink,
    FixtureSink,
    TrackedObject,
    Tracker,
    do,
    load_fixture_videos,
    measure_content_offset_ms,
    measure_sample,
    pick_measure_window,
    seed_from_track,
    summarise_tracks,
    track_model,
)

weights_file = "yolo26n.pt"

segment_name = "Segment_2026-10-14T21:43:00_cam-id_low_res.mp4"

sibling_name = "Segment_2026-10-14T21:43:01_cam-id.mp4"

started_at = datetime.datetime(2026, 10, 14, 21, 43, 0, tzinfo=UTC)


def write_segment(directory: str, name: str, frames: int = 10) -> str:
    path = os.path.join(directory, name)

    writer = cv2.VideoWriter(path, cv2.VideoWriter.fourcc(*"mp4v"), 20, (320, 240))

    for index in range(frames):
        writer.write(np.full((240, 320, 3), index * 20, dtype=np.uint8))

    writer.release()

    return path


def make_video(id: str, file_name: str, is_low_res: bool) -> Video:
    return Video(
        id=id,
        file_name=file_name,
        camera_id="cam-id",
        is_low_res=is_low_res,
        started_at=started_at,
        status="needs detection",
    )


def make_track(track_id: str, ended_at: datetime.datetime, key: str | None = None) -> Track:

    point = lambda x, y: ArrayOfVec2Inner.model_validate({"x": x, "y": y})



    return Track(

        id=track_id,

        key=key,

        camera_id="cam-id",

        video_id="low-id",

        class_id=0,

        class_name="person",

        started_at=ended_at - datetime.timedelta(seconds=5),

        ended_at=ended_at,

        frame_count=10,

        mean_score=0.5,

        max_score=0.6,

        centroid=point(10.0, 20.0),

        bounding_box=[

            point(0.0, 0.0),

            point(10.0, 0.0),

            point(10.0, 10.0),

            point(0.0, 10.0),

        ],

        trajectory=[],

        is_static=False,

        status="tracking",

    )

class DetectorTest(unittest.TestCase):
    def setUp(self):
        if not os.path.exists(weights_file):
            self.skipTest(f"{weights_file} must be present in the working directory")

        # Production defaults point at the container's model and GPU; the tests run
        # against the local weights on CPU.
        env = patch.dict(
            os.environ,
            {"MODEL_PATH": weights_file, "DEVICE": "cpu", "IMGSZ": "384x640", "CONF": "0.33"},
        )
        env.start()
        self.addCleanup(env.stop)

        self.directory = tempfile.mkdtemp()
        write_segment(self.directory, segment_name)

        self.low_res = make_video("low-id", segment_name, True)

        self.high_res = make_video("high-id", sibling_name, False)

    def test_live_run_patches_both_rows_and_posts_once(self):
        video_api = MagicMock()
        video_api.get_videos.return_value = MagicMock(objects=[self.high_res])

        detection_api = MagicMock()

        posted: list[int] = []

        class RecordingApiSink(ApiSink):
            def post_detections(self, detections: list[Detection]) -> None:
                posted.append(len(detections))
                super().post_detections(detections)

        do(RecordingApiSink(video_api, detection_api, MagicMock()), [self.low_res], self.directory)

        patches = [call.args for call in video_api.patch_video.call_args_list]

        self.assertEqual(len(patches), 3)
        self.assertEqual(patches[0][1].status, "detecting")
        self.assertEqual(patches[-1][1].status, "needs tracking")
        self.assertEqual({patch[0] for patch in patches}, {"low-id", "high-id"})

        self.assertEqual(posted, [0])

        detection_api.post_detections.assert_not_called()

    def test_live_sink_posts_detections_and_skips_empty_batches(self):
        video_api = MagicMock()
        detection_api = MagicMock()

        sink = ApiSink(video_api, detection_api, MagicMock())

        sink.post_detections([])
        detection_api.post_detections.assert_not_called()

        detection = Detection(
            class_id=0,
            class_name="person",
            score=0.9,
            seen_at=datetime.datetime(2026, 10, 14, 21, 43, 0, tzinfo=UTC),
            video_id="low-id",
            camera_id="cam-id",
        )

        sink.post_detections([detection])
        detection_api.post_detections.assert_called_once()

        sink.patch_video(Video(status="detecting"), Video(status="detecting"))
        video_api.patch_video.assert_not_called()

    def test_recent_tracks_use_a_camera_window_not_a_datetime_query(self):
        track_api = MagicMock()
        track_api.get_tracks.return_value = MagicMock(
            objects=[
                make_track("inside", datetime.datetime(2026, 10, 14, 21, 43, 30, tzinfo=UTC)),
                make_track("outside", datetime.datetime(2026, 10, 14, 19, 0, 0, tzinfo=UTC)),
            ],
        )

        sink = ApiSink(MagicMock(), MagicMock(), track_api)

        seeds = sink.recent_tracks(self.low_res, 180)

        self.assertEqual([seed.id for seed in seeds], ["inside"])

        kwargs = track_api.get_tracks.call_args.kwargs

        self.assertNotIn("started_at__gt", kwargs)
        self.assertEqual(kwargs["limit"], TRACK_SEED_LIMIT)

    def test_fixture_run_writes_the_same_result_without_an_api(self):
        out_dir = os.path.join(self.directory, "out")

        sink = FixtureSink(out_dir)

        do(sink, [self.low_res], self.directory)

        with open(os.path.join(out_dir, "low-id.json")) as file:
            record = json.load(file)

        self.assertEqual(record["status"], "needs tracking")
        self.assertEqual(record["file_name"], segment_name)
        self.assertIn("detection_summary", record)
        self.assertIn("no-detections.json", [os.path.basename(path) for path in glob.glob(os.path.join(out_dir, "*.json"))])

    def test_real_fixture_reproduces_the_live_detection_count(self):
        """Opt-in equivalence check: CAMRY_TEST_FIXTURE=1 with a fetched fixture present."""

        if os.getenv("CAMRY_TEST_FIXTURE", "") != "1":
            self.skipTest("set CAMRY_TEST_FIXTURE=1 to run the live-fixture equivalence check")

        fixture_dirs = [
            directory
            for directory in glob.glob(os.path.join("fixtures", "*"))
            if os.path.isdir(directory)
            and os.path.exists(os.path.join(directory, "video-0.json"))
            and os.path.exists(os.path.join(directory, "video-0.detections.json"))
        ]

        if not fixture_dirs:
            self.skipTest("no fetched fixture found under fixtures/")

        fixture_dir = sorted(fixture_dirs)[0]

        with open(os.path.join(fixture_dir, "video-0.detections.json")) as file:
            live_detections = json.load(file)

        posted: list[int] = []

        videos = [video for video in load_fixture_videos(fixture_dir) if video.id == live_detections[0]["video_id"]]

        self.assertEqual(len(videos), 1)

        video_api = MagicMock()
        video_api.get_videos.return_value = MagicMock(objects=[])

        detection_api = MagicMock()
        detection_api.post_detections.side_effect = lambda detections, **kwargs: posted.append(len(detections))

        track_api = MagicMock()

        do(ApiSink(video_api, detection_api, track_api), videos, fixture_dir)

        self.assertEqual(posted, [len(live_detections)])

    def test_undecodable_segment_fails_loudly(self):
        video_api = MagicMock()
        video_api.get_videos.return_value = MagicMock(objects=[])

        with self.assertRaises(ValueError):
            do(ApiSink(video_api, MagicMock(), MagicMock()), [self.low_res], os.path.join(self.directory, "missing"))

    def test_fixture_rows_are_loaded_in_segment_order_and_point_at_local_files(self):
        fixture_dir = os.path.join(self.directory, "fixture")
        os.makedirs(fixture_dir)

        for index in (1, 0):
            write_segment(fixture_dir, f"video-{index}.mp4")
            with open(os.path.join(fixture_dir, f"video-{index}.json"), "w") as file:
                json.dump(
                    {
                        "id": f"id-{index}",
                        "file_name": segment_name,
                        "camera_id": "cam-id",
                        "is_low_res": True,
                        "started_at": "2026-10-14T21:43:00+08:00",
                        "status": "needs tracking",
                    },
                    file,
                )

        with open(os.path.join(fixture_dir, "video-0.detections.json"), "w") as file:
            json.dump([], file)

        videos = load_fixture_videos(fixture_dir)

        self.assertEqual([video.id for video in videos], ["id-0", "id-1"])
        self.assertEqual([video.file_name for video in videos], ["video-0.mp4", "video-1.mp4"])

    def test_camera_overlay_offset_is_read_once_and_cached(self):
        camera_api = MagicMock()
        camera_api.get_camera.return_value = MagicMock(objects=[MagicMock(overlay_offset_ms=-400)])
        sink = ApiSink(MagicMock(), MagicMock(), MagicMock(), camera_api)

        self.assertEqual(sink.camera_overlay_offset_ms("cam-id"), -400)
        self.assertEqual(sink.camera_overlay_offset_ms("cam-id"), -400)
        self.assertEqual(camera_api.get_camera.call_count, 1)

    def test_missing_camera_api_leaves_the_offset_unset(self):
        sink = ApiSink(MagicMock(), MagicMock(), MagicMock())

        self.assertIsNone(sink.camera_overlay_offset_ms("cam-id"))


class TrackingTest(unittest.TestCase):
    """The tracker's own behaviour: noise gates, identity, continuity, determinism."""

    video = make_video("v-id", segment_name, True)

    def box(self, class_name: str, x0: float, y0: float, x1: float, y1: float, ms: float) -> Detection:
        seen_at = started_at + datetime.timedelta(milliseconds=ms)

        return Detection(
            class_id=0,
            class_name=class_name,
            score=0.8,
            seen_at=seen_at,
            bounding_box=[
                ArrayOfVec2Inner(X=x, Y=y)
                for x, y in [(x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0)]
            ],
            centroid=ArrayOfVec2Inner(X=(x0 + x1) / 2, Y=(y0 + y1) / 2),
            video_id="v-id",
            camera_id="cam-id",
        )

    def tracker(self) -> Tracker:
        return Tracker(width=320, height=240, min_hits=4, max_age=10, match_score=0.45)

    def run_frames(self, frames: list[list[Detection]]) -> Tracker:
        tracker = self.tracker()

        for index, detections in enumerate(frames):
            tracker.update(self.video, index * 50.0, detections)

        return tracker

    def test_a_box_seen_for_one_frame_is_noise_not_an_object(self):
        tracker = self.run_frames([[], [self.box("giraffe", 10, 10, 40, 60, 50)], []])

        self.assertEqual(tracker.emit(), [])

    def test_a_moving_box_keeps_one_identity_across_frames(self):
        frames = [[self.box("person", 10 + 6 * i, 10, 40 + 6 * i, 60, i * 50)] for i in range(10)]

        tracks = self.run_frames(frames).emit()

        self.assertEqual(len(tracks), 1)
        self.assertEqual(tracks[0].hits, 10)
        self.assertEqual({detection.track_id for frame in frames for detection in frame}, {tracks[0].id})

    def test_a_one_frame_class_flip_does_not_become_an_object(self):
        frames = [[self.box("person", 10, 10, 40, 60, i * 50)] for i in range(6)]
        frames.append([self.box("giraffe", 11, 11, 41, 61, 300)])
        frames += [[self.box("person", 12, 12, 42, 62, (4 + i) * 50)] for i in range(6)]

        tracks = self.run_frames(frames).emit()

        self.assertEqual([track.class_name for track in tracks], ["person"])
        self.assertIsNone(frames[6][0].track_id)

    def test_identity_survives_a_segment_boundary(self):
        first = self.run_frames([[self.box("person", 10 + 4 * i, 10, 40 + 4 * i, 60, i * 50)] for i in range(8)]).emit()

        self.assertEqual(len(first), 1)

        seed = seed_from_track(track_model(first[0]))

        if seed is None:
            self.fail("an emitted track should seed the next segment")

        second = Tracker(width=320, height=240, min_hits=4, max_age=10, match_score=0.45, seed=[seed])

        for index, offset in enumerate(range(8, 14)):
            second.update(self.video, index * 50.0, [self.box("person", 42 + 4 * index, 10, 72 + 4 * index, 60, index * 50)])

        tracks = second.emit()

        self.assertEqual(len(tracks), 1)
        self.assertEqual(tracks[0].id, first[0].id)

    def test_the_same_sighting_gets_the_same_id_every_run(self):
        frames = [[self.box("car", 20, 20, 60, 50, i * 50)] for i in range(6)]

        first = self.run_frames([list(frame) for frame in frames]).emit()
        second = self.run_frames([list(frame) for frame in frames]).emit()

        self.assertEqual([track.id for track in first], [track.id for track in second])

    def test_tracks_are_posted_by_key_and_return_the_row_ids_the_api_made(self):
        tracks = self.run_frames(
            [[self.box("person", 10 + 6 * i, 10, 40 + 6 * i, 60, i * 50)] for i in range(10)]
        ).emit()

        ended = datetime.datetime(2026, 10, 14, 21, 43, 30, tzinfo=UTC)

        track_api = MagicMock()
        track_api.get_tracks.side_effect = [
            MagicMock(objects=[]),
            MagicMock(objects=[make_track("row-1", ended, key=tracks[0].id)]),
        ]

        sink = ApiSink(MagicMock(), MagicMock(), track_api)

        self.assertEqual(sink.post_tracks(self.video, tracks), {tracks[0].id: "row-1"})

        model = track_api.post_tracks.call_args.args[0][0]

        self.assertEqual(model.key, tracks[0].id)
        self.assertIsNone(model.id)

    def test_tracks_that_already_exist_are_read_back_and_never_reposted(self):
        tracks = self.run_frames(
            [[self.box("person", 10 + 6 * i, 10, 40 + 6 * i, 60, i * 50)] for i in range(10)]
        ).emit()

        ended = datetime.datetime(2026, 10, 14, 21, 43, 30, tzinfo=UTC)

        track_api = MagicMock()
        track_api.get_tracks.return_value = MagicMock(
            objects=[make_track("row-1", ended, key=tracks[0].id)],
        )

        sink = ApiSink(MagicMock(), MagicMock(), track_api)

        self.assertEqual(sink.post_tracks(self.video, tracks), {tracks[0].id: "row-1"})

        track_api.post_tracks.assert_not_called()

    def test_a_bulk_post_that_loses_a_race_still_stamps_every_track(self):
        tracks = self.run_frames(
            [[self.box("person", 10 + 6 * i, 10, 40 + 6 * i, 60, i * 50)] for i in range(10)]
        ).emit()

        ended = datetime.datetime(2026, 10, 14, 21, 43, 30, tzinfo=UTC)
        key = tracks[0].id
        rival = "11111111-1111-5111-8111-111111111111"

        track_api = MagicMock()
        track_api.get_tracks.side_effect = [
            MagicMock(objects=[make_track("row-1", ended, key=key)]),
            MagicMock(objects=[make_track("row-1", ended, key=key), make_track("row-2", ended, key=rival)]),
        ]
        track_api.post_tracks.side_effect = [
            Exception('duplicate key value violates unique constraint "track_key_key"'),
            MagicMock(),
        ]

        sink = ApiSink(MagicMock(), MagicMock(), track_api)

        self.assertEqual(sink.post_tracks(self.video, tracks), {key: "row-1", rival: "row-2"})

    def test_a_track_that_stops_mid_segment_is_still_written(self):
        frames = [[self.box("car", 20 + 4 * i, 20, 60 + 4 * i, 50, i * 50)] for i in range(8)]
        frames += [[] for _ in range(40)]

        tracks = self.run_frames(frames).emit()

        self.assertEqual(len(tracks), 1)
        self.assertEqual(tracks[0].hits, 8)

    def test_summary_counts_objects_per_class(self):
        frames = [
            [
                self.box("person", 10, 10, 40, 60, i * 50),
                self.box("car", 200, 100, 260, 150, i * 50),
            ]
            for i in range(6)
        ]

        summary = summarise_tracks(self.run_frames(frames).emit())

        self.assertEqual({item["class_name"] for item in summary}, {"person", "car"})
        self.assertTrue(all(item["count"] == 1 for item in summary))


def moving_pattern_frames(count: int, shift: int) -> np.ndarray:
    frames = np.zeros((count, 90, 160), dtype=np.float64)

    for i in range(count):
        x = min(140, max(0, i + shift))
        frames[i, 30:50, x : x + 20] = 255.0

    return frames


class OverlayOffsetTest(unittest.TestCase):
    def test_recovers_a_shifted_content_offset(self):
        # high[i] is the scene at frame i, low[i] is the scene five frames later,
        # so high[i] matches low[i - 5].
        high = moving_pattern_frames(120, 0)
        low = moving_pattern_frames(120, 5)

        self.assertEqual(measure_content_offset_ms(high, low), -500)

    def test_an_alignment_outside_the_search_window_is_not_reported(self):
        # 25 frames is 2.5 s, past the 2 s the search covers: the best score
        # sits at the edge, which is the window running out, not a match.
        high = moving_pattern_frames(120, 0)
        low = moving_pattern_frames(120, 25)

        self.assertIsNone(measure_content_offset_ms(high, low))

    def test_an_alignment_beyond_the_frame_alignment_band_is_not_reported(self):
        # 12 frames is inside the 2 s search, so the score finds it, but the
        # paired streams are frame-aligned: a 1.2 s correction is the camera's
        # clock lag shared by both streams, not something the overlay can fix.
        high = moving_pattern_frames(120, 0)
        low = moving_pattern_frames(120, 12)

        self.assertIsNone(measure_content_offset_ms(high, low))

    def test_a_static_scene_is_not_measurable(self):
        frames = np.full((120, 90, 160), 42.0)

        self.assertIsNone(measure_content_offset_ms(frames, frames.copy()))

    def test_a_scene_with_no_structure_is_not_measurable(self):
        rng = np.random.default_rng(7)
        high = rng.random((120, 90, 160)) * 255
        low = rng.random((120, 90, 160)) * 255

        self.assertIsNone(measure_content_offset_ms(high, low))

    @staticmethod
    def tracked_box(track_id: str, frame_index: int) -> Detection:
        ms = frame_index * 50.0
        detection = Detection(
            class_id=0,
            class_name="person",
            score=0.8,
            seen_at=started_at + datetime.timedelta(milliseconds=ms),
            bounding_box=[
                ArrayOfVec2Inner(X=x, Y=y)
                for x, y in [
                    (frame_index, 10),
                    (frame_index + 20, 10),
                    (frame_index + 20, 60),
                    (frame_index, 60),
                    (frame_index, 10),
                ]
            ],
            centroid=ArrayOfVec2Inner(X=frame_index + 10, Y=35),
            video_id="v-id",
            camera_id="cam-id",
        )
        detection.track_id = track_id

        return detection

    @staticmethod
    def moving_track(track_id: str = "t1", is_static: bool = False) -> TrackedObject:
        return TrackedObject(
            id=track_id,
            camera_id="cam-id",
            video_id="v-id",
            class_id=0,
            class_name="person",
            first_seen=started_at,
            last_seen=started_at,
            hits=10,
            age=0,
            bbox=(0.0, 10.0, 20.0, 60.0),
            centroid=(10.0, 35.0),
            first_centroid=(10.0, 35.0),
            is_static=is_static,
        )

    @staticmethod
    def frames_with_motion(moving: range, total: int) -> list[tuple[float, list[Detection]]]:
        return [
            (index * 50.0, [OverlayOffsetTest.tracked_box("t1", index)] if index in moving else [])
            for index in range(total)
        ]

    def test_the_measure_window_lands_on_the_moving_objects(self):
        # 1,200 frames at 20 fps, motion only in frames 600-699: the 12 s window
        # that contains all of it starts 240 - 100 frames earlier.
        frames = self.frames_with_motion(range(600, 700), 1200)

        self.assertEqual(pick_measure_window(frames, [self.moving_track()], 20.0), 460)

    def test_a_window_needs_enough_moving_frames_to_be_worth_measuring(self):
        frames = self.frames_with_motion(range(600, 615), 1200)

        self.assertIsNone(pick_measure_window(frames, [self.moving_track()], 20.0))

    def test_a_scene_of_only_static_objects_has_no_window(self):
        frames = self.frames_with_motion(range(600, 700), 1200)

        self.assertIsNone(pick_measure_window(frames, [self.moving_track(is_static=True)], 20.0))

    def test_measure_sample_starts_where_it_is_told_to(self):
        path = os.path.join(tempfile.mkdtemp(), "sample.mp4")
        writer = cv2.VideoWriter(path, int.from_bytes(b"mp4v", "little"), 20.0, (160, 90))

        self.assertTrue(writer.isOpened())

        for index in range(400):
            writer.write(np.full((90, 160, 3), 30 if index < 100 else 220, dtype=np.uint8))

        writer.release()

        first_half = measure_sample(path, 0)
        second_half = measure_sample(path, 100)

        if first_half is None or second_half is None:
            self.fail("measure_sample returned no frames")

        self.assertLess(float(first_half[0].mean()), 100.0)
        self.assertGreater(float(second_half[0].mean()), 150.0)
        self.assertLessEqual(len(second_half), OVERLAY_MEASURE_SECONDS * OVERLAY_MEASURE_FPS + 1)


if __name__ == "__main__":
    unittest.main()
