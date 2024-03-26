use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
)]
#[diesel(table_name=crate::repository::schema::matches)]
pub struct Match {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub update_timestamp: Option<NaiveDateTime>,
    pub winning_score: Option<i16>,
    pub losing_score: Option<i16>,
}

#[derive(Clone, Debug, Deserialize, Insertable, Serialize)]
#[diesel(table_name=crate::repository::schema::matches)]
pub struct MatchInsert {
    pub winning_score: Option<i16>,
    pub losing_score: Option<i16>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchData {
    pub winning_player_ids: Vec<i32>,
    pub losing_player_ids: Vec<i32>,
    pub winning_score: Option<i16>,
    pub losing_score: Option<i16>,
}
