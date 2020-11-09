use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize,};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub pass_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub pass_hash: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub sub: i32,
   pub company: String
}
