--
-- tracking
--
CREATE TABLE
    public.track (
        id uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid (),
        key uuid NOT NULL UNIQUE DEFAULT gen_random_uuid (),
        created_at timestamptz NOT NULL DEFAULT now(),
        updated_at timestamptz NOT NULL DEFAULT now(),
        deleted_at timestamptz NULL DEFAULT NULL,
        camera_id uuid NOT NULL REFERENCES public.camera (id),
        video_id uuid NOT NULL REFERENCES public.video (id),
        class_id bigint NOT NULL,
        class_name text NOT NULL,
        started_at timestamptz NOT NULL,
        ended_at timestamptz NOT NULL,
        frame_count integer NOT NULL DEFAULT 0,
        mean_score float NOT NULL DEFAULT 0,
        max_score float NOT NULL DEFAULT 0,
        centroid Point NOT NULL,
        bounding_box Polygon NOT NULL,
        trajectory jsonb NOT NULL DEFAULT '[]'::jsonb,
        is_static boolean NOT NULL DEFAULT false,
        status text NOT NULL DEFAULT 'tracking'
    );

ALTER TABLE public.track OWNER TO postgres;

CREATE INDEX track_camera_id_started_at ON public.track (camera_id, started_at);

CREATE INDEX track_class_name_started_at ON public.track (class_name, started_at);

CREATE INDEX track_video_id_started_at ON public.track (video_id, started_at);

CREATE INDEX track_trajectory ON public.track USING GIN (trajectory jsonb_path_ops);

ALTER TABLE public.detection
    ADD COLUMN track_id uuid NULL REFERENCES public.track (id);

CREATE INDEX detection_track_id ON public.detection (track_id);

ALTER TABLE public.video
    ADD COLUMN track_summary jsonb NOT NULL DEFAULT '[]'::jsonb;

--
-- triggers for track
--
CREATE
OR REPLACE FUNCTION create_track () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = now();
  NEW.updated_at = now();
  NEW.deleted_at = null;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_track BEFORE INSERT ON track FOR EACH ROW
EXECUTE PROCEDURE create_track ();

CREATE
OR REPLACE FUNCTION update_track () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = OLD.created_at;
  NEW.updated_at = now();
  IF OLD.deleted_at IS NOT null AND NEW.deleted_at IS NOT null THEN
    NEW.deleted_at = OLD.deleted_at;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_track BEFORE
UPDATE ON track FOR EACH ROW
EXECUTE PROCEDURE update_track ();

CREATE RULE "delete_track" AS ON DELETE TO "track"
DO INSTEAD (
    UPDATE track
    SET
        created_at = old.created_at,
        updated_at = old.updated_at,
        deleted_at = now()
    WHERE
        id = old.id
        AND deleted_at IS null
);
