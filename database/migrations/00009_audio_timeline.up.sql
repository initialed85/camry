--
-- audio timeline projection
--
-- The date slider needs one row per camera-hour, not one row per event: a raw audio_event page
-- for a 7-day window is ~20 MB, 81% of which is the video_id_object the strip never reads, and
-- the loudest-first limit truncates the window (the 03:00-17:00 sampling bug).
--
-- djangolang introspects views (pkg/introspect/introspect.go: relkind IN ('r', 'v')) but the
-- generator skips them and skips anything without a primary key
-- (pkg/template/template.go: "// TODO: add support for views"), so this projection cannot
-- become a generated model. It is served by a custom endpoint over this view:
-- GET /api/custom/audio-timeline (internal/audio_timeline.go).
CREATE OR REPLACE VIEW public.v_audio_event_hour AS
SELECT
    date_trunc('hour', seen_at) AS bucket,
    camera_id,
    count(*) AS event_count,
    max(score) AS top_score,
    max(prominence_db) AS peak_prominence_db,
    (array_agg(kind ORDER BY score DESC, seen_at))[1] AS top_kind,
    min(seen_at) AS first_seen_at,
    max(seen_at) AS last_seen_at
FROM
    public.audio_event
WHERE
    deleted_at IS NULL
GROUP BY
    date_trunc('hour', seen_at),
    camera_id;
