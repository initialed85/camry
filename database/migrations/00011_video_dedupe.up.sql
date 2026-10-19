--
-- collapse the video rows the double-claim produced
--
-- On 2026-10-18/19 two segment producers wrote the Front Door camera at the same time. Postgres
-- restarted at 12:24Z, the cameras' `claimed_until` was in the future, and the segment producer's
-- claim loop only asked "is this lease expired?", so after the lease lapsed two pods believed they
-- owned the same camera. Both ran ffmpeg against the same output pattern, and each inserted its own
-- row for the same file: 1765 groups of 2 rows on one camera between 2026-10-18T12:24Z and
-- 2026-10-19T03:35Z (3648 rows, 1824 files, 116/h).
--
-- The rows are not two segments. They are two views of one file that the two writers byte-interleaved
-- (tmp/seg_4f03400f.mp4: 113,580,707 bytes, no moov, unopenable). Each row carries a slice of the
-- truth: one has the file size, the other has the thumbnail, and the detections landed on whichever
-- row the detector claimed.
--
-- So the keeper is the row with the most to keep - most detections, then most tracks, then the
-- earliest started_at, then id - and the loser's children are re-pointed onto it before it is
-- deleted. Children are re-pointed, never deleted: 94,186 detections, 486 tracks and 4,414 audio
-- events hang off the duplicated files, and dropping a track would orphan detection.track_id.
--
-- Re-pointing is safe on the unique constraints that exist: track.key is the tracker's identity
-- (track_key_key) and is untouched, and no unique index spans video_id.
--
-- The keeper's detection_summary / track_summary are kept as written. The pair describes one
-- segment, so the summaries of the two rows describe the same content; the merged row is not
-- claiming more than it holds.
--
-- Soft-deleted rows are in scope, and they are the reason this migration hard-deletes. djangolang
-- puts a `DO INSTEAD UPDATE ... SET deleted_at = now()` rule on every model table (delete_video,
-- delete_detection, ...), so a plain DELETE FROM video is not a delete at all: it is a soft update
-- that leaves the row - and its duplicate (camera_id, file_name) - in place, and reports DELETE 0.
-- The unique index 00012 builds covers every row, deleted_at included, so a soft-deleted loser
-- blocks it. Rules are not applied while a session reports itself as a replica, which is the same
-- lever prune's SQL pulls to hard-delete expired rows. SET LOCAL, so it is gone at COMMIT; while it
-- holds, RI triggers are skipped too, and the re-pointing above is what keeps the children valid.
--
-- Measured on dev (65,314 video rows, 27.9M detections, dry run rolled back): the loser mapping
-- takes 0.7 s, re-pointing 47,093 detections 3 min 28 s (three video_id-leading btree indexes are
-- maintained per row), 243 tracks 1 min 5 s, 2,208 audio events 2 s, deleting 1,765 rows 1.2 s.
-- The applied run (2026-10-19, 63,651 rows left, 0 duplicate groups, no dangling track/video
-- parents) finished in 3 min 28 s, dominated by the detection re-point.
-- It holds row locks on the re-pointed children for that time, which is why it is its own
-- migration: 00012 needs an ACCESS EXCLUSIVE lock on video, and asking for that while holding
-- 47k row locks deadlocks against the live writers (measured: it does exactly that).
--
-- The DO block at the end is the migration's own check: if any group of 2+ rows on the same
-- (camera_id, file_name) survives, the migration fails instead of leaving 00012 to fail.
BEGIN;

SET LOCAL session_replication_role = replica;

CREATE TEMP TABLE _dedupe_video ON COMMIT DROP AS
WITH ranked AS (
  SELECT
    v.id,
    first_value(v.id) OVER w AS keeper_id,
    row_number() OVER w AS rn
  FROM public.video v
  WHERE (v.camera_id, v.file_name) IN (
    SELECT camera_id, file_name
    FROM public.video
    GROUP BY camera_id, file_name
    HAVING count(*) > 1
  )
  WINDOW w AS (
    PARTITION BY v.camera_id, v.file_name
    ORDER BY
      (SELECT count(*) FROM public.detection d WHERE d.video_id = v.id) DESC,
      (SELECT count(*) FROM public.track t WHERE t.video_id = v.id) DESC,
      v.started_at ASC,
      v.id ASC
  )
)
SELECT id, keeper_id FROM ranked WHERE rn > 1;

DO $$
BEGIN
  RAISE NOTICE 'video dedupe: % loser rows', (SELECT count(*) FROM _dedupe_video);
END $$;

UPDATE public.detection d
SET video_id = m.keeper_id
FROM _dedupe_video m
WHERE d.video_id = m.id;

UPDATE public.track t
SET video_id = m.keeper_id
FROM _dedupe_video m
WHERE t.video_id = m.id;

UPDATE public.audio_event a
SET video_id = m.keeper_id
FROM _dedupe_video m
WHERE a.video_id = m.id;

DELETE FROM public.video v
USING _dedupe_video m
WHERE v.id = m.id;

DO $$
DECLARE
  remaining bigint;
BEGIN
  SELECT count(*) INTO remaining
  FROM (
    SELECT camera_id, file_name
    FROM public.video
    GROUP BY camera_id, file_name
    HAVING count(*) > 1
  ) s;

  IF remaining > 0 THEN
    RAISE EXCEPTION 'video dedupe left % duplicate (camera_id, file_name) groups', remaining;
  END IF;

  RAISE NOTICE 'video dedupe: no duplicate (camera_id, file_name) groups remain';
END $$;

COMMIT;
