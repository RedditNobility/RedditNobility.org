CREATE TABLE users
(
    id             BIGINT AUTO_INCREMENT PRIMARY KEY,
    username       TEXT,
    password       TEXT DEFAULT NULL,
    level          TEXT DEFAULT 'USER',
    status         TEXT DEFAULT 'FOUND',
    status_changed BIGINT,
    moderator      TEXT DEFAULT NULL,
    discoverer     TEXT DEFAULT 'Unknown',
    created        BIGINT
)