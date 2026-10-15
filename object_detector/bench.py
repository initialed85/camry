"""Run the detector over a local fixture and record the numbers that matter.

Usage:
  object_detector/.venv/bin/python object_detector/bench.py --fixture fixtures/front_door_2026-10-14T21_45 [--label baseline]

Environment forwarded to the detector run:
  FIXTURE_DIR, FIXTURE_OUT_DIR, MODEL_PATH, DEVICE, IMGSZ, CONF, ... (whatever the detector reads)
"""

import argparse
import glob
import json
import os
import resource
import subprocess
import sys
import time

repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def child_peak_rss_bytes() -> int:
    # macOS reports bytes here, Linux reports kilobytes.
    peak = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
    return peak if sys.platform == "darwin" else peak * 1024


def load_result_records(out_dir: str) -> list[dict]:
    records = []
    for path in sorted(glob.glob(os.path.join(out_dir, "*.json"))):
        with open(path) as file:
            records.append(json.load(file))
    return records


def live_row_count(fixture_dir: str) -> int:
    total = 0
    for path in glob.glob(os.path.join(fixture_dir, "video-*.detections.json")):
        with open(path) as file:
            total += len(json.load(file))
    return total


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixture", required=True)
    parser.add_argument("--label", default="current")
    parser.add_argument("--out", default="")
    parser.add_argument("--model", default=os.getenv("MODEL_PATH", "yolo26n.pt"))
    parser.add_argument("--jsonl", default="")
    args = parser.parse_args()

    out_dir = args.out or os.path.join(args.fixture, "out")

    if os.path.isdir(out_dir):
        for path in glob.glob(os.path.join(out_dir, "*.json")):
            os.remove(path)

    env = os.environ | {
        "FIXTURE_DIR": args.fixture,
        "FIXTURE_OUT_DIR": out_dir,
        "MODEL_PATH": args.model,
    }

    before = time.time()
    completed = subprocess.run(
        [sys.executable, "-u", "-m", "object_detector"],
        cwd=repo_root,
        env=env,
        stdout=sys.stdout,
        stderr=sys.stderr,
    )
    wall_seconds = time.time() - before
    peak_rss_bytes = child_peak_rss_bytes()

    if completed.returncode != 0:
        print(f"detector run failed with exit code {completed.returncode}", file=sys.stderr)
        return completed.returncode

    records = load_result_records(out_dir)

    detections = sum(len(record.get("detections") or []) for record in records)
    frames = sum(
        (record.get("detection_summary") or [{}])[0].get("handled_frame_count", 0) for record in records
    )
    classes: dict[str, int] = {}
    for record in records:
        for detection in record.get("detections") or []:
            classes[str(detection.get("class_name"))] = classes.get(str(detection.get("class_name")), 0) + 1

    live_rows = live_row_count(args.fixture)

    record = {
        "label": args.label,
        "fixture": os.path.basename(args.fixture),
        "model": args.model,
        "segments": len(records),
        "wall_seconds": round(wall_seconds, 2),
        "frames": frames,
        "ms_per_frame": round(wall_seconds * 1000 / frames, 2) if frames else None,
        "peak_rss_mb": round(peak_rss_bytes / (1024 * 1024), 1),
        "detections": detections,
        "live_detections": live_rows,
        "top_classes": dict(sorted(classes.items(), key=lambda x: x[1], reverse=True)[:5]),
    }

    print(json.dumps(record, indent=1))

    jsonl_path = args.jsonl or os.path.join(args.fixture, "bench.jsonl")
    with open(jsonl_path, "a") as file:
        file.write(json.dumps(record) + "\n")

    return 0


if __name__ == "__main__":
    sys.exit(main())
