-- Per-segment overlay offset: the detector correlates the high-resolution and
-- low-resolution recordings of the same segment and stores the content offset it
-- measured, in milliseconds, using the same convention as the frame sweep in
-- tmp/measure_offset.py: high[i] matches low[i + offset]. NULL means the segment
-- was not measurable (static scene, decode failure) and the consumer falls back
-- to the camera's constant.
alter table video add column if not exists overlay_offset_ms integer;

-- Per-camera fallback constant. The paired streams are frame-aligned, so the
-- overlay needs no per-camera correction: the 1-2 s a camera's on-screen clock
-- lags the server clock is shared by both streams and is not an overlay offset.
-- Zero means "align the timelines directly"; the detector may still store a
-- measured per-segment offset within OVERLAY_MEASURE_MAX_ACCEPT_MS.
alter table camera add column if not exists overlay_offset_ms integer not null default 0;

update camera
set overlay_offset_ms = measured.offset_ms
from (
    values
        ('Driveway', 0),
        ('Front Door', 0),
        ('Side Gate', 0)
) as measured (name, offset_ms)
where camera.name = measured.name;
