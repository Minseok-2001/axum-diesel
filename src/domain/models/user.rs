use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::user;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub nickname: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub nickname: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
