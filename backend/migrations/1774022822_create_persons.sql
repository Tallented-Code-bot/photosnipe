-- Create persons table
CREATE TABLE IF NOT EXISTS persons (
    id BIGINT PRIMARY KEY,         -- Discord user ID
    username TEXT NOT NULL,        -- Username in Discord
    display_name TEXT              -- Nickname/display (optional)
);
