use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::models::player::{NewPlayer, Player};
use crate::models::schema::players::dsl::*;

pub fn get_players(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Vec<Player> {
    players
        .load::<Player>(connection)
        .expect("Failed to get players.")
}

pub fn get_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_id: i32,
) -> Option<Player> {
    players.find(player_id).first::<Player>(connection).ok()
}

pub fn create_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_player: NewPlayer,
) -> Result<Player, diesel::result::Error> {
    diesel::insert_into(players)
        .values(&new_player)
        .get_result(connection)
}

pub fn delete_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(players.find(player_id)).execute(connection)
}

pub fn update_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player: Player,
) -> Result<Player, diesel::result::Error> {
    diesel::update(players.find(player.id))
        .set(&player)
        .get_result(connection)
}
