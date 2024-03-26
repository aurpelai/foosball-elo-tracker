use crate::repository::schema::sql_types::ResultType;

use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(AsExpression, Clone, Copy, Debug, Deserialize, Eq, FromSqlRow, PartialEq, Serialize)]
#[diesel(sql_type = ResultType)]
pub enum MatchResultType {
    ShutoutWin,
    Win,
    Loss,
    ShutoutLoss,
}

impl ToSql<ResultType, Pg> for MatchResultType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            MatchResultType::ShutoutWin => out.write_all(b"SHUTOUT_WIN")?,
            MatchResultType::Win => out.write_all(b"WIN")?,
            MatchResultType::Loss => out.write_all(b"LOSS")?,
            MatchResultType::ShutoutLoss => out.write_all(b"SHUTOUT_LOSS")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ResultType, Pg> for MatchResultType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"SHUTOUT_WIN" => Ok(MatchResultType::ShutoutWin),
            b"WIN" => Ok(MatchResultType::Win),
            b"LOSS" => Ok(MatchResultType::Loss),
            b"SHUTOUT_LOSS" => Ok(MatchResultType::ShutoutLoss),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
