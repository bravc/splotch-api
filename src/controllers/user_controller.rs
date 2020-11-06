use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::users::dsl::*;
use crate::Pool;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
// use crate::diesel::Pooled
use crate::models::user::{NewUser, User};

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

#[get("users/{id}")]
pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Get user by ID!")
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Get users!")
}

#[post("/users")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Created().body("Create user!")
}

#[delete("/users/{id}")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Delete user!")
}
