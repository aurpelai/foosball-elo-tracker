use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Identifiable,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
)]
#[diesel(table_name=crate::repository::schema::teams)]
pub struct Team {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Insertable, Serialize)]
#[diesel(table_name=crate::repository::schema::teams)]
pub struct TeamUpsert {
    pub name: Option<String>,
}
