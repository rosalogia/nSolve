// @generated automatically by Diesel CLI.

diesel::table! {
    problems (id) {
        id -> Int4,
        title -> Text,
        content_path -> Text,
        author_id -> Int4,
    }
}

diesel::table! {
    user_accounts (id) {
        id -> Int4,
        display_name -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(problems -> user_accounts (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    problems,
    user_accounts,
);
