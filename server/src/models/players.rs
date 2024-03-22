use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::repository::schema::players)]
pub struct Player {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name=crate::repository::schema::players)]
pub struct NewPlayer {
    pub name: String,
}