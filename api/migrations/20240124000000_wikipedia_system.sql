-- Migration to support community-driven Wikipedia system for people

-- 1. Update person_bases with community data
ALTER TABLE person_bases ADD COLUMN global_description TEXT;
ALTER TABLE person_bases ADD COLUMN age INTEGER;
ALTER TABLE person_bases ADD COLUMN creator_id UUID REFERENCES users(id) ON DELETE SET NULL;
ALTER TABLE person_bases ADD COLUMN image_url TEXT; -- Primary display image

-- 2. Create person_photos table for multiple vertical/scrolling images
CREATE TABLE person_photos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES person_bases(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    "order" INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. Create person_edits table for community proposals
CREATE TYPE edit_status AS ENUM ('pending', 'approved', 'rejected');

CREATE TABLE person_edits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES person_bases(id) ON DELETE CASCADE,
    proposer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    new_description TEXT,
    new_age INTEGER,
    new_image_url TEXT,
    status edit_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Create person_votes table for consensus algorithm
CREATE TABLE person_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    edit_id UUID NOT NULL REFERENCES person_edits(id) ON DELETE CASCADE,
    voter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_approve BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(edit_id, voter_id)
);

-- 5. Indexes
CREATE INDEX idx_person_photos_person_id ON person_photos(person_id);
CREATE INDEX idx_person_edits_person_id ON person_edits(person_id);
CREATE INDEX idx_person_votes_edit_id ON person_votes(edit_id);

-- 6. Initial population from trackings (if any exist)
-- For existing people, we'll take the first tracking's description and image_url as the base
UPDATE person_bases pb
SET
    global_description = (SELECT description FROM person_trackings pt WHERE pt.person_id = pb.id ORDER BY created_at ASC LIMIT 1),
    image_url = (SELECT image_url FROM person_trackings pt WHERE pt.person_id = pb.id ORDER BY created_at ASC LIMIT 1),
    creator_id = (SELECT user_id FROM person_trackings pt WHERE pt.person_id = pb.id ORDER BY created_at ASC LIMIT 1);
