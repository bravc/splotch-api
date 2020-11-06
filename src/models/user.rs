use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize,};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pass_hash: &'a str,
}