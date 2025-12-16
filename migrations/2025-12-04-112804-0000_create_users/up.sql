-- Your SQL goes here
CREATE SEQUENCE IF NOT EXISTS users_id_seq;

CREATE TABLE "users"(
	"id" BIGINT NOT NULL PRIMARY KEY,
	"email" VARCHAR NOT NULL,
	"password_hash" VARCHAR NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS users_email_unique_idx ON users(email);
