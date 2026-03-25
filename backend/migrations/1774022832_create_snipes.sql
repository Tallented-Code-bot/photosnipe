-- Create snipes table
CREATE TABLE IF NOT EXISTS snipes (
    id SERIAL PRIMARY KEY,
    sniper_id BIGINT NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    snipee_id BIGINT NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    picture_url TEXT NOT NULL,      -- Discord attachment URL (images hosted by Discord)
    text TEXT,                      -- Optional text associated with the snipe
    created_at TIMESTAMPTZ DEFAULT now(),
    channel_id BIGINT,              -- Discord channel (optional)
    guild_id BIGINT                 -- Discord server (optional)
);
