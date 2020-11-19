pub mod auth_controller;
pub mod db_utils;
pub mod user_controller;
pub mod spotify_controller;

use super::*;
use crate::{controllers, Pool};
use actix_web::dev::ServiceResponse;
use actix_web::test::TestRequest;
use actix_web::{test, web, App};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

#[allow(dead_code)]
async fn test_request(req: TestRequest) -> ServiceResponse {
    let database_url = std::env::var("TEST_DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let mut app = test::init_service(
        App::new()
            .service(
                web::scope("/auth/")
                    .service(controllers::auth_controller::authenticate)
                    .service(controllers::auth_controller::create_user),
            )
            .service(
                web::scope("/api")
                    .service(controllers::user_controller::get_users)
                    .service(controllers::user_controller::get_user)
                    .service(controllers::auth_controller::me)
                    .service(controllers::user_controller::delete_user)
            )
            .data(pool.clone()),
    )
    .await;
    test::call_service(&mut app, req.to_request()).await
}
