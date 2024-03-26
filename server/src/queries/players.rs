use crate::models::{player_results::*, players::*, team_players::*};
use crate::queries::{matches, teams};
use crate::repository::schema::players::dsl::*;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn get_all_players(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Player>, diesel::result::Error> {
    players.load::<Player>(connection)
}

pub fn get_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _id: &i32,
) -> Result<Player, diesel::result::Error> {
    players.find(_id).first::<Player>(connection)
}

pub fn get_players(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _ids: &Vec<i32>,
) -> Result<Vec<Player>, diesel::result::Error> {
    players
        .filter(id.eq_any(_ids))
        .get_results::<Player>(connection)
}

pub fn get_players_from_match(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _match_id: &i32,
) -> Result<Vec<Player>, diesel::result::Error> {
    let r#match = matches::get_match(connection, _match_id)?;
    PlayerResult::belonging_to(&r#match)
        .inner_join(players)
        .select(Player::as_select())
        .get_results(connection)
}

pub fn get_players_from_team(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _team_id: &i32,
) -> Result<Vec<Player>, diesel::result::Error> {
    let team = teams::get_team(connection, _team_id)?;
    TeamPlayer::belonging_to(&team)
        .inner_join(players)
        .select(Player::as_select())
        .get_results(connection)
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &PlayerUpsert,
) -> Result<Player, diesel::result::Error> {
    diesel::insert_into(players)
        .values(data)
        .get_result::<Player>(connection)
}

pub fn update(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &Player,
) -> Result<Player, diesel::result::Error> {
    diesel::update(players.find(data.id))
        .set(data)
        .get_result::<Player>(connection)
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _player_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(players.find(_player_id)).execute(connection)
}
