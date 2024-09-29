CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    discord_id BIGINT UNIQUE NOT NULL,
    username VARCHAR(255) NOT NULL,
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL,
    experience INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE warnings (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    moderator_id BIGINT NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_users_discord_id ON users(discord_id);

CREATE INDEX idx_warnings_user_id ON warnings(user_id);