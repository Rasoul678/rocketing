// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Int4,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    persons,
    posts,
    todos,
);
