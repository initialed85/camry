"""Fetch a segment (plus its live detections and camera) from a Camry API into a local fixture.

Usage:
  object_detector/.venv/bin/python object_detector/tools/fetch_fixture.py --video-id <uuid>
  ... --file-name Segment_..._low_res.mp4 [--segments 3] [--name my_fixture]

Environment:
  CAMRY_BASE_URL (default https://camry.dev.initialed85.cc)
  CAMRY_TOKEN    bearer token (GET /api/custom/login with the password to obtain one)
"""

import argparse
import datetime
import json
import os
import re
import sys
import urllib.parse
import urllib.request

default_base_url = "https://camry.dev.initialed85.cc"


def get_json(base_url: str, token: str, path: str, params: dict[str, str]) -> dict:
    url = f"{base_url}{path}?{urllib.parse.urlencode(params)}"

    request = urllib.request.Request(url)
    if token:
        request.add_header("Authorization", f"Bearer {token}")

    with urllib.request.urlopen(request, timeout=30) as response:
        return json.load(response)


def get_objects(base_url: str, token: str, path: str, params: dict[str, str]) -> list[dict]:
    objects: list[dict] = []

    offset = 0

    while True:
        page = get_json(base_url, token, path, params | {"offset": str(offset)})

        objects.extend(page.get("objects") or [])

        total_count = int(page.get("total_count") or 0)

        if len(objects) >= total_count or not page.get("objects"):
            return objects

        offset += len(page["objects"])


def get_video(base_url: str, token: str, video_id: str, file_name: str) -> dict:
    params = {"id__eq": video_id} if video_id else {"file_name__eq": file_name}

    objects = get_objects(base_url, token, "/api/videos", params)

    if not objects:
        raise ValueError(f"no video found for {params}")

    return objects[0]


def get_low_res_video(base_url: str, token: str, video: dict) -> dict:
    if video.get("is_low_res"):
        return video

    started_at = video.get("started_at")

    if not started_at:
        raise ValueError("high-res video has no started_at to match a low-res sibling on")

    start = datetime.datetime.fromisoformat(started_at)

    candidates = get_objects(
        base_url,
        token,
        "/api/videos",
        {
            "camera_id__eq": str(video.get("camera_id")),
            "is_low_res__eq": "true",
            "started_at__gt": (start - datetime.timedelta(minutes=5)).isoformat(),
            "started_at__lt": (start + datetime.timedelta(minutes=5)).isoformat(),
        },
    )

    if not candidates:
        raise ValueError(f"no low-res sibling found for {video.get('file_name')}")

    # The two segment writers can cross a wall-clock second, so match on the
    # nearest start time rather than on the filename.
    def seconds_from_start(candidate: dict) -> float:
        return abs(
            (datetime.datetime.fromisoformat(str(candidate.get("started_at"))) - start).total_seconds()
        )

    return min(candidates, key=seconds_from_start)


def get_camera(base_url: str, token: str, camera_id: str) -> dict:
    objects = get_objects(base_url, token, "/api/cameras", {"id__eq": camera_id})

    if not objects:
        raise ValueError(f"no camera found for {camera_id}")

    return objects[0]


def download(base_url: str, token: str, file_name: str, destination: str) -> int:
    request = urllib.request.Request(f"{base_url}/media/{urllib.parse.quote(file_name)}")
    if token:
        request.add_header("Authorization", f"Bearer {token}")

    with urllib.request.urlopen(request, timeout=60) as response, open(destination, "wb") as file:
        total = 0
        while True:
            chunk = response.read(1 << 16)
            if not chunk:
                break
            file.write(chunk)
            total += len(chunk)

    return total


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--video-id", default="")
    parser.add_argument("--file-name", default="")
    parser.add_argument("--fixtures-dir", default="fixtures")
    parser.add_argument("--name", default="")
    parser.add_argument(
        "--segments",
        type=int,
        default=1,
        help="fetch this many consecutive low-res segments from the same camera (for cross-segment tracking tests)",
    )
    args = parser.parse_args()

    base_url = os.getenv("CAMRY_BASE_URL", default_base_url).rstrip("/")
    token = os.getenv("CAMRY_TOKEN", "")

    if not token:
        print("CAMRY_TOKEN is empty: fetch the token from POST /api/custom/login first.", file=sys.stderr)
        return 1

    if not args.video_id and not args.file_name:
        print("pass --video-id or --file-name", file=sys.stderr)
        return 1

    video = get_low_res_video(base_url, token, get_video(base_url, token, args.video_id, args.file_name))

    camera = get_camera(base_url, token, str(video.get("camera_id")))

    started_at = datetime.datetime.fromisoformat(str(video.get("started_at")))

    name = args.name or f"{re.sub(r'[^a-zA-Z0-9]+', '_', str(camera.get('name'))).strip('_')}_{started_at.strftime('%Y-%m-%dT%H_%M')}"

    fixture_dir = os.path.join(args.fixtures_dir, name)

    os.makedirs(fixture_dir, exist_ok=True)

    with open(os.path.join(fixture_dir, "camera.json"), "w") as file:
        json.dump(camera, file, indent=1)

    rows = [video]

    if args.segments > 1:
        following = get_objects(
            base_url,
            token,
            "/api/videos",
            {
                "camera_id__eq": str(video.get("camera_id")),
                "is_low_res__eq": "true",
                "started_at__gt": started_at.isoformat(),
                "started_at__lt": (started_at + datetime.timedelta(minutes=args.segments * 2)).isoformat(),
                "started_at__asc": "",
            },
        )
        rows.extend(following[: args.segments - 1])

    for index, row in enumerate(rows):
        file_name = str(row.get("file_name"))

        size = download(base_url, token, file_name, os.path.join(fixture_dir, f"video-{index}.mp4"))

        with open(os.path.join(fixture_dir, f"video-{index}.json"), "w") as file:
            json.dump(row, file, indent=1)

        detections = get_objects(
            base_url,
            token,
            "/api/detections",
            {"video_id__eq": str(row.get("id")), "seen_at__asc": ""},
        )

        with open(os.path.join(fixture_dir, f"video-{index}.detections.json"), "w") as file:
            json.dump(detections, file)

        print(f"video-{index}: {file_name} ({size} bytes, {len(detections)} live detections)")

    print(f"fixture: {fixture_dir} ({len(rows)} segment(s))")

    return 0


if __name__ == "__main__":
    sys.exit(main())
