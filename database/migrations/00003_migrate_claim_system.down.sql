ALTER TABLE camera
ADD COLUMN segment_producer_claimed_until timestamptz NOT NULL DEFAULT to_timestamp(0),
ADD COLUMN stream_producer_claimed_until timestamptz NOT NULL DEFAULT to_timestamp(0),
DROP COLUMN claimed_until,
DROP COLUMN claimed_by;

ALTER TABLE video
ADD COLUMN object_detector_claimed_until timestamptz NOT NULL DEFAULT to_timestamp(0),
ADD COLUMN object_tracker_claimed_until timestamptz NOT NULL DEFAULT to_timestamp(0),
DROP COLUMN claimed_until,
DROP COLUMN claimed_by;
