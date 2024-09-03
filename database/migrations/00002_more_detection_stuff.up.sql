ALTER TABLE video
ADD COLUMN detection_summary jsonb NOT NULL default '[]'::jsonb;
