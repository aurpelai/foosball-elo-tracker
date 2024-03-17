use crate::models::schema::teams::dsl::*;
use crate::models::team::{NewTeam, Team};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub fn get_teams(connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Vec<Team> {
    teams
        .load::<Team>(connection)
        .expect("Failed to get teams.")
}

pub fn get_team(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    team_id: i32,
) -> Option<Team> {
    teams.find(team_id).first::<Team>(connection).ok()
}

pub fn get_team_by_players(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: NewTeam,
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

pub fn create_team(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: NewTeam,
) -> Result<Team, diesel::result::Error> {
    diesel::insert_into(teams)
        .values(&data)
        .get_result(connection)
}

pub fn create_or_get_team(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: NewTeam,
) -> Result<Team, diesel::result::Error> {
    let existing_team = get_team_by_players(connection, data.clone());

    match existing_team {
        Some(team) => Ok(team),
        None => create_team(connection, data.clone()),
    }
}

pub fn delete_team(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    team_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(teams.find(team_id)).execute(connection)
}
