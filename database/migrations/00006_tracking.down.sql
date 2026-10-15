--
-- tracking
--
ALTER TABLE public.video DROP COLUMN track_summary;

DROP INDEX public.detection_track_id;

ALTER TABLE public.detection DROP COLUMN track_id;

DROP RULE delete_track ON public.track;

DROP TRIGGER update_track ON public.track;

DROP FUNCTION update_track ();

DROP TRIGGER create_track ON public.track;

DROP FUNCTION create_track ();

DROP TABLE public.track;
