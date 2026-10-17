--
-- audio detection
--
ALTER TABLE public.video DROP COLUMN audio_summary;

ALTER TABLE public.video DROP COLUMN audio_profile;

DROP INDEX public.video_audio_detected_at;

DROP INDEX public.video_audio_detector_claimed_until;

DROP RULE delete_audio_event ON public.audio_event;

DROP TRIGGER update_audio_event ON public.audio_event;

DROP FUNCTION update_audio_event ();

DROP TRIGGER create_audio_event ON public.audio_event;

DROP FUNCTION create_audio_event ();

DROP TABLE public.audio_event;

ALTER TABLE public.video DROP COLUMN audio_detected_at;

ALTER TABLE public.video DROP COLUMN audio_detector_claimed_until;
