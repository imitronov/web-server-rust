// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int8,
        name -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
