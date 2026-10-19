-- The rows this migration deleted are the evidence of a bug that no longer exists (see the up
-- migration); a rollback cannot rebuild them. A child row re-pointed onto a keeper is not restored
-- to its deleted parent, so a down migration is a no-op by design.
SELECT 1;
