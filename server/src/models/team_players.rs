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
#[diesel(belongs_to(crate::models::teams::Team))]
#[diesel(belongs_to(crate::models::players::Player))]
#[diesel(primary_key(team_id, player_id))]
#[diesel(table_name=crate::repository::schema::team_players)]
pub struct TeamPlayer {
    pub team_id: i32,
    pub player_id: i32,
}
