// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        creation_timestamp -> Timestamp,
        title -> Text,
        body -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
