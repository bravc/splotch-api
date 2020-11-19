use crate::models::user::User;
use crate::schema::snippets;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Deserialize, Serialize, Associations, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Snippet {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub timestamp_start: i32,
    pub timestamp_end: i32,
    pub track_uri: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "snippets"]
pub struct NewSnippet<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub track_uri: &'a str,
    pub timestamp_start: i32,
    pub timestamp_end: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputSnippet {
    pub name: String,
    pub track_uri: String,
    pub timestamp_start: i32,
    pub timestamp_end: i32,
}
