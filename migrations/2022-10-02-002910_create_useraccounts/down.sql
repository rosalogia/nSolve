-- This file should undo anything in `up.sql`
DROP TABLE user_accounts;
ALTER TABLE problems DROP COLUMN author_id;
ALTER TABLE problems DROP CONSTRAINT fk_author;