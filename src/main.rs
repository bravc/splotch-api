#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate serde_derive;

use actix_web::{
    dev::ServiceRequest, middleware::Logger, web, App, Error, HttpResponse, HttpServer,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use jsonwebtoken::{decode, DecodingKey, Validation};
use listenfd::ListenFd;

mod controllers;
mod errors;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn auth_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };
    match decode::<models::user::Claims>(
        &credentials.token(),
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    ) {
        Ok(_) => Ok(req),
        Err(err) => Err(HttpResponse::Unauthorized().body(err.to_string()).into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth_middleware);
        App::new()
            .service(
                web::scope("/auth/")
                    .service(controllers::auth_controller::authenticate)
                    .service(controllers::auth_controller::create_user),
            )
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(controllers::user_controller::get_users)
                    .service(controllers::user_controller::get_user)
                    .service(controllers::auth_controller::me)
                    .service(controllers::user_controller::delete_user),
            )
            .wrap(Logger::default())
            .data(pool.clone())
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
