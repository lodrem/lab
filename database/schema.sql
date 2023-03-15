--
-- Table: user
--
DROP TABLE IF EXISTS "user";
CREATE TABLE "user"
(
    id            UUID        NOT NULL PRIMARY KEY,
    email         VARCHAR     NOT NULL UNIQUE,
    password_hash VARCHAR     NOT NULL,
    "name"        VARCHAR     NOT NULL,
    avatar_key    VARCHAR,
    created_at    TIMESTAMPTZ NOT NULL,
    updated_at    TIMESTAMPTZ NOT NULL
);

--
-- Table: user_relationship
--
DROP TABLE IF EXISTS user_relationship;
CREATE TABLE user_relationship
(
    follower_id UUID        NOT NULL,
    followee_id UUID        NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (follower_id, followee_id)
);
CREATE INDEX idx_user_relationship_followee_id_follower_id_created_at
    ON user_relationship USING btree (followee_id, follower_id, created_at);
CREATE INDEX idx_user_relationship_follower_id_followee_id_created_at
    ON user_relationship USING btree (follower_id, followee_id, created_at);

--
-- Table: storage_object
--
DROP TABLE IF EXISTS storage_object;
CREATE TABLE storage_object
(
    id         UUID        NOT NULL PRIMARY KEY,
    "key"      VARCHAR     NOT NULL UNIQUE,
    "type"     VARCHAR     NOT NULL,
    "storage"  VARCHAR     NOT NULL,
    "path"     VARCHAR     NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX idx_storage_object_type_key ON storage_object USING btree ("type", "key");
CREATE INDEX idx_storage_object_type_created_at ON storage_object USING btree ("type", created_at);
COMMENT ON COLUMN storage_object.path IS 'The path to locate the object.';
COMMENT ON COLUMN storage_object.storage IS 'The storage provider that stored this object.';

--
-- Table: post
--
DROP TABLE IF EXISTS post;
CREATE TABLE post
(
    id         UUID        NOT NULL PRIMARY KEY,
    "content"  VARCHAR     NOT NULL,
    author_id  UUID        NOT NULL,
    "num_like" INTEGER     NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX idx_post_author_id_created_at ON post USING btree (author_id, created_at);
CREATE INDEX idx_post_created_at ON post USING btree (created_at);
COMMENT ON COLUMN post.content IS 'The content of the post.';
COMMENT ON COLUMN post.num_like IS 'The number of user like this post.';

--
-- Table: user_like_post
--
DROP TABLE IF EXISTS user_like_post;
CREATE TABLE user_like_post
(
    post_id    UUID        NOT NULL,
    user_id    UUID        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (post_id, user_id)
);

--
-- Table: post_bookmark
--
DROP TABLE IF EXISTS post_bookmark;
CREATE TABLE post_bookmark
(
    owner_id   UUID        NOT NULL,
    post_id    UUID        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (owner_id, post_id)
);