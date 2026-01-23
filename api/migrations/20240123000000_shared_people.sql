-- Migration to share person data across users

-- 1. Create person_bases table
CREATE TABLE person_bases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Insert unique names from current people table
INSERT INTO person_bases (name)
SELECT DISTINCT name FROM people;

-- 3. Rename people to person_trackings
ALTER TABLE people RENAME TO person_trackings;

-- 4. Add person_id column to person_trackings
ALTER TABLE person_trackings ADD COLUMN person_id UUID REFERENCES person_bases(id) ON DELETE CASCADE;

-- 5. Link person_trackings to person_bases
UPDATE person_trackings pt
SET person_id = pb.id
FROM person_bases pb
WHERE pt.name = pb.name;

-- 6. Make person_id NOT NULL and drop name column
ALTER TABLE person_trackings ALTER COLUMN person_id SET NOT NULL;
ALTER TABLE person_trackings DROP COLUMN name;

-- 7. Add index for person_id
CREATE INDEX idx_person_trackings_person_id ON person_trackings(person_id);

-- 8. Existing indexes on person_trackings (formerly people)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_people_user_id') THEN
        ALTER INDEX idx_people_user_id RENAME TO idx_person_trackings_user_id;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_people_category') THEN
        ALTER INDEX idx_people_category RENAME TO idx_person_trackings_category;
    END IF;
END $$;

DROP INDEX IF EXISTS idx_people_name;

-- 9. Add index on person_bases name for search
CREATE INDEX IF NOT EXISTS idx_person_bases_name ON person_bases(name);
