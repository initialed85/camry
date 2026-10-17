--
-- audio detection
--
-- The audio detector claims high-resolution segments (the low-resolution stream carries no
-- audio track), analyses loudness and spectral shape, and writes one row per loud event plus a
-- per-segment summary and loudness curve. Claims use the same lease pattern as
-- object_detector_claimed_until: djangolang generates POST /api/audio-detector-claim-video from
-- the column, gated on the lease being in the past and ordered oldest-first.
--
-- audio_detected_at is the completion marker (the claim filter is audio_detected_at IS NULL
-- alongside an expired lease). It is deliberately not video.status: status belongs to the
-- segment producer and the object detector.
ALTER TABLE public.video
    ADD COLUMN IF NOT EXISTS audio_detector_claimed_until timestamptz NOT NULL DEFAULT to_timestamp(0);

ALTER TABLE public.video
    ADD COLUMN IF NOT EXISTS audio_detected_at timestamptz NULL DEFAULT NULL;

-- Loudness curve for the date slider: fixed 10 Hz grid over the segment, each sample the RMS
-- decibels below full scale as an integer. 600 samples for a 60 s segment.
ALTER TABLE public.video
    ADD COLUMN IF NOT EXISTS audio_profile jsonb NOT NULL DEFAULT '[]'::jsonb;

-- Per-segment event rollup for the table column, same role as detection_summary.
ALTER TABLE public.video
    ADD COLUMN IF NOT EXISTS audio_summary jsonb NOT NULL DEFAULT '[]'::jsonb;

CREATE INDEX video_audio_detector_claimed_until ON public.video (audio_detector_claimed_until);

CREATE INDEX video_audio_detected_at ON public.video (audio_detected_at);

CREATE TABLE
    public.audio_event (
        id uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid (),
        created_at timestamptz NOT NULL DEFAULT now(),
        updated_at timestamptz NOT NULL DEFAULT now(),
        deleted_at timestamptz NULL DEFAULT NULL,
        camera_id uuid NOT NULL REFERENCES public.camera (id),
        video_id uuid NOT NULL REFERENCES public.video (id),
        seen_at timestamptz NOT NULL,
        offset_ms integer NOT NULL,
        ended_at timestamptz NOT NULL,
        duration_ms integer NOT NULL,
        kind text NOT NULL,
        rms_dbfs float NOT NULL,
        peak_dbfs float NOT NULL,
        floor_dbfs float NOT NULL,
        prominence_db float NOT NULL,
        crest_db float NOT NULL,
        score float NOT NULL,
        spectral jsonb NOT NULL DEFAULT '[]'::jsonb
    );

ALTER TABLE public.audio_event OWNER TO postgres;

CREATE INDEX audio_event_seen_at ON public.audio_event (seen_at);

CREATE INDEX audio_event_camera_id_seen_at ON public.audio_event (camera_id, seen_at);

CREATE INDEX audio_event_video_id_seen_at ON public.audio_event (video_id, seen_at);

CREATE INDEX audio_event_kind_seen_at ON public.audio_event (kind, seen_at);

CREATE INDEX audio_event_spectral ON public.audio_event USING GIN (spectral jsonb_path_ops);

--
-- triggers for audio_event
--
CREATE
OR REPLACE FUNCTION create_audio_event () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = now();
  NEW.updated_at = now();
  NEW.deleted_at = null;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_audio_event BEFORE INSERT ON audio_event FOR EACH ROW
EXECUTE PROCEDURE create_audio_event ();

CREATE
OR REPLACE FUNCTION update_audio_event () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = OLD.created_at;
  NEW.updated_at = now();
  IF OLD.deleted_at IS NOT null AND NEW.deleted_at IS NOT null THEN
    NEW.deleted_at = OLD.deleted_at;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_audio_event BEFORE
UPDATE ON audio_event FOR EACH ROW
EXECUTE PROCEDURE update_audio_event ();

CREATE RULE "delete_audio_event" AS ON DELETE TO "audio_event"
DO INSTEAD (
    UPDATE audio_event
    SET
        created_at = old.created_at,
        updated_at = old.updated_at,
        deleted_at = now()
    WHERE
        id = old.id
        AND deleted_at IS null
);
