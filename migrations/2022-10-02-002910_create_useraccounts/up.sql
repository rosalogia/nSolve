-- Your SQL goes here
CREATE TABLE user_accounts (
    id SERIAL PRIMARY KEY,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL
);

ALTER TABLE problems ADD COLUMN author_id SERIAL;
ALTER TABLE problems ADD CONSTRAINT fk_author FOREIGN KEY(author_id) REFERENCES user_accounts(id);