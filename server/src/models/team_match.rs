use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::team_matches)]
pub struct TeamMatch {
    pub id: i32,
    pub team_id: i32,
    pub match_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name=crate::models::schema::team_matches)]
pub struct NewTeamMatch {
    pub team_id: i32,
    pub match_id: i32,
}
