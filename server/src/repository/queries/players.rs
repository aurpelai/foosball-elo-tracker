use crate::models::{
    player::{NewPlayer, Player},
    schema::players::dsl::*,
};

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
