-- Add unique constraint to ensure a user can only track a person once
-- This prevents a user from adding the same person as both 'kisah' and 'bini' for example

ALTER TABLE person_trackings ADD CONSTRAINT person_trackings_user_id_person_id_key UNIQUE (user_id, person_id);
