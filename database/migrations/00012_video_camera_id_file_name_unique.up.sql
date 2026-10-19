--
-- one row per (camera, file)
--
-- The segment producer fences a camera with a session-scoped advisory lock (pkg/segment_producer/
-- claim.go), and the fence is what stops two producers writing the same file. This index is the
-- database's own backstop: with it, a second row for the same camera and file name is not a row the
-- UI renders twice, it is an insert that fails.
--
-- The claim loop's question used to be "is this lease expired?", which is not "am I writing this
-- camera?". That gap is what produced the 1765 duplicate groups this migration removes, and every
-- duplicate insert since would have been silent.
--
-- Plain CREATE UNIQUE INDEX, not CONCURRENTLY: golang-migrate runs a whole migration file in one
-- implicit transaction, and CONCURRENTLY cannot run inside a transaction block. The build takes an
-- ACCESS EXCLUSIVE lock on video for its duration, which stalls every writer to video - segment
-- producers, detectors, the API - so it runs outside peak load. Measured on the deduped 63.6k-row
-- table: 0.95s (00013 then dropped the redundant index in 0.08s).
--
-- 00011 is the precondition: it is the check that no (camera_id, file_name) group of 2+ rows
-- survives, and a build over dirty data fails with a duplicate-key error and leaves the table alone.
-- Measured on the undeduped table: it aborted in 0.12s naming
-- (4f03400f-…, Segment_2026-10-19T02:08:00_…_low_res.mp4), which is the point of the index.
BEGIN;

CREATE UNIQUE INDEX IF NOT EXISTS video_camera_id_file_name_key ON public.video (camera_id, file_name);

COMMIT;
