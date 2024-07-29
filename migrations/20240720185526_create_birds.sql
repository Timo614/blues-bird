CREATE TABLE IF NOT EXISTS birds (
    id SERIAL PRIMARY KEY NOT NULL,
    image TEXT NOT NULL,
    bird TEXT NOT NULL,
    country TEXT NOT NULL,
    location TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX index_birds_uuid ON birds (id);
CREATE INDEX index_birds_bird ON birds (bird);
CREATE INDEX index_birds_created_at ON birds (created_at);