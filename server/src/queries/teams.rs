use crate::models::{team_players::*, team_results::*, teams::*};
use crate::queries::{matches, players};
use crate::repository::schema::{matches::dsl::*, team_players::dsl::*, teams::dsl::*};

use diesel::{
    dsl::*,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

pub fn get_all_teams(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Team>, diesel::result::Error> {
    teams.load::<Team>(connection)
}

pub fn get_team(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _id: &i32,
) -> Result<Team, diesel::result::Error> {
    teams.find(_id).first::<Team>(connection)
}

pub fn get_teams(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _ids: &Vec<i32>,
) -> Result<Vec<Team>, diesel::result::Error> {
    teams
        .filter(crate::repository::schema::teams::id.eq_any(_ids))
        .get_results::<Team>(connection)
}

pub fn get_teams_from_match(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _match_id: &i32,
) -> Result<Vec<Team>, diesel::result::Error> {
    let r#match = matches::get_match(connection, _match_id)?;
    TeamResult::belonging_to(&r#match)
        .inner_join(teams)
        .select(Team::as_select())
        .get_results::<Team>(connection)
}

pub fn get_teams_of_a_player(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _player_id: &i32,
) -> Result<Vec<Team>, diesel::result::Error> {
    let player = players::get_player(connection, _player_id)?;
    TeamPlayer::belonging_to(&player)
        .inner_join(teams)
        .select(Team::as_select())
        .get_results::<Team>(connection)
}

fn get_common_team_from_players(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    player_ids: &Vec<i32>,
) -> Result<Option<Team>, diesel::result::Error> {
    assert_eq!(
        player_ids.len(),
        2,
        "Should search for teams with exactly two players."
    );

    let players = players::get_players(connection, player_ids)?;
    TeamPlayer::belonging_to(&players)
        .inner_join(teams)
        .group_by(crate::repository::schema::teams::id)
        .having(count(crate::repository::schema::teams::id).gt(1))
        .select(Team::as_select())
        .first::<Team>(connection)
        .optional()
}

pub fn find_latest_team_rating(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _team_id: &i32,
) -> Result<TeamRating, diesel::result::Error> {
    let team = get_team(connection, _team_id)?;

    TeamResult::belonging_to(&team)
        .inner_join(matches)
        .order(timestamp.desc())
        .limit(1)
        .select(TeamRating::as_select())
        .get_result::<TeamRating>(connection)
}

pub fn insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _player_ids: &Vec<i32>,
    _team_data: &TeamUpsert,
) -> Result<Team, diesel::result::Error> {
    assert_eq!(
        _player_ids.len(),
        2,
        "A team must consist of exactly two players!"
    );

    match diesel::insert_into(teams)
        .values(_team_data)
        .get_result::<Team>(connection)
    {
        Ok(result) => {
            let mut team_players_data = vec![];

            for _id in _player_ids {
                team_players_data.push(TeamPlayer {
                    team_id: result.id,
                    player_id: *_id,
                });
            }

            match diesel::insert_into(team_players)
                .values(team_players_data)
                .execute(connection)
            {
                Ok(_) => Ok(result),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub fn get_or_insert(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _player_ids: &Vec<i32>,
) -> Result<Team, diesel::result::Error> {
    assert_eq!(
        _player_ids.len(),
        2,
        "A team must consist of exactly two players!"
    );

    match get_common_team_from_players(connection, &_player_ids)? {
        Some(result) => Ok(result),
        None => insert(connection, &_player_ids, &TeamUpsert { name: None }),
    }
}

pub fn delete(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    _team_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(teams.find(_team_id)).execute(connection)
}
