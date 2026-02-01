-- 1. Users
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    display_name TEXT NOT NULL,
    avatar_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 2. Clubs
CREATE TABLE IF NOT EXISTS clubs (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT,
    owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    image_url TEXT,
    image_path TEXT, -- Local path if stored on disk
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 3. Club Memberships
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('OWNER', 'MOD', 'MEMBER');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE member_status AS ENUM ('ACTIVE', 'BANNED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS club_memberships (
    club_id INTEGER REFERENCES clubs(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    role user_role DEFAULT 'MEMBER',
    status member_status DEFAULT 'ACTIVE',
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (club_id, user_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS one_owner_per_club
ON club_memberships (club_id)
WHERE role = 'OWNER';

-- 4. Club Messages
CREATE TABLE IF NOT EXISTS club_messages (
    id SERIAL PRIMARY KEY,
    club_id INTEGER REFERENCES clubs(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_club_messages_club_time ON club_messages(club_id, created_at DESC);

-- 5. Events
CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    club_id INTEGER REFERENCES clubs(id) ON DELETE CASCADE, -- Nullable als openbaar event
    title TEXT NOT NULL,
    description TEXT,
    location TEXT,
    starts_at TIMESTAMP WITH TIME ZONE NOT NULL,
    ends_at TIMESTAMP WITH TIME ZONE,
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 6. Event Attendees (RSVP)
DO $$ BEGIN
    CREATE TYPE rsvp_status AS ENUM ('GOING', 'INTERESTED', 'NOT_GOING');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS event_attendees (
    event_id INTEGER REFERENCES events(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    status rsvp_status DEFAULT 'GOING',
    PRIMARY KEY (event_id, user_id)
);
