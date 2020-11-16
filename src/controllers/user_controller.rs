use crate::controllers::db_utils;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::Pool;
use actix_web::{delete, get, web, HttpResponse, Responder};
use diesel::dsl::delete;
use std::vec::Vec;

#[get("/users/{id}")]
pub async fn get_user(user_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    db_utils::get_by_id(pool, users, user_id.into_inner())
        .map(|u: User| HttpResponse::Created().json(u))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> impl Responder {
    let conn = db.get().unwrap();

    db_utils::get_all(&conn, users)
        .map(|u: Vec<User>| HttpResponse::Ok().json(u))
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

#[cfg(test)]
mod tests {
    use crate::controllers::test_request;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_get_users() {
        let req = test::TestRequest::with_uri("/api/users");
        let resp = test_request(req).await;
        assert_eq!(resp.status(), 200);
    }
}
