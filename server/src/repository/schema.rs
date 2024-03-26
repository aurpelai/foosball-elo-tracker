// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "result_type"))]
    pub struct ResultType;
}

diesel::table! {
    matches (id) {
        id -> Int4,
        timestamp -> Timestamp,
        update_timestamp -> Nullable<Timestamp>,
        winning_score -> Nullable<Int2>,
        losing_score -> Nullable<Int2>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ResultType;

    player_results (player_id, match_id) {
        player_id -> Int4,
        match_id -> Int4,
        result_type -> ResultType,
        rating -> Int2,
        rating_delta -> Int2,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    team_players (team_id, player_id) {
        team_id -> Int4,
        player_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ResultType;

    team_results (team_id, match_id) {
        team_id -> Int4,
        match_id -> Int4,
        result_type -> ResultType,
        rating -> Int2,
        rating_delta -> Int2,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::joinable!(player_results -> matches (match_id));
diesel::joinable!(player_results -> players (player_id));
diesel::joinable!(team_players -> players (player_id));
diesel::joinable!(team_players -> teams (team_id));
diesel::joinable!(team_results -> matches (match_id));
diesel::joinable!(team_results -> teams (team_id));

diesel::allow_tables_to_appear_in_same_query!(
    matches,
    player_results,
    players,
    team_players,
    team_results,
    teams,
);
