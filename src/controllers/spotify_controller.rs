use crate::errors::SplotchError;
use crate::models::spotify::RecentlyPlayed;
use crate::models::spotify::TopTracks;
use crate::models::{spotify::FreshToken, user::User};

use actix_web::{HttpResponse, web::Query, Responder, get};
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct TopQueryParams {
    time_range: String
}



#[get("/top")]
pub async fn top(user: User, params: Query<TopQueryParams>) -> Result<impl Responder, SplotchError> {
    let token = FreshToken::from_user(user).await;
    let client = Client::new();
    let res = client
        .get("https://api.spotify.com/v1/me/top/tracks?limit=50")
        .bearer_auth(token.access_token)
        .query(&params.into_inner())
        .send()
        .await?;
    let json = res.json::<TopTracks>().await?;
    Ok(HttpResponse::Ok().json(json))
}

#[get("/recent")]
pub async fn recent(user: User) -> Result<impl Responder, SplotchError> {
    let token = FreshToken::from_user(user).await;
    let client = Client::new();
    let res = client
        .get("https://api.spotify.com/v1/me/player/recently-played")
        .bearer_auth(token.access_token)
        .send()
        .await?;
    let json = res.json::<RecentlyPlayed>().await?;
    Ok(HttpResponse::Ok().json(json))
}

#[get("/fresh")]
pub async fn fresh(user: User) -> Result<impl Responder, SplotchError> {
    let token = FreshToken::from_user(user).await;
    Ok(HttpResponse::Ok().json(token))
}
