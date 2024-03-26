use crate::models::{matches::*, team_results::*};
use crate::queries::{player_results, team_results, teams};
use crate::repository::{schema::matches::dsl::*, types::*};

use diesel::{
    dsl::*,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn get_all_matches(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Match>, diesel::result::Error> {
    matches.load::<Match>(connection)
}

pub fn get_match(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _id: &i32,
) -> Result<Match, diesel::result::Error> {
    matches.find(_id).first::<Match>(connection)
}

pub fn get_matches_by_team(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _team_id: &i32,
) -> Result<Vec<(Match, TeamResult)>, diesel::result::Error> {
    let team = teams::get_team(connection, _team_id)?;
    TeamResult::belonging_to(&team)
        .inner_join(matches)
        .select((Match::as_select(), TeamResult::as_select()))
        .get_results(connection)
}

pub fn get_matches_between_teams(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _team_ids: &Vec<i32>,
) -> Result<Vec<Match>, diesel::result::Error> {
    assert_eq!(
        _team_ids.len(),
        2,
        "A rivalry has to consist of exactly two teams."
    );

    let teams = teams::get_teams(connection, _team_ids)?;
    TeamResult::belonging_to(&teams)
        .inner_join(matches)
        .group_by(crate::repository::schema::matches::id)
        .having(count(crate::repository::schema::matches::id).gt(1))
        .select(Match::as_select())
        .get_results::<Match>(connection)
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    data: &MatchData,
) -> Result<Match, diesel::result::Error> {
    let winning_team = teams::get_or_insert(connection, &data.winning_player_ids)?;
    let losing_team = teams::get_or_insert(connection, &data.losing_player_ids)?;

    let match_insert_data = MatchInsert {
        winning_score: data.winning_score,
        losing_score: data.losing_score,
    };

    let inserted_match;

    match diesel::insert_into(matches)
        .values(match_insert_data)
        .get_result::<Match>(connection)
    {
        Ok(result) => inserted_match = result,
        Err(err) => return Err(err),
    };

    let mut win_type = MatchResultType::Win;
    let mut loss_type = MatchResultType::Loss;

    if let Some(0) = data.losing_score {
        win_type = MatchResultType::ShutoutWin;
        loss_type = MatchResultType::ShutoutLoss;
    }

    team_results::insert(
        connection,
        &TeamResultInsert {
            team_id: winning_team.id,
            match_id: inserted_match.id,
            result_type: win_type,
        },
    )?; // TODO match this instead of question marking

    team_results::insert(
        connection,
        &TeamResultInsert {
            team_id: losing_team.id,
            match_id: inserted_match.id,
            result_type: loss_type,
        },
    )?; // TODO match this instead of question marking

    return Ok(inserted_match);
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    match_id: &i32,
) -> Result<usize, diesel::result::Error> {
    team_results::delete(connection, match_id).ok();
    player_results::delete(connection, match_id).ok();
    diesel::delete(matches.find(match_id)).execute(connection)
}
