--
-- drop the index 00012 replaced
--
-- video_camera_id_file_name is a plain btree on exactly (camera_id, file_name), the same columns in
-- the same order as the unique index 00012 builds. It is strictly weaker: same scan, same
-- selectivity, no constraint. Every query it was added for (the segment producer's and detector's
-- lookups by camera and file) is served by the unique index, so keeping both means paying a second
-- insert and a second bulk-delete for every segment, on the hottest write path in the system.
--
-- video_file_name stays: it serves the file-name-only lookups (the detector's high-res/low-res
-- sibling match), where (camera_id, file_name) cannot be used as a leading-column scan.
BEGIN;

DROP INDEX IF EXISTS public.video_camera_id_file_name;

COMMIT;
