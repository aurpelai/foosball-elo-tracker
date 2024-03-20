use crate::models::{
    r#match::Match,
    schema::team_matches::dsl::*,
    team_match::{NewTeamMatch, TeamMatch},
};
use crate::repository::queries::teams;

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

pub fn filter_by_team_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _team_id: &i32,
) -> Vec<TeamMatch> {
    team_matches
        .filter(team_id.eq(_team_id))
        .load::<TeamMatch>(connection)
        .expect("Failed to find a team match by team id '{_team_id}.")
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
    let mut values = vec![];
    let teams = teams::filter_by_match_id(connection, &data.id);
    let teams_iterator = teams.iter();

    for team in teams_iterator {
        values.push(NewTeamMatch {
            team_id: team.id,
            match_id: data.id,
        });
    }

    diesel::insert_into(team_matches)
        .values(values)
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
