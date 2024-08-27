--
-- init
--
CREATE SCHEMA IF NOT EXISTS public;

ALTER SCHEMA public OWNER TO postgres;

COMMENT ON SCHEMA public IS 'standard public schema';

SET
    default_tablespace = '';

SET
    default_table_access_method = heap;

CREATE EXTENSION IF NOT EXISTS postgis SCHEMA public;

CREATE EXTENSION IF NOT EXISTS postgis_raster SCHEMA public;

SET
    postgis.gdal_enabled_drivers = 'ENABLE_ALL';

CREATE EXTENSION IF NOT EXISTS hstore SCHEMA public;

ALTER ROLE postgres
SET
    search_path TO public,
    postgis,
    hstore;

SET
    search_path TO public,
    postgis,
    hstore;

--
-- camera
--
DROP TABLE IF EXISTS public.camera CASCADE;

CREATE TABLE
    public.camera (
        id uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid (),
        created_at timestamptz NOT NULL DEFAULT now(),
        updated_at timestamptz NOT NULL DEFAULT now(),
        deleted_at timestamptz NULL DEFAULT NULL,
        name text NOT NULL CHECK (trim(name) != ''),
        stream_url text NOT NULL CHECK (trim(stream_url) != ''),
        last_seen timestamptz NOT NULL DEFAULT to_timestamp(0),
        -- claimed_at timestamptz NOT NULL DEFAULT to_timestamp(0) CHECK (claimed_at <= now()),
        claimed_at timestamptz NOT NULL DEFAULT to_timestamp(0),
        -- claim_duration interval NOT NULL DEFAULT interval '1 minute' CHECK (claim_duration > interval '0 seconds'),
        claim_duration interval NOT NULL DEFAULT interval '1 minute',
        -- claim_expires_at timestamptz NOT NULL DEFAULT to_timestamp(1) CHECK (claim_expires_at > claimed_at)
        claim_expires_at timestamptz NOT NULL DEFAULT to_timestamp(1)
    );

ALTER TABLE public.camera OWNER TO postgres;

CREATE UNIQUE INDEX camera_unique_name_not_deleted ON public.camera (name)
WHERE
    deleted_at IS null;

CREATE UNIQUE INDEX camera_unique_name_deleted ON public.camera (name, deleted_at)
WHERE
    deleted_at IS NOT null;

--
-- video
--
DROP TABLE IF EXISTS public.video CASCADE;

CREATE TABLE
    public.video (
        id uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid (),
        created_at timestamptz NOT NULL DEFAULT now(),
        updated_at timestamptz NOT NULL DEFAULT now(),
        deleted_at timestamptz NULL DEFAULT NULL,
        file_name text NOT NULL CHECK (trim(file_name) != ''),
        started_at timestamptz NOT NULL,
        ended_at timestamptz NULL,
        duration interval NULL,
        file_size float NULL,
        thumbnail_name text NULL,
        status text NULL,
        camera_id uuid NOT NULL REFERENCES public.camera (id)
    );

ALTER TABLE public.video OWNER TO postgres;

CREATE INDEX video_started_at ON public.video (started_at);

CREATE INDEX video_camera_id_started_at ON public.video (camera_id, started_at);

CREATE INDEX video_ended_at ON public.video (ended_at);

CREATE INDEX video_camera_id_ended_at ON public.video (camera_id, ended_at);

CREATE INDEX video_file_name ON public.video (file_name);

CREATE INDEX video_camera_id_file_name ON public.video (camera_id, file_name);

CREATE INDEX video_status ON public.video (status);

CREATE INDEX video_camera_id_status ON public.video (camera_id, status);

--
-- detection
--
DROP TABLE IF EXISTS public.detection CASCADE;

CREATE TABLE
    public.detection (
        id uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid (),
        created_at timestamptz NOT NULL DEFAULT now(),
        updated_at timestamptz NOT NULL DEFAULT now(),
        deleted_at timestamptz NULL DEFAULT NULL,
        seen_at timestamptz NOT NULL,
        class_id bigint NOT NULL,
        class_name text NOT NULL,
        score float NOT NULL,
        centroid Point NOT NULL,
        bounding_box Polygon NOT NULL,
        video_id uuid NOT NULL REFERENCES public.video (id),
        camera_id uuid NOT NULL REFERENCES public.camera (id)
    );

ALTER TABLE public.detection OWNER TO postgres;

CREATE INDEX detection_seen_at ON public.detection (seen_at);

CREATE INDEX detection_class_id_seen_at ON public.detection (class_id, seen_at);

CREATE INDEX detection_class_name_seen_at ON public.detection (class_name, seen_at);

CREATE INDEX detection_video_id_seen_at ON public.detection (video_id, seen_at);

CREATE INDEX detection_video_id_class_id_seen_at ON public.detection (video_id, class_id, seen_at);

CREATE INDEX detection_video_id_class_name_seen_at ON public.detection (video_id, class_name, seen_at);

--
-- triggers for camera
--
CREATE
OR REPLACE FUNCTION create_camera () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = now();
  NEW.updated_at = now();
  NEW.deleted_at = null;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_camera BEFORE INSERT ON camera FOR EACH ROW
EXECUTE PROCEDURE create_camera ();

CREATE
OR REPLACE FUNCTION update_camera () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = OLD.created_at;
  NEW.updated_at = now();
  IF OLD.deleted_at IS NOT null AND NEW.deleted_at IS NOT null THEN
    NEW.deleted_at = OLD.deleted_at;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_camera BEFORE
UPDATE ON camera FOR EACH ROW
EXECUTE PROCEDURE update_camera ();

CREATE RULE "delete_camera" AS ON DELETE TO "camera"
DO INSTEAD (
    UPDATE camera
    SET
        created_at = old.created_at,
        updated_at = now(),
        deleted_at = now()
    WHERE
        id = old.id
        AND deleted_at IS null
);

CREATE RULE "delete_camera_cascade_to_video" AS ON DELETE TO "camera"
DO ALSO (
    DELETE FROM video
    WHERE
        camera_id = old.id
        AND deleted_at IS null
);

CREATE RULE "delete_camera_cascade_to_detection" AS ON DELETE TO "camera"
DO ALSO (
    DELETE FROM detection
    WHERE
        camera_id = old.id
        AND deleted_at IS null
);

--
-- triggers for video
--
CREATE
OR REPLACE FUNCTION create_video () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = now();
  NEW.updated_at = now();
  NEW.deleted_at = null;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_video BEFORE INSERT ON video FOR EACH ROW
EXECUTE PROCEDURE create_video ();

CREATE
OR REPLACE FUNCTION update_video () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = OLD.created_at;
  NEW.updated_at = now();
  IF OLD.deleted_at IS NOT null AND NEW.deleted_at IS NOT null THEN
    NEW.deleted_at = OLD.deleted_at;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_video BEFORE
UPDATE ON video FOR EACH ROW
EXECUTE PROCEDURE update_video ();

CREATE RULE "delete_video" AS ON DELETE TO "video"
DO INSTEAD (
    UPDATE video
    SET
        created_at = old.created_at,
        updated_at = old.updated_at,
        deleted_at = now()
    WHERE
        id = old.id
        AND deleted_at IS null
);

--
-- triggers for detection
--
CREATE
OR REPLACE FUNCTION create_detection () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = now();
  NEW.updated_at = now();
  NEW.deleted_at = null;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_detection BEFORE INSERT ON detection FOR EACH ROW
EXECUTE PROCEDURE create_detection ();

CREATE
OR REPLACE FUNCTION update_detection () RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at = OLD.created_at;
  NEW.updated_at = now();
  IF OLD.deleted_at IS NOT null AND NEW.deleted_at IS NOT null THEN
    NEW.deleted_at = OLD.deleted_at;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_detection BEFORE
UPDATE ON detection FOR EACH ROW
EXECUTE PROCEDURE update_detection ();

CREATE RULE "delete_detection" AS ON DELETE TO "detection"
DO INSTEAD (
    UPDATE detection
    SET
        created_at = old.created_at,
        updated_at = old.updated_at,
        deleted_at = now()
    WHERE
        id = old.id
        AND deleted_at IS null
);
