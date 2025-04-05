CREATE TYPE MediaType AS ENUM ('image', 'video', 'document', 'audio', 'other');

-- Users

CREATE TABLE users (
    id VARCHAR PRIMARY KEY DEFAULT concat('usr_', gen_random_uuid()),
    email VARCHAR NOT NULL UNIQUE,
    username VARCHAR NOT NULL UNIQUE,
    profile_image_url VARCHAR,
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_username ON users (username);

-- Pins

CREATE TABLE verification_pins(
    id VARCHAR PRIMARY KEY DEFAULT concat('vpin_', gen_random_uuid()),
    email VARCHAR NOT NULL,
    pin VARCHAR NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_verification_pins_email ON verification_pins (email);

-- Posts

CREATE TABLE posts (
    id VARCHAR PRIMARY KEY DEFAULT concat('pst_', gen_random_uuid()),
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_posts_user_id ON posts (user_id);

-- Posts Media

CREATE TABLE posts_media (
    id VARCHAR PRIMARY KEY DEFAULT concat('pm_', gen_random_uuid()),
    post_id VARCHAR NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    media_url VARCHAR NOT NULL,
    media_type MediaType NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    width INTEGER,
    height INTEGER,
    file_size INTEGER, 
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Post Likes

CREATE TABLE post_likes (
    id VARCHAR PRIMARY KEY DEFAULT concat('plk_', gen_random_uuid()),
    post_id VARCHAR NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_post_likes_post_id_user_id ON post_likes (post_id, user_id);

-- Post Comments

CREATE TABLE post_comments (
    id VARCHAR PRIMARY KEY DEFAULT concat('pcm_', gen_random_uuid()),
    post_id VARCHAR NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id VARCHAR REFERENCES post_comments(id) ON DELETE CASCADE,
    content VARCHAR NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_post_comments_post_id ON post_comments (post_id);
CREATE INDEX idx_post_comments_user_id ON post_comments (user_id);
CREATE INDEX idx_post_comments_parent_id ON post_comments (parent_id);