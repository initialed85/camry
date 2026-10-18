--
-- detection + object counts per hour per camera, for the timeline strip
--
-- Not a generated model: djangolang's introspection filters on relkind IN ('r', 'v') inside the
-- configured schema, so this lives in a schema of its own and a plain table there would be picked
-- up and generated. Materialized views (relkind 'm') are invisible to codegen for the same reason.
--
-- A materialized view is also the only shape that survives the CDC stream. Each API pod runs
-- `CREATE PUBLICATION djangolang_<node> FOR ALL TABLES` (djangolang raw_stream), and its consumer
-- errors out on any table it does not know: writing to a plain table in this schema killed the
-- local API with "table detection_hour unknown in change" and the server closed. Publications
-- cover tables only, so REFRESH MATERIALIZED VIEW produces no logical messages at all.
--
-- Counting detections is not enough: 100 detections of one car is one object. The object count
-- tracks, which is what the "Objects" overlay draws, so an hour with detections and no new objects
-- reads as zero objects.
--
-- Aggregating on request is not viable: the detection table holds 27.9M live rows (25.5M in the
-- strip's 7-day window) and the hour aggregate over them costs 213 s serial / 106 s with 6 parallel
-- workers, with a 1.5 GB external merge sort. This view holds one row per (hour, camera) - about
-- 400 rows a day - and the strip reads it off its unique index.
--
-- The 7-day window matches prune's retention, and the refresh job rebuilds the whole view
-- (`REFRESH MATERIALIZED VIEW CONCURRENTLY`, measured ~106 s), so late-landing detections - the
-- object detector runs hours behind the segment producer - are picked up at the next refresh.
--
-- The strip asks for whole hours, so buckets are calendar hours and the refresh is hourly: a
-- bucket is at most one hour stale, against a detector that is already hours behind.
BEGIN;

CREATE SCHEMA IF NOT EXISTS timeline;

CREATE MATERIALIZED VIEW timeline.detection_hour AS
WITH detections AS (
  SELECT
    date_trunc('hour', seen_at) AS bucket,
    camera_id,
    count(*) AS detection_count,
    max(score) AS top_score,
    (array_agg(class_name ORDER BY score DESC, seen_at))[1] AS top_class_name,
    min(seen_at) AS first_seen_at,
    max(seen_at) AS last_seen_at
  FROM public.detection
  WHERE deleted_at IS NULL
    AND seen_at >= date_trunc('hour', now()) - interval '7 days'
  GROUP BY date_trunc('hour', seen_at), camera_id
), tracks AS (
  -- Objects are counted in the hour their track started, so the row reads as "new objects in this
  -- hour" rather than "objects visible in this hour".
  SELECT
    date_trunc('hour', started_at) AS bucket,
    camera_id,
    count(*) AS object_count,
    max(max_score) AS top_score,
    (array_agg(class_name ORDER BY max_score DESC, started_at))[1] AS top_class_name,
    min(started_at) AS first_seen_at,
    max(ended_at) AS last_seen_at
  FROM public.track
  WHERE deleted_at IS NULL
    AND started_at >= date_trunc('hour', now()) - interval '7 days'
  GROUP BY date_trunc('hour', started_at), camera_id
)
SELECT
  coalesce(detections.bucket, tracks.bucket) AS bucket,
  coalesce(detections.camera_id, tracks.camera_id) AS camera_id,
  coalesce(detections.detection_count, 0) AS detection_count,
  coalesce(tracks.object_count, 0) AS object_count,
  coalesce(detections.top_score, tracks.top_score) AS top_score,
  coalesce(detections.top_class_name, tracks.top_class_name) AS top_class_name,
  coalesce(detections.first_seen_at, tracks.first_seen_at) AS first_seen_at,
  coalesce(detections.last_seen_at, tracks.last_seen_at) AS last_seen_at
FROM detections
FULL OUTER JOIN tracks ON detections.bucket = tracks.bucket AND detections.camera_id = tracks.camera_id
WITH NO DATA;

-- required by REFRESH ... CONCURRENTLY, and it is the access path the endpoint uses
CREATE UNIQUE INDEX detection_hour_bucket_camera_id ON timeline.detection_hour (bucket, camera_id);

COMMIT;
