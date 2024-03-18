// @generated automatically by Diesel CLI.

diesel::table! {
    matches (id) {
        id -> Int4,
        winning_team_id -> Int4,
        losing_team_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        winning_team_score -> Nullable<Int2>,
        losing_team_score -> Nullable<Int2>,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        player_one_id -> Int4,
        player_two_id -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    matches,
    players,
    teams,
);
