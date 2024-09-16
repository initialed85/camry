import traceback
import cv2
import os
import time
import datetime

from atexit import register
from typing import Dict, List, Tuple
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
    DetectionBoundingBoxInner,
    CustomApi,
    ClaimRequest,
)


def cleanup():
    cv2.destroyAllWindows()


register(cleanup)

debug = os.getenv("DEBUG", "") == "1"


def do(
    video_api: VideoApi,
    detection_api: DetectionApi,
    custom_api: CustomApi,
    source_path: str,
    one_shot_video_file_name: str | None = None,
):
    videos = []

    if one_shot_video_file_name:
        videos_response = video_api.get_videos(
            file_name__eq=one_shot_video_file_name,
            started_at__desc="",
        )

        videos = videos_response.objects
    else:
        print("waiting to claim a video... ", end="")

        try:
            video = custom_api.patch_claim_video_for_object_detector(
                claim_request=ClaimRequest(
                    claim_duration_seconds=60,  # TODO: inject this as an env var
                )
            )
            videos = [video]
        except Exception as e:
            if "wanted exactly 1 unclaimed video, got 0" in str(e):
                print("no videos available to claim.")
                return

            print("")

            raise

    if not videos:
        return

    for video in videos:
        print(f"claimed {video.file_name} - {video.status} - {video.started_at} - {video.duration}")

        before = datetime.datetime.now()

        if one_shot_video_file_name:
            detections_response = detection_api.get_detections(
                camera_id__eq=video.camera_id,
                video_id__eq=video.id,
            )

            for detection in detections_response.objects or []:
                print(f"deleting old detection {detection.id}")
                detection_api.delete_detection(detection.id)

        if video.file_name is None:
            continue

        video_api.patch_video(video.id, Video(status="detecting"))

        model = YOLO("yolov8n.pt")

        try:

            frame_index_and_timedelta_and_results: List[Tuple[int, datetime.timedelta, List[Results]]] = []

            def do_inference():
                if video.file_name is None:
                    return 0

                filename = os.path.join(source_path, video.file_name)

                cap: cv2.VideoCapture = cv2.VideoCapture(filename)

                try:
                    frame_index = -1
                    handled_frame_count = 0

                    while cap.isOpened():
                        frame_index += 1

                        success, frame = cap.read()
                        if not success:
                            break

                        raw_timedelta = cap.get(cv2.CAP_PROP_POS_MSEC)

                        if frame_index % 4 != 0:
                            continue

                        results: List[Results] = model(
                            frame,
                            verbose=debug,
                        )

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

            handle_results()

            def post_detections():
                if detections:
                    print(f"posting {len(detections)} detections")

                    detection_api.post_detections(
                        detections,
                    )
                else:
                    print("no detections to post")

                print(f"updating video")

                before_summarise = datetime.datetime.now()

                detection_summary = []

                if detections:
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
                        detection_summary_by_class_name[class_name] = (weighted_score, average_score, frame_count)

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

                    for item in detection_summary:
                        print(item)

                before_request = datetime.datetime.now()

                video_api.patch_video(
                    video.id,
                    Video(
                        status="needs tracking",
                        detection_summary=detection_summary,
                        object_tracker_claimed_until=datetime.datetime.now().replace(tzinfo=UTC),
                    ),
                )

                after = datetime.datetime.now()

                print(
                    f"{video.file_name} handled in {after - before} (summarise took {after - before_summarise}, request took {after - before_request})"
                )

            post_detections()

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
    custom_api = CustomApi(api_client)

    while 1:
        try:
            do(video_api, detection_api, custom_api, source_path, one_shot_file_name)

            if one_shot_file_name:
                break
        except Exception as e:
            print(f"attempt to execute {do} raised {e}; traceback follows:\n{traceback.format_exc()}")
        except KeyboardInterrupt:
            print("exiting...")
            return

        time.sleep(1)


if __name__ == "__main__":
    pass
