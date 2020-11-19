use crate::schema::snippets::dsl::*;
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

pub async fn create_playlist(user: &User, playlist_name: &str) -> Result<Playlist, SplotchError> {
    let token = FreshToken::from_user(&user).await;
    let client = Client::new();
    let res = client
        .post(&format!(
            "https://api.spotify.com/v1/users/{}/playlists",
            user.id
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
) -> Result<Playlist, SplotchError> {
    let token = FreshToken::from_user(&user).await;
    let client = Client::new();
    let res = client
        .post(&format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id
        ))
        .bearer_auth(token.access_token)
        .json(track_uris)
        .send()
        .await?;
    Ok(res.json::<Playlist>().await?)
}

pub async fn create_snippet(
    user: &User,
    pool: web::Data<Pool>,
    new_snippet: &InputSnippet,
) -> Result<Snippet, SplotchError> {
    let input = NewSnippet {
        user_id: user.id,
        title: &new_snippet.title,
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
