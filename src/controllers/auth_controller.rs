use crate::controllers::db_utils;
use crate::diesel::ExpressionMethods;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::user::{Claims, InputAuth, InputUser, NewUser, User};
use crate::schema::users::dsl::*;
use crate::Pool;
use actix_web::web::Query;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::update;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SpotifyRedirectParams {
	pub code: Option<String>,
	pub error: Option<String>,
	pub state: String,
}

#[derive(Serialize)]
pub struct SpotifyAuthBody {
	pub grant_type: String,
	pub code: String,
	pub redirect_uri: String,
	pub client_id: String,
	pub client_secret: String,
}

#[derive(Deserialize)]
pub struct SpotifyAuthResponse {
	pub access_token: String,
	pub token_type: String,
	pub scope: String,
	pub expires_in: u32,
	pub refresh_token: String,
}

#[derive(Serialize)]
pub struct ReturnedUser {
	pub user: User,
	pub access_token: String,
}

#[get("/spotify/callback")]
pub async fn spot_auth(
	params: Query<SpotifyRedirectParams>,
	db: web::Data<Pool>,
) -> impl Responder {
	// HttpResponse::Ok().finish()
	let client = Client::new();
	let redirect_uri = std::env::var("REDIRECT_URL").expect("DATABASE_URL must be set");
	let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
	let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
	let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

	let conn = db.get().unwrap();
	match params.code.as_ref() {
		Some(code) => {
			let body = SpotifyAuthBody {
				grant_type: String::from("authorization_code"),
				code: code.to_string(),
				redirect_uri,
				client_id,
				client_secret,
			};

			let res = client
				.post("https://accounts.spotify.com/api/token")
				.form(&body)
				.send()
				.await;
			match res {
				Ok(r) => {
					log::error!("{:?}", r);
					if r.status() == 200 {
						let auth = r.json::<SpotifyAuthResponse>().await;
						auth.map(|spot_auth| {
							let user_id: i32 = params.state.parse().unwrap();
							let user = users.find(user_id).first::<User>(&conn).unwrap();
							let _ = update(&user)
								.set(spotify_refresh.eq(spot_auth.refresh_token))
								.get_result::<User>(&conn);
							HttpResponse::Found().set_header("Location", frontend_url).finish()
						})
						.map_err(|err| HttpResponse::Unauthorized().body(err.to_string()))
					} else {
						Err(HttpResponse::Unauthorized().body(r.text().await.unwrap()))
					}
				}
				Err(e) => Err(HttpResponse::Unauthorized().body(e.to_string())),
			}
		}
		None => Err(HttpResponse::Unauthorized().finish()),
	}
}

#[post("/token")]
pub async fn authenticate(
	db: web::Data<Pool>,
	item: web::Json<InputAuth>,
) -> Result<impl Responder, Error> {
	let conn = db.get().unwrap();

	let user_id = users
		.filter(username.eq(&item.username))
		.get_result::<User>(&conn);
	Ok(user_id
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
					HttpResponse::Ok().json(ReturnedUser {
						user,
						access_token: token.unwrap(),
					})
				}
				false => HttpResponse::BadRequest().json("Password did not match"),
			},
		)
		.map_err(|_| HttpResponse::NotFound()))
}

#[post("/register")]
pub async fn create_user(pool: web::Data<Pool>, item: web::Json<InputUser>) -> impl Responder {
	let new_user = NewUser {
		username: &item.username,
		email: &item.email,
		pass_hash: &hash(&item.password, DEFAULT_COST).expect("could not encode"),
	};

	db_utils::insert_into_table(pool, users, new_user)
		.map(|u: User| HttpResponse::Created().json(u))
		.map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

#[get("/me")]
pub async fn me(user: User) -> impl Responder {
	HttpResponse::Ok().json(user)
}
