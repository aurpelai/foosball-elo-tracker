use crate::models::{
    matches::Match,
    player_matches::{NewPlayerMatch, PlayerMatch},
};
use crate::repository::{queries::players, schema::player_matches::dsl::*};

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn load_all(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Vec<PlayerMatch> {
    player_matches
        .load::<PlayerMatch>(connection)
        .expect("Failed to load team matches.")
}

pub fn find_by_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_match_id: &i32,
) -> Option<PlayerMatch> {
    player_matches
        .find(player_match_id)
        .first::<PlayerMatch>(connection)
        .ok()
}

pub fn find_by_player_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _player_id: &i32,
) -> Vec<PlayerMatch> {
    player_matches
        .filter(player_id.eq(_player_id))
        .load::<PlayerMatch>(connection)
        .expect("Failed to find a team match by team id '{_player_id}.")
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &NewPlayerMatch,
) -> Result<PlayerMatch, diesel::result::Error> {
    diesel::insert_into(player_matches)
        .values(data)
        .get_result(connection)
}

pub fn insert_from_match(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Match,
) -> Result<PlayerMatch, diesel::result::Error> {
    let mut insert_values = vec![];
    let players = players::filter_by_match_id(connection, &data.id);
    let players_iterator = players.iter();

    for player in players_iterator {
        insert_values.push(NewPlayerMatch {
            player_id: player.id,
            match_id: data.id,
        });
    }

    diesel::insert_into(player_matches)
        .values(insert_values)
        .get_result(connection)
}

pub fn delete_by_match_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _match_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(player_matches.filter(match_id.eq(_match_id))).execute(connection)
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_match_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(player_matches.find(player_match_id)).execute(connection)
}
