WITH
    cte1 AS (
        SELECT
            v.id,
            v.camera_id,
            v.started_at,
            v.ended_at,
            v.detection_summary @> '[{"class_name": "person"}]' AS person_detected
        FROM
            video v
        WHERE
            v.status = 'needs tracking'
    ),
    cte2 AS (
        SELECT
            cte1.id,
            cte1.camera_id,
            cte1.started_at,
            cte1.ended_at,
            cte1.person_detected,
            jsonb_array_elements(v.detection_summary)
        FROM
            cte1
            LEFT JOIN LATERAL (
                SELECT
                    *
                FROM
                    video v
                WHERE
                    v.id = cte1.id
                    AND cte1.person_detected IS true
            ) AS v ON true
    ),
    cte3 AS (
        SELECT
            cte2.id,
            cte2.camera_id,
            cte2.started_at,
            cte2.ended_at,
            (jsonb_array_elements ->> 'class_name')::text AS class_name,
            (jsonb_array_elements ->> 'average_score')::numeric AS average_score,
            (jsonb_array_elements ->> 'weighted_score')::numeric AS weighted_score,
            (jsonb_array_elements ->> 'handled_frame_count')::numeric AS handled_frame_count,
            (jsonb_array_elements ->> 'detected_frame_count')::numeric AS detected_frame_count
        FROM
            cte2
    ),
    cte4 AS (
        SELECT
            *,
            CASE
                WHEN detected_frame_count > handled_frame_count THEN (detected_frame_count / handled_frame_count)::int
                ELSE 1
            END AS detected_person_count
        FROM
            cte3
        WHERE
            class_name = 'person'
    ),
    cte5 AS (
        SELECT
            cte1.*,
            CASE
                WHEN cte4.detected_person_count > 0 THEN cte4.detected_person_count
                ELSE 0
            END AS detected_person_count
        FROM
            cte1
            LEFT JOIN cte4 ON cte1.id = cte4.id
    ),
    cte6 AS (
        SELECT
            *,
            CASE
                WHEN LAG(person_detected) OVER (
                    PARTITION BY
                        camera_id
                    ORDER BY
                        started_at
                ) = false
                AND person_detected IS true THEN true
                ELSE false
            END AS event_start,
            CASE
                WHEN LAG(person_detected) OVER (
                    PARTITION BY
                        camera_id
                    ORDER BY
                        started_at
                ) = true
                AND person_detected IS false THEN true
                ELSE false
            END AS event_end
        FROM
            cte5
    )
SELECT
    cte6.*,
    c.name AS camera_name
FROM
    cte6
    INNER JOIN camera c ON c.id = cte6.camera_id;
