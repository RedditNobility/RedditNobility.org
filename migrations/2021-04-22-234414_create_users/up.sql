CREATE TABLE users
(
    id             BIGINT AUTO_INCREMENT PRIMARY KEY,
    username       TEXT,
    password       TEXT,
    level          TEXT,
    status         TEXT,
    status_changed BIGINT,
    moderator      TEXT,
    discoverer     TEXT,
    properties     TEXT,
    created        BIGINT
)