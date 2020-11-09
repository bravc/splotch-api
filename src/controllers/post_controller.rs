use crate::diesel::{BelongingToDsl, QueryDsl, RunQueryDsl};
use crate::models::{post::Post, user::User};
use crate::schema::posts::dsl::*;
use crate::Pool;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::result::Error as DieselError;
use std::vec::Vec;
use log::error;

#[get("/posts")]
pub async fn get_posts(user: User, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().unwrap();
    Post::belonging_to(&user)
        .first(&conn)
        .map(|p: Post| HttpResponse::Ok().json(p))
        .map_err(|_| HttpResponse::Ok().json("No posts found"))
}

// #[post("/posts")]
// pub async fn get_user(user: User, pool: web::Data<Pool>) -> impl Responder {
//     let conn = pool.get().unwrap();
//     let results: Result<User, DieselError> = users.find(user_id.into_inner()).first(&conn);
//     match results {
//         Ok(u) => HttpResponse::Ok().json(u),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }
