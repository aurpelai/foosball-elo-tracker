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
    team_matches (id) {
        id -> Int4,
        team_id -> Int4,
        match_id -> Int4,
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

diesel::joinable!(team_matches -> matches (match_id));
diesel::joinable!(team_matches -> teams (team_id));

diesel::allow_tables_to_appear_in_same_query!(
    matches,
    players,
    team_matches,
    teams,
);
