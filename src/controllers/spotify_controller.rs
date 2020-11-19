use crate::{Pool, models::{snippet::InputSnippet, spotify::RecentlyPlayed}};
use crate::models::spotify::TopTracks;
use crate::models::{
    spotify::{FreshToken, SpotUser, Track},
    user::User,
};
use crate::{errors::SplotchError, services::spotify_service};

use actix_web::post;
use actix_web::{
    get,
    web::{self, Query},
    HttpResponse, Responder,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct TopQueryParams {
    time_range: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePlaylistInput {
    name: String,
    tracks: Vec<Track>,
}

impl CreatePlaylistInput {
    pub fn to_uris(&self) -> Vec<&str> {
        let mut uris: Vec<&str> = Vec::new();
        for track in &self.tracks {
            uris.push(track.uri.as_str())
        }
        uris
    }
}

#[get("/top")]
pub async fn top(
    user: User,
    params: Query<TopQueryParams>,
) -> Result<impl Responder, SplotchError> {
    let token = FreshToken::from_user(&user).await;
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
    let token = FreshToken::from_user(&user).await;
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
    let token = FreshToken::from_user(&user).await;
    Ok(HttpResponse::Ok().json(token))
}

#[get("/spotify/me")]
pub async fn me(user: User) -> Result<impl Responder, SplotchError> {
    let token = FreshToken::from_user(&user).await;
    let client = Client::new();
    let res = client
        .get("https://api.spotify.com/v1/me")
        .bearer_auth(token.access_token)
        .send()
        .await?;
    let json = res.json::<SpotUser>().await?;
    Ok(HttpResponse::Ok().json(json))
}

#[post("/playlist")]
pub async fn playlist(
    user: User,
    item: web::Json<CreatePlaylistInput>,
) -> Result<impl Responder, SplotchError> {
    let input = item.into_inner();
    let empty_playlist = spotify_service::create_playlist(&user, input.name.as_ref()).await?;

    let playlist =
        spotify_service::add_to_playlist(&user, empty_playlist.id, &input.to_uris())
            .await?;
    Ok(HttpResponse::Created().json(playlist))
}

#[post("/snippet")]
pub async fn snippet_create(
    user: User,
    db: web::Data<Pool>,
    item: web::Json<InputSnippet>,
) -> Result<impl Responder, SplotchError> {
    let input = item.into_inner();
    let snippet = spotify_service::create_snippet(&user, db, &input).await?;

    Ok(HttpResponse::Created().json(snippet))
}

#[get("/snippet/all")]
pub async fn snippet_get_all(
    user: User,
    db: web::Data<Pool>,
) -> Result<impl Responder, SplotchError> {
    let snippet = spotify_service::get_snippets(&user, db).await?;

    Ok(HttpResponse::Created().json(snippet))
}
