-- Add migration script here
CREATE TABLE IF NOT EXISTS people
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    farts       BLOB                NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS orders
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    price       REAL                NOT NULL DEFAULT 0
);
