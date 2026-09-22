--
-- is_low_res
--
ALTER TABLE public.video
    ADD COLUMN is_low_res boolean NOT NULL DEFAULT false;
