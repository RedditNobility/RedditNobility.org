CREATE TABLE otps
(
    id   BIGINT AUTO_INCREMENT PRIMARY KEY,
    user BIGINT,
    password  TEXT,
    expiration BIGINT,
    created BIGINT

)