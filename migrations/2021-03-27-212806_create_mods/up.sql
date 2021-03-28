CREATE TABLE moderators
(
    id       BIGINT AUTO_INCREMENT PRIMARY KEY,
    username TEXT,
    password TEXT,
    admin    bool DEFAULT false
)