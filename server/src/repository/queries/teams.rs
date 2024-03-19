use crate::models::{
    schema::teams::dsl::*,
    team::{NewTeam, Team},
};

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn load_all(connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Vec<Team> {
    teams
        .load::<Team>(connection)
        .expect("Failed to load teams.")
}

pub fn find_by_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    team_id: &i32,
) -> Option<Team> {
    teams.find(team_id).first::<Team>(connection).ok()
}

pub fn find_by_player_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_id: &i32,
) -> Vec<Team> {
    teams
        .filter(player_one_id.eq(player_id))
        .or_filter(player_two_id.eq(player_id))
        .load::<Team>(connection)
        .expect("Failed to load teams by player id '{player_id}'.")
}

pub fn find_by_player_ids(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &NewTeam,
) -> Option<Team> {
    teams
        .filter(
            player_one_id
                .eq(data.player_one_id)
                .and(player_two_id.eq(data.player_two_id)),
        )
        .or_filter(
            player_one_id
                .eq(data.player_two_id)
                .and(player_two_id.eq(data.player_one_id)),
        )
        .first::<Team>(connection)
        .ok()
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &NewTeam,
) -> Result<Team, diesel::result::Error> {
    diesel::insert_into(teams)
        .values(data)
        .get_result(connection)
}

pub fn find_or_insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &NewTeam,
) -> Result<Team, diesel::result::Error> {
    match find_by_player_ids(connection, &data) {
        Some(team) => Ok(team),
        None => insert(connection, &data),
    }
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    team_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(teams.find(team_id)).execute(connection)
}
