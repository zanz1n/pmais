-- Add up migration script here

CREATE TABLE "key_value"(
    "key" text PRIMARY KEY,
    "ttl" integer,
    "value" blob NOT NUlL
);
