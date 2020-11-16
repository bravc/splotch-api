use crate::controllers::db_utils;
use crate::diesel::{BelongingToDsl, RunQueryDsl};
use crate::models::{
    post::{InputPost, NewPost, Post},
    user::User,
};
use crate::schema::posts::dsl::*;
use crate::Pool;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/posts")]
pub async fn get_posts(user: User, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().unwrap();
    Post::belonging_to(&user)
        .load::<Post>(&conn)
        .map(|p: Vec<Post>| HttpResponse::Ok().json(p))
        .map_err(|_| HttpResponse::Ok().json("No posts found"))
}

#[post("/posts")]
pub async fn create_post(
    user: User,
    pool: web::Data<Pool>,
    item: web::Json<InputPost>,
) -> impl Responder {
    let new_post = NewPost {
        title: &item.title,
        content: &item.content,
        user_id: user.id,
    };
    db_utils::insert_into_table(pool, posts, new_post)
        .map(|p: Post| HttpResponse::Created().json(p))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
