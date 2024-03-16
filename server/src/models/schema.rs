// @generated automatically by Diesel CLI.

diesel::table! {
    player (id) {
        id -> Int4,
        name -> Text,
        active -> Bool,
    }
}
