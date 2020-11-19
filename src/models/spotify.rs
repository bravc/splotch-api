use super::user::User;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RefreshBody<'a> {
    pub refresh_token: &'a str,
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Deserialize, Serialize)]
pub struct FreshToken {
    pub access_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub album: Album,
    pub artists: Vec<Artist>,
    pub duration_ms: u32,
    pub uri: String,
    pub popularity: u32,
}
#[derive(Deserialize, Serialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub uri: String,
}
#[derive(Deserialize, Serialize)]
pub struct Image {
    pub url: String
}

#[derive(Deserialize, Serialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub images: Vec<Image>,
    pub uri: String,
}

#[derive(Deserialize, Serialize)]
pub struct Playlist {
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub user: SpotUser,
    pub tracks: TopTracks,
    pub uri: String,
}

#[derive(Deserialize, Serialize)]
pub struct PlayHistory {
    pub track: Track,
    pub played_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct TopTracks {
    pub items: Vec<Track>
}
#[derive(Deserialize, Serialize)]
pub struct RecentlyPlayed {
    pub items: Vec<PlayHistory>
}

#[derive(Deserialize, Serialize)]
pub struct SpotUser {
    pub images: Vec<Image>,
    pub display_name: String,
    pub email: String
}

impl FreshToken {
    pub async fn from_user(user: &User) -> Self {
        let client = Client::new();
        let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
        let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
        let refresh = user.spotify_refresh.as_ref();

        let body = RefreshBody {
            refresh_token: refresh.unwrap(),
            grant_type: String::from("refresh_token"),
            client_id,
            client_secret,
        };
        let f = client
            .post("https://accounts.spotify.com/api/token")
            .form(&body)
            .send()
            .await
            .unwrap();
        f.json::<FreshToken>().await.unwrap()
    }
}
