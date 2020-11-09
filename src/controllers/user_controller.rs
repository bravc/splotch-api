use crate::diesel::ExpressionMethods;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::Pool;
use actix_web::{delete, get, web, HttpResponse, Responder};

use diesel::dsl::delete;
use diesel::result::Error as DieselError;
use std::vec::Vec;

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

#[get("/users/{id}")]
pub async fn get_user(user_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().unwrap();
    let results: Result<User, DieselError> = users.find(user_id.into_inner()).first(&conn);
    match results {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> impl Responder {
    get_all_users(db)
        .map(|u| HttpResponse::Ok().json(u))
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()))
}

#[delete("/users/{id}")]
pub async fn delete_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> impl Responder {
    let conn = db.get().unwrap();
    users
        .find(user_id.into_inner())
        .first::<User>(&conn)
        .map(|user| {
            delete(&user)
                .get_result::<User>(&conn)
                .map(|_| HttpResponse::NoContent())
                .map_err(|_| HttpResponse::InternalServerError().finish())
        })
        .map_err(|_| HttpResponse::NotFound().finish())
}
