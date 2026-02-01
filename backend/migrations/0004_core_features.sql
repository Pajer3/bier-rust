-- 1. Beers
CREATE TABLE beers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    brewery TEXT,
    type TEXT,
    abv FLOAT,
    ibu INTEGER,
    color TEXT,
    image_url TEXT,
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_beers_name ON beers(name);
CREATE INDEX idx_beers_type ON beers(type);

-- 2. Reviews
CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    beer_id UUID NOT NULL REFERENCES beers(id) ON DELETE CASCADE,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    text TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_user_beer_review UNIQUE (user_id, beer_id)
);

CREATE INDEX idx_reviews_beer_id ON reviews(beer_id);

-- 3. Reports (Moderation)
CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reporter_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    target_type TEXT NOT NULL, -- 'beer', 'review', 'club', 'user'
    target_id TEXT NOT NULL,   -- UUID or ID as string
    reason TEXT,
    status TEXT DEFAULT 'open', -- 'open', 'resolved', 'dismissed'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
