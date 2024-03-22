use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::repository::schema::matches)]
pub struct Match {
    pub id: i32,
    pub winning_team_id: i32,
    pub losing_team_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub winning_team_score: Option<i16>,
    pub losing_team_score: Option<i16>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name=crate::repository::schema::matches)]
pub struct NewMatch {
    pub winning_team_id: i32,
    pub losing_team_id: i32,
    pub winning_team_score: Option<i16>,
    pub losing_team_score: Option<i16>,
}
