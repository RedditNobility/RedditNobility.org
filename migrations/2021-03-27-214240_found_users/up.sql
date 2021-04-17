CREATE TABLE users
(
    id        BIGINT AUTO_INCREMENT PRIMARY KEY,
    username  TEXT,
    status    TEXT,
    moderator TEXT DEFAULT NULL,
    created   BIGINT
)