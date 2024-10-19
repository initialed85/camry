ALTER TABLE camera
DROP COLUMN segment_producer_claimed_until,
DROP COLUMN stream_producer_claimed_until,
ADD COLUMN claimed_until timestamptz NULL DEFAULT NULL,
ADD COLUMN claimed_by uuid NULL DEFAULT NULL;

ALTER TABLE video
DROP COLUMN object_detector_claimed_until,
DROP COLUMN object_tracker_claimed_until,
ADD COLUMN claimed_until timestamptz NULL DEFAULT NULL,
ADD COLUMN claimed_by uuid NULL DEFAULT NULL;
