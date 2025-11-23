import sys
import os

sys.path.append(os.path.join(os.getcwd(), "object_detector"))
sys.path.append(os.path.join(os.getcwd(), "object_detector", "api"))

import traceback
import cv2
import os
import time
import datetime
import sys

from atexit import register
from typing import Dict, List, Optional, Tuple
from pytz import UTC
from ultralytics import YOLO
from ultralytics.models.yolo.detect.predict import Results

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
)


def cleanup():
    cv2.destroyAllWindows()


register(cleanup)

debug = os.getenv("DEBUG", "") == "1"


def do(
    video_api: VideoApi,
    detection_api: DetectionApi,
    object_detector_claim_video_api: ObjectDetectorClaimVideoApi,
    source_path: str,
    one_shot_video_file_name: Optional[str] = None,
):
    one_shot_video_file_name = sys.argv[1]

    videos = [
        Video(
            camera_id=None,
            camera_id_object=None,
            created_at=None,
            deleted_at=None,
            detection_summary=None,
            duration=None,
            ended_at=None,
            file_name=one_shot_video_file_name,
            file_size=None,
            id=None,
            object_detector_claimed_until=None,
            object_tracker_claimed_until=None,
            referenced_by_detection_video_id_objects=None,
            started_at=None,
            status=None,
            thumbnail_name=None,
            updated_at=None,
        )
    ]

    for video in videos:
        print(f"claimed {video.file_name} - {video.status} - {video.started_at} - {video.duration}")

        model = YOLO("yolov8n.pt")

        try:

            frame_index_and_timedelta_and_results: List[Tuple[int, datetime.timedelta, List[Results]]] = []

            def do_inference():
                if video.file_name is None:
                    return 0

                filename = video.file_name

                cap: cv2.VideoCapture = cv2.VideoCapture(filename)

                try:
                    frame_index = -1
                    handled_frame_count = 0

                    while cap.isOpened():
                        frame_index += 1

                        before = time.time()
                        success, frame = cap.read()
                        after = time.time()
                        print(f"reading frame took {(after - before) * 1000}ms")

                        if not success:
                            break

                        raw_timedelta = cap.get(cv2.CAP_PROP_POS_MSEC)

                        # 0, 4, 8, 12, 16 etc- so I guess we're 25% of the original frame rate
                        if frame_index % 2 != 0:
                            continue

                        before = time.time()
                        results: List[Results] = model(
                            frame,
                            verbose=debug,
                            max_det=100,
                            iou=0.1,
                        )
                        after = time.time()
                        print(f"inferencing took {(after - before) * 1000}ms")

                        frame_index_and_timedelta_and_results.append(
                            (
                                frame_index,
                                datetime.timedelta(milliseconds=raw_timedelta),
                                results,
                            )
                        )

                        handled_frame_count += 1
                finally:
                    cap.release()

                return handled_frame_count

            handled_frame_count = do_inference()

            detections: List[Detection] = []

            def handle_results():
                for frame_index, timedelta, results in frame_index_and_timedelta_and_results:
                    for result in results:
                        for box in result.boxes or []:  # should be one box per result (because stream=True)
                            class_id = [int(v) for v in box.cls][0]
                            class_name = result.names[class_id]
                            confidence = [float(v) for v in box.conf][0]

                            for raw_xyxy in box.xyxy:
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

                                cx, cy, _, _ = [float(v) for v in box.xywh[0]]

                                centroid = DetectionBoundingBoxInner(
                                    X=cx,
                                    Y=cy,
                                )

                                # print(f"{class_name} ({class_id}) @ {confidence} {[(ltx, lty), (rbx, rby)]} {(cx, cy)}")

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

            handle_results()

            if one_shot_video_file_name:
                break
        finally:
            del model


def run():
    api_url = os.getenv("API_URL")
    if not api_url:
        raise ValueError("API_URL env var empty or unset")

    source_path = os.getenv("SOURCE_PATH")
    if not source_path:
        raise ValueError("SOURCE_PATH env var empty or unset")

    one_shot_file_name = os.getenv("ONE_SHOT_FILE_NAME", "").strip() or None

    configuration = Configuration(host=api_url, debug=debug)
    api_client = ApiClient(configuration)
    video_api = VideoApi(api_client)
    detection_api = DetectionApi(api_client)
    custom_api = ObjectDetectorClaimVideoApi(api_client)

    do(video_api, detection_api, custom_api, source_path, one_shot_file_name)


if __name__ == "__main__":
    run()
