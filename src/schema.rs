// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int8,
        name -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(articles, users,);
