use crate::models::{
    r#match::Match,
    schema::team_matches::dsl::*,
    team_match::{NewTeamMatch, TeamMatch},
};

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn load_all(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Vec<TeamMatch> {
    team_matches
        .load::<TeamMatch>(connection)
        .expect("Failed to load team matches.")
}

pub fn find_by_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    team_match_id: &i32,
) -> Option<TeamMatch> {
    team_matches
        .find(team_match_id)
        .first::<TeamMatch>(connection)
        .ok()
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &NewTeamMatch,
) -> Result<TeamMatch, diesel::result::Error> {
    diesel::insert_into(team_matches)
        .values(data)
        .get_result(connection)
}

pub fn insert_from_match(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Match,
) -> Result<TeamMatch, diesel::result::Error> {
    let team_matches_data = vec![
        NewTeamMatch {
            team_id: data.winning_team_id,
            match_id: data.id,
        },
        NewTeamMatch {
            team_id: data.losing_team_id,
            match_id: data.id,
        },
    ];

    diesel::insert_into(team_matches)
        .values(team_matches_data)
        .get_result(connection)
}

pub fn delete_by_match_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _match_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(team_matches.filter(match_id.eq(_match_id))).execute(connection)
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    team_match_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(team_matches.find(team_match_id)).execute(connection)
}
