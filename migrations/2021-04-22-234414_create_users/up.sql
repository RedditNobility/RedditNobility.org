CREATE TABLE IF NOT EXISTS users
(
    id               BIGINT AUTO_INCREMENT PRIMARY KEY,
    discord_id       BIGINT,
    username         TEXT,
    password         TEXT,
    password_changed BIGINT,
    permissions      TEXT,
    status           TEXT,
    status_changed   BIGINT,
    reviewer         TEXT,
    discoverer       TEXT,
    properties       TEXT,
    title            TEXT,
    birthday         TEXT DEFAULT NULL,
    created          BIGINT
)