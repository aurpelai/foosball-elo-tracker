use crate::models::{
    r#match::{Match, NewMatch, Rivalry},
    schema::matches::dsl::*,
};
use crate::repository::queries::team_matches;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn load_all(connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Vec<Match> {
    matches
        .load::<Match>(connection)
        .expect("Failed to load matches.")
}

pub fn find_by_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    match_id: &i32,
) -> Option<Match> {
    matches.find(match_id).first::<Match>(connection).ok()
}

pub fn find_by_team_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    team_id: &i32,
) -> Vec<Match> {
    matches
        .filter(winning_team_id.eq(team_id))
        .or_filter(losing_team_id.eq(team_id))
        .load::<Match>(connection)
        .expect("Failed to find matches by team id '{team_id}.")
}

pub fn find_by_team_ids(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Rivalry,
) -> Vec<Match> {
    matches
        .filter(
            winning_team_id
                .eq(data.team_one_id)
                .and(losing_team_id.eq(data.team_two_id)),
        )
        .or_filter(
            winning_team_id
                .eq(data.team_two_id)
                .and(losing_team_id.eq(data.team_one_id)),
        )
        .load::<Match>(connection)
        .expect("Failed to find matches between team ids '{team_one_id}' and '{team_two_id}'.")
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &NewMatch,
) -> Result<Match, diesel::result::Error> {
    diesel::insert_into(matches)
        .values(data)
        .get_result(connection)
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    match_id: &i32,
) -> Result<usize, diesel::result::Error> {
    team_matches::delete_by_match_id(connection, match_id).ok();
    return diesel::delete(matches.find(match_id)).execute(connection);
}
