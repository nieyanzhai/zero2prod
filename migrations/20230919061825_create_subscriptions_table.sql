-- Add migration script here
CREATE TABLE subscriptions
(
    id            uuid        NOT NULL PRIMARY KEY,
    name          text        NOT NULL,
    email         text        NOT NULL unique,
    subscribed_at timestamptz NOT NULL
)