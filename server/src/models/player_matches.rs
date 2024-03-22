use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::repository::schema::player_matches)]
pub struct PlayerMatch {
    pub id: i32,
    pub player_id: i32,
    pub match_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name=crate::repository::schema::player_matches)]
pub struct NewPlayerMatch {
    pub player_id: i32,
    pub match_id: i32,
}
