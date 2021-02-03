use crate::{models::spotify::SpotUser, schema::snippets::dsl::*};
use crate::{controllers::db_utils, Pool};
use actix_web::{web};
use reqwest::Client;

use crate::{
    errors::SplotchError,
    models::{
        snippet::{InputSnippet, NewSnippet, Snippet},
        spotify::{FreshToken, Playlist},
        user::User,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct PlaylistInput {
    pub name: String,
}

#[derive(Serialize)]
struct AddToPlaylistInput<'a> {
    pub uris: &'a Vec<&'a str>,
}

pub async fn create_playlist(user: &User, playlist_name: &str) -> Result<Playlist, SplotchError> {
    let token = FreshToken::from_user(&user).await;
    let client = Client::new();
    let res = client
        .post(&format!(
            "https://api.spotify.com/v1/users/{}/playlists",
            user.spotify_id.as_ref().unwrap()
        ))
        .bearer_auth(&token.access_token)
        .json(&PlaylistInput {
            name: playlist_name.to_owned(),
        })
        .send()
        .await?;
    Ok(res.json::<Playlist>().await?)
}

pub async fn add_to_playlist(
    user: &User,
    playlist_id: String,
    track_uris: &Vec<&str>,
) -> Result<String, SplotchError> {
    let token = FreshToken::from_user(&user).await;
    let client = Client::new();
    client
        .post(&format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id
        ))
        .bearer_auth(token.access_token)
        .json(&AddToPlaylistInput {
            uris: track_uris
        })
        .send()
        .await?;
    Ok(String::from("Playlist added to."))
}

pub async fn create_snippet(
    user: &User,
    pool: web::Data<Pool>,
    new_snippet: &InputSnippet,
) -> Result<Snippet, SplotchError> {
    let input = NewSnippet {
        user_id: user.id,
        name: &new_snippet.name,
        track_uri: &new_snippet.track_uri,
        timestamp_end: new_snippet.timestamp_end,
        timestamp_start: new_snippet.timestamp_start,
    };

    let p: Snippet = db_utils::insert_into_table(pool, snippets, input)?;
    Ok(p)
}

pub async fn get_snippets(
    user: &User,
    pool: web::Data<Pool>,
) -> Result<Vec<Snippet>, SplotchError> {
    let conn = pool.get().unwrap();

    Ok(db_utils::get_all(&conn, snippets)?)
}

pub async fn get_user_info(
    user: &User,
) -> Result<SpotUser, SplotchError> {
    let token = FreshToken::from_user(&user).await;
    let client = Client::new();
    let res = client
        .get("https://api.spotify.com/v1/me")
        .bearer_auth(token.access_token)
        .send()
        .await?;
    
    Ok(res.json::<SpotUser>().await?)
}
