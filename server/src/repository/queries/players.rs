use crate::models::players::{NewPlayer, Player};
use crate::repository::{queries::teams, schema::players::dsl::*};

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn load_all(connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Vec<Player> {
    players
        .load::<Player>(connection)
        .expect("Failed to get players.")
}

pub fn find_by_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_id: &i32,
) -> Option<Player> {
    players.find(player_id).first::<Player>(connection).ok()
}

pub fn load_by_team(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    team_id: &i32,
) -> Vec<Player> {
    match teams::find_by_id(connection, team_id) {
        Some(data) => players
            .filter(id.eq_any([data.player_one_id, data.player_two_id]))
            .get_results(connection)
            .expect("Failed to get players of team '{data.id}'"),
        None => vec![],
    }
}

pub fn filter_by_match_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    match_id: &i32,
) -> Vec<Player> {
    let mut query_result = vec![];
    let teams = teams::filter_by_match_id(connection, &match_id);
    let teams_iterator = teams.iter();

    for team in teams_iterator {
        let team_players = load_by_team(connection, &team.id);
        query_result.extend_from_slice(&team_players)
    }

    query_result
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &NewPlayer,
) -> Result<Player, diesel::result::Error> {
    diesel::insert_into(players)
        .values(data)
        .get_result(connection)
}

pub fn update(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Player,
) -> Result<Player, diesel::result::Error> {
    diesel::update(players.find(data.id))
        .set(data)
        .get_result(connection)
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(players.find(player_id)).execute(connection)
}
