use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::schema::users::dsl::users as user_dsl;
use crate::schema::*;
use crate::Pool;
use actix_web::{dev, web, Error, FromRequest, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::NaiveDateTime;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub pass_hash: String,
    pub spotify_refresh: Option<String>,
    pub created_at: NaiveDateTime,
}

impl FromRequest for User {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        debug!("chillin");
        let pool = req.app_data::<web::Data<Pool>>().unwrap();
        let conn = pool.get().unwrap();

        if let Ok(auth) = BearerAuth::from_request(req, payload).into_inner() {
            let validation = Validation {
                validate_exp: false,
                ..Default::default()
            };
            let cred = decode::<Claims>(
                &auth.token(),
                &DecodingKey::from_secret("secret".as_ref()),
                &validation,
            );
            match cred {
                Ok(token) => match user_dsl.find(token.claims.sub).first(&conn) {
                    Ok(user) => ok(user),
                    Err(e) => err(HttpResponse::InternalServerError()
                        .body(e.to_string())
                        .into()),
                },
                Err(e) => err(HttpResponse::Unauthorized().body(e.to_string()).into()),
            }
        } else {
            err(HttpResponse::Unauthorized().into())
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub pass_hash: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub company: String,
}
