-- Your SQL goes here

CREATE TABLE tasks
(
    id      SERIAL PRIMARY KEY,
    title   VARCHAR NOT NULL,
    body    TEXT    NOT NULL,
    checked BOOLEAN NOT NULL DEFAULT 'f'
)