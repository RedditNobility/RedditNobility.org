table! {
    users (id) {
        id -> Bigint,
        username -> Text,
        password -> Text,
        level -> Text,
        status -> Text,
        status_changed ->Bigint,
        discoverer -> Text,
        moderator ->Text,
        created ->Bigint,
    }
}
table! {
    auth_tokens (id) {
        id -> Bigint,
        user -> Bigint,
        token -> Text,
    }
}
table! {
    api_keys (id) {
        id -> Bigint,
        api_key -> Text,
    }
}
