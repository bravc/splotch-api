use crate::models::user::User;
use crate::schema::posts;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Deserialize, Serialize, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPost {
    pub title: String,
    pub content: String,
}