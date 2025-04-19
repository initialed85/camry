DROP VIEW IF EXISTS video_with_seen_person;

CREATE VIEW
    video_with_seen_person AS (
        WITH
            cte1 AS (
                SELECT
                    date_trunc('second', v.started_at)::timestamptz AS started_at,
                    v.id AS video_id,
                    c.id AS camera_id,
                    jsonb_array_elements(v.detection_summary) AS detection_summary,
                    file_name
                FROM
                    video v
                    INNER JOIN camera c ON c.id = v.camera_id
                WHERE
                    v.detection_summary @> '[{"class_name": "person"}]'
                    AND started_at > now() - interval '1 day'
                    AND ended_at IS NOT null
                    AND status = 'needs tracking'
                    AND detection_summary IS NOT null
                    AND object_detector_claimed_until <= now()
            ),
            cte2 AS (
                SELECT
                    video_id,
                    camera_id,
                    started_at,
                    detection_summary ->> 'class_name' AS class_name,
                    (detection_summary ->> 'average_score')::numeric AS average_score,
                    (detection_summary ->> 'detected_frame_count')::numeric AS detected_frame_count,
                    (detection_summary ->> 'handled_frame_count')::numeric AS handled_frame_count,
                    file_name
                FROM
                    cte1
            ),
            cte3 AS (
                SELECT
                    video_id,
                    camera_id,
                    started_at,
                    class_name,
                    average_score,
                    detected_frame_count,
                    handled_frame_count,
                    file_name,
                    ceil(round(detected_frame_count / handled_frame_count, 2)) AS detected_object_count
                FROM
                    cte2
                WHERE
                    class_name = 'person'
                    AND average_score >= 0.55
                    AND detected_frame_count > 20
            ),
            cte4 AS (
                SELECT
                    video_id,
                    camera_id,
                    started_at,
                    class_name,
                    average_score,
                    detected_frame_count,
                    handled_frame_count,
                    file_name,
                    detected_object_count,
                    LAG(started_at) OVER (
                        PARTITION BY
                            camera_id
                        ORDER BY
                            started_at
                    ) AS prev_started_at,
                    LAG(detected_object_count) OVER (
                        PARTITION BY
                            camera_id
                        ORDER BY
                            started_at
                    ) AS prev_detected_object_count
                FROM
                    cte3
            ),
            cte5 AS (
                SELECT
                    video_id,
                    camera_id,
                    started_at,
                    class_name,
                    average_score,
                    detected_frame_count,
                    handled_frame_count,
                    file_name,
                    detected_object_count,
                    CASE
                        WHEN (
                            prev_started_at IS NULL
                            OR started_at - prev_started_at > INTERVAL '119 seconds'
                        )
                        OR (
                            prev_detected_object_count IS null
                            OR detected_object_count > prev_detected_object_count
                        ) THEN 1
                        ELSE 0
                    END AS is_new_event
                FROM
                    cte4
            ),
            cte6 AS (
                SELECT
                    video_id,
                    camera_id,
                    started_at,
                    class_name,
                    average_score,
                    detected_frame_count,
                    handled_frame_count,
                    file_name,
                    detected_object_count,
                    is_new_event,
                    started_at AT TIME ZONE 'Australia/Perth' AS started_at_local,
                    date_trunc('second', now())::timestamptz - started_at AS when
                FROM
                    cte5
                ORDER BY
                    started_at DESC
            )
        SELECT
            *
        FROM
            cte6
    );
