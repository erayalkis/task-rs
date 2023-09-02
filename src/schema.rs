// @generated automatically by Diesel CLI.

diesel::table! {
    lists (id) {
        id -> Integer,
        title -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        body -> Text,
        completed -> Bool,
        list_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    lists,
    tasks,
);
