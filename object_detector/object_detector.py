import os
import time
import datetime

from typing import List, cast

from ultralytics import YOLO
from ultralytics.models.yolo.detect.predict import Results

from .openapi_client import (
    ApiClient,
    Configuration,
    CameraApi,
    Camera,
    VideoApi,
    Video,
    DetectionApi,
    Detection,
    DetectionBoundingBoxInner,
)

debug = os.getenv("DEBUG", "") == "1"


def do(camera: Camera, video_api: VideoApi, detection_api: DetectionApi, one_shot_video_file_name: str | None = None):
    videos_response = None

    if one_shot_video_file_name:
        videos_response = video_api.get_videos(
            file_name__eq=one_shot_video_file_name,
            started_at__desc="",
            shallow="",
        )
    else:
        videos_response = video_api.get_videos(
            camera_id__eq=camera.id,
            status__eq="needs detection",
            started_at__desc="",
            shallow="",
        )

    videos = videos_response.objects or []

    if not videos:
        return

    for video in videos:
        print(f"{video.file_name} - {video.status} - {video.started_at} - {video.duration}")

        before = datetime.datetime.now()

        if one_shot_video_file_name:
            detections_response = detection_api.get_detections(
                camera_id__eq=camera.id,
                video_id__eq=video.id,
                shallow="",
            )

            for detection in detections_response.objects or []:
                print(f"deleting old detection {detection.id}")
                detection_api.delete_detection(detection.id)

        video_api.patch_video(video.id, Video(status="detecting"))

        model = YOLO("yolov8n.pt")
        results = model(
            f"media/{video.file_name}",
            stream=True,  # this causes the results variable to be a generator
            vid_stride=4,
            verbose=debug,
        )

        detections: List[Detection] = []

        for result in cast(Results, results):

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
                        (ltx, lty),  # left top again (postgres likes its polygons closed off)
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
                        seen_at=video.started_at,  # TODO: need to tie this to video time somehow- could mean big refactor
                        bounding_box=bounding_box,
                        centroid=centroid,
                        video_id=video.id,
                        camera_id=video.camera_id,
                    )

                    detections.append(detection)

        print(f"posting {len(detections)} detections")

        detection_api.post_detections(
            detections,
            shallow="",
        )

        print(f"updating video")

        before_request = datetime.datetime.now()

        video_api.patch_video(
            video.id,
            Video(status="needs tracking"),
            shallow="",
        )

        after = datetime.datetime.now()

        print(f"{video.file_name} handled in {after - before} (request took {after - before_request})")

        if one_shot_video_file_name:
            break


def run():
    net_cam_url = os.getenv("NET_CAM_URL")
    if not net_cam_url:
        raise ValueError("NET_CAM_URL env var empty or unset")

    camera_name = os.getenv("CAMERA_NAME")
    if not camera_name:
        raise ValueError("CAMERA_NAME env var empty or unset")

    api_url = os.getenv("API_URL")
    if not api_url:
        raise ValueError("API_URL env var empty or unset")

    one_shot_file_name = os.getenv("ONE_SHOT_FILE_NAME", "").strip() or None

    configuration = Configuration(host=api_url, debug=debug)
    api_client = ApiClient(configuration)
    camera_api = CameraApi(api_client)
    video_api = VideoApi(api_client)
    detection_api = DetectionApi(api_client)

    cameras_response = camera_api.get_cameras(
        stream_url__eq=net_cam_url,
        name__eq=camera_name,
        shallow="",
    )

    cameras = cameras_response.objects or []

    if not cameras or len(cameras) != 1:
        raise ValueError(
            f"failed to find exactly 1 camera for NET_CAM_URL={repr(net_cam_url)} CAMERA_NAME={repr(camera_name)}"
        )

    camera = cameras[0]

    print(f"camera: {camera.id}, {camera.name}, {camera.stream_url}")

    while 1:
        try:
            do(camera, video_api, detection_api, one_shot_file_name)

            if one_shot_file_name:
                break
        except KeyboardInterrupt:
            print("exiting...")
            return

        time.sleep(1)


if __name__ == "__main__":
    pass
