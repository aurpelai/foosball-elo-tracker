use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
)]
#[diesel(table_name=crate::repository::schema::players)]
pub struct Player {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Insertable, Serialize)]
#[diesel(table_name=crate::repository::schema::players)]
pub struct PlayerUpsert {
    pub name: String,
}
