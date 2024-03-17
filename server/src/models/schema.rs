// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Int4,
        name -> Text,
        active -> Bool,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        player_one_id -> Int4,
        player_two_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    players,
    teams,
);
