-- Your SQL goes here
CREATE TABLE problems (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    content_path TEXT NOT NULL
)