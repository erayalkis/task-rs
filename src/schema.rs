// @generated automatically by Diesel CLI.

diesel::table! {
    lists (id) {
        id -> Nullable<Integer>,
        title -> Text,
    }
}

diesel::table! {
    todos (id) {
        id -> Nullable<Integer>,
        body -> Text,
        completed -> Bool,
        list_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    lists,
    todos,
);
