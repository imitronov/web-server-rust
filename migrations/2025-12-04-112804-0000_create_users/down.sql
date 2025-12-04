-- This file should undo anything in `up.sql`
DROP SEQUENCE IF EXISTS users_id_seq;

DROP INDEX IF EXISTS users_email_unique_idx;

DROP TABLE IF EXISTS "users";
