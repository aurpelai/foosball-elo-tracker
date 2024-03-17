// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Int4,
        name -> Text,
        active -> Bool,
    }
}
