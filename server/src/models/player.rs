use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::player)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub active: Bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name=crate::models::schema::player)]
pub struct PlayerEvent {
    pub name: String,
    pub active: Bool,
}
