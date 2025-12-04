-- Your SQL goes here
CREATE SEQUENCE IF NOT EXISTS articles_id_seq;

CREATE TABLE "articles"(
	"id" BIGINT NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"body" TEXT NOT NULL,
	"published" BOOL NOT NULL
);
