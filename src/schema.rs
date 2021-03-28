table! {
    moderators (id) {
        id -> Bigint,
        username -> Text,
        password -> Text,
        admin -> Bool,
    }
}
table! {
    members (id) {
        id -> Bigint,
        username -> Text,
        moderator -> Text,
        created_on ->Bigint,
    }
}
table! {
    fusers (id) {
        id -> Bigint,
        username -> Text,
    }
}
