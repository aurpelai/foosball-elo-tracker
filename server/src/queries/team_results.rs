use crate::models::{player_results::*, team_results::*};
use crate::queries::{player_results, players};
use crate::repository::schema::team_results::dsl::*;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

fn get_new_team_rating() -> TeamRating {
    // TODO Params
    // TODO elo algorithm

    let initial_team_elo = 1500;
    let team_elo_delta = 6;
    let new_team_elo = initial_team_elo + team_elo_delta;

    TeamRating {
        rating: new_team_elo,
        rating_delta: team_elo_delta,
    }
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &TeamResultInsert,
) -> Result<TeamResult, diesel::result::Error> {
    let new_rating = get_new_team_rating();
    let inserted_team_match;

    match diesel::insert_into(team_results)
        .values(TeamResult {
            team_id: data.team_id,
            match_id: data.match_id,
            result_type: data.result_type,
            rating: new_rating.rating,
            rating_delta: new_rating.rating_delta,
        })
        .get_result::<TeamResult>(connection)
    {
        Ok(result) => inserted_team_match = result,
        Err(err) => return Err(err),
    };

    let players;

    match players::get_players_from_team(connection, &data.team_id) {
        Ok(result) => players = result,
        Err(err) => return Err(err),
    }

    for player in players.iter() {
        match player_results::insert(
            connection,
            &PlayerResultInsert {
                player_id: player.id,
                match_id: data.match_id,
                result_type: data.result_type,
            },
        ) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
    }

    return Ok(inserted_team_match);
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _match_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(team_results.filter(match_id.eq(_match_id))).execute(connection)
}
