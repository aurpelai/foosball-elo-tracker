use crate::models::{
    player::{NewPlayer, Player},
    schema::players::dsl::*,
};

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn load_players(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Vec<Player> {
    players
        .load::<Player>(connection)
        .expect("Failed to get players.")
}

pub fn find_player_by_id(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_id: &i32,
) -> Option<Player> {
    players.find(player_id).first::<Player>(connection).ok()
}

pub fn insert_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &NewPlayer,
) -> Result<Player, diesel::result::Error> {
    diesel::insert_into(players)
        .values(data)
        .get_result(connection)
}

pub fn update_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Player,
) -> Result<Player, diesel::result::Error> {
    diesel::update(players.find(data.id))
        .set(data)
        .get_result(connection)
}

pub fn delete_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(players.find(player_id)).execute(connection)
}
