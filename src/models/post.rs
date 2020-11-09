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
