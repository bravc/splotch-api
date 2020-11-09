use crate::diesel::ExpressionMethods;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::user::{Claims, InputAuth, InputUser, NewUser, User};
use crate::schema::users::dsl::*;
use crate::Pool;
use actix_web::{post, web, Error, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::dsl::insert_into;

use jsonwebtoken::{encode, EncodingKey, Header};

#[post("/token")]
pub async fn authenticate(
  db: web::Data<Pool>,
  item: web::Json<InputAuth>,
) -> Result<impl Responder, Error> {
  let conn = db.get().unwrap();

  let user_id = users
    .filter(username.eq(&item.username))
    .get_result::<User>(&conn);
  Ok(
    user_id
      .map(
        |user: User| match verify(&item.password, &user.pass_hash).expect("f") {
          true => {
            let my_claims = Claims {
              sub: user.id,
              company: "hey".to_string(),
            };
            let token = encode(
              &Header::default(),
              &my_claims,
              &EncodingKey::from_secret("secret".as_ref()),
            );
            HttpResponse::Ok().json(token.unwrap())
          }
          false => HttpResponse::BadRequest().json("Password did not match"),
        },
      )
      .map_err(|_| HttpResponse::NotFound()),
  )
}

#[post("/register")]
pub async fn create_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> impl Responder {
  let conn = db.get().unwrap();
  let new_user = NewUser {
    username: &item.username,
    email: &item.email,
    pass_hash: &hash(&item.password, DEFAULT_COST).expect("could not encode"),
  };
  let res = insert_into(users)
    .values(&new_user)
    .get_result::<User>(&conn);
  match res {
    Ok(u) => HttpResponse::Created().json(u),
    Err(er) => HttpResponse::NotFound().json(er.to_string()),
  }
}
