CREATE TABLE IF NOT EXISTS users
(
    id             BIGINT AUTO_INCREMENT PRIMARY KEY,
    discord_id     BIGINT,
    username       TEXT,
    password       TEXT,
    permissions    TEXT,
    status         TEXT,
    status_changed BIGINT,
    reviewer       TEXT,
    discoverer     TEXT,
    properties     TEXT,
    title          TEXT,
    created        BIGINT
)