-- Table: user
CREATE TABLE "user"
(
    id         SERIAL PRIMARY KEY,
    nickname   VARCHAR(50),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

-- Table: board
CREATE TABLE board
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(50),
    description VARCHAR(50),
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP,
    deleted_at  TIMESTAMP
);

-- Table: post
CREATE TABLE post
(
    id         SERIAL PRIMARY KEY,
    board_id   INTEGER,
    user_id    INTEGER,
    hash_tags  TEXT[],
    like_count INTEGER,
    view_count INTEGER,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    CONSTRAINT fk_post_board FOREIGN KEY (board_id) REFERENCES board (id),
    CONSTRAINT fk_post_user FOREIGN KEY (user_id) REFERENCES "user" (id)
);

-- Table: comment
CREATE TABLE comment
(
    id         SERIAL PRIMARY KEY,
    user_id    INTEGER,
    post_id    INTEGER,
    content    TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    CONSTRAINT fk_comment_user FOREIGN KEY (user_id) REFERENCES "user" (id),
    CONSTRAINT fk_comment_post FOREIGN KEY (post_id) REFERENCES post (id)
);
