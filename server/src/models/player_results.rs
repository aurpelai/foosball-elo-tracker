use crate::repository::types::*;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Associations,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    Queryable,
    Selectable,
    Serialize,
)]
#[diesel(belongs_to(crate::models::players::Player))]
#[diesel(belongs_to(crate::models::matches::Match))]
#[diesel(primary_key(player_id, match_id))]
#[diesel(table_name=crate::repository::schema::player_results)]
pub struct PlayerResult {
    pub player_id: i32,
    pub match_id: i32,
    pub result_type: MatchResultType,
    pub rating: i16,
    pub rating_delta: i16,
}

#[derive(Clone, Debug, Deserialize, Insertable, Serialize)]
#[diesel(table_name=crate::repository::schema::player_results)]
pub struct PlayerResultInsert {
    pub player_id: i32,
    pub match_id: i32,
    pub result_type: MatchResultType,
}

#[derive(Clone, Debug, Deserialize, Queryable, Selectable, Serialize)]
#[diesel(table_name=crate::repository::schema::player_results)]
pub struct PlayerRating {
    pub rating: i16,
    pub rating_delta: i16,
}
