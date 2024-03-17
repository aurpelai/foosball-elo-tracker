use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::models::schema::teams::dsl::*;
use crate::models::team::{NewTeam, Team};

pub fn get_teams(mut connection: PooledConnection<ConnectionManager<PgConnection>>) -> Vec<Team> {
    teams
        .load::<Team>(&mut connection)
        .expect("Failed to get teams.")
}

pub fn get_team(
    mut connection: PooledConnection<ConnectionManager<PgConnection>>,
    team_id: i32,
) -> Option<Team> {
    teams.find(team_id).first::<Team>(&mut connection).ok()
}

pub fn create_team(
    mut connection: PooledConnection<ConnectionManager<PgConnection>>,
    new_team: NewTeam,
) -> Result<Team, diesel::result::Error> {
    diesel::insert_into(teams)
        .values(&new_team)
        .get_result(&mut connection)
}

pub fn delete_team(
    mut connection: PooledConnection<ConnectionManager<PgConnection>>,
    team_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(teams.find(team_id)).execute(&mut connection)
}
