use crate::models::player_results::*;
use crate::repository::schema::player_results::dsl::*;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

fn get_new_player_rating() -> PlayerRating {
    // TODO Params
    // TODO elo algorithm

    let initial_player_elo = 1500;
    let player_elo_delta = 6;
    let new_player_elo = initial_player_elo + player_elo_delta;

    PlayerRating {
        rating: new_player_elo,
        rating_delta: player_elo_delta,
    }
}
pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &PlayerResultInsert,
) -> Result<Vec<PlayerResult>, diesel::result::Error> {
    let new_rating = get_new_player_rating();
    let insert_values = PlayerResult {
        player_id: data.player_id,
        match_id: data.match_id,
        result_type: data.result_type,
        rating: new_rating.rating,
        rating_delta: new_rating.rating_delta,
    };

    diesel::insert_into(player_results)
        .values(insert_values)
        .get_results(connection)
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _match_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(player_results.filter(match_id.eq(_match_id))).execute(connection)
}
