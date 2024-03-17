use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::teams)]
pub struct Team {
    pub id: i32,
    pub player_one_id: i32,
    pub player_two_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name=crate::models::schema::teams)]
pub struct NewTeam {
    pub player_one_id: i32,
    pub player_two_id: i32,
}
