table! {
    moderators (id) {
        id -> Bigint,
        username -> Text,
        password -> Text,
        admin -> Bool,
    }
}

table! {
    users (id) {
        id -> Bigint,
        username -> Text,
        status -> Text,
        moderator ->Text,
        created ->Bigint,
    }
}
