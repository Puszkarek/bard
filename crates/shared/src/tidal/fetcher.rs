use anyhow::Result;
use reqwest::{Client, header};
use serde_json::Value;

use super::auth::TidalAuth;
use crate::config::Config;
use crate::models::SongInfo;

pub async fn fetch_lyrics(song: &SongInfo) -> Result<Option<String>> {
    // Create a new reqwest client
    let client = Client::new();

    // Get Tidal configuration
    let config = Config::load()?;

    // Check if Tidal is configured
    let tidal_config = match &config.tidal {
        Some(tidal) => tidal,
        None => return Ok(None), // No Tidal config, return None to indicate no lyrics available
    };

    // Create authentication handler
    let mut tidal_auth = TidalAuth::new(
        tidal_config.access_token.clone(),
        tidal_config.refresh_token.clone(),
    );

    // Ensure we have a valid access token
    let access_token = tidal_auth.get_valid_access_token().await?;

    let track_id = get_track_id(&client, &access_token, &song).await?;

    match track_id {
        Some(id) => {
            return get_lyrics(&client, &access_token, &id).await;
        }
        None => {
            return Ok(None);
        }
    }
}

async fn get_lyrics(client: &Client, token: &str, track_id: &str) -> Result<Option<String>> {
    // Create the Tidal API URL
    let url = format!("https://api.tidal.com/v1/tracks/{}/lyrics", track_id);

    // Make the request to Tidal API
    let response = client
        .get(&url)
        .query(&[("countryCode", "BR")])
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response
        let lyrics_data = response.text().await;

        // Extract lyrics from the response
        if let Ok(lyrics_data) = lyrics_data {
            let lyrics_json: Value = serde_json::from_str(&lyrics_data)?;
            if let Some(lyrics) = lyrics_json["subtitles"].as_str() {
                return Ok(Some(lyrics.to_string()));
            }
        }
    }

    Ok(None)
}

async fn get_track_id(client: &Client, token: &str, song: &SongInfo) -> Result<Option<String>> {
    // https://tidal.com/v2/search/?includeContributors=true&includeDidYouMean=true&includeUserPlaylists=true&limit=50&query={}&supportsUserData=true&types=ARTISTS%2CALBUMS%2CTRACKS%2CVIDEOS%2CPLAYLISTS%2CUSERPROFILES&countryCode=BR&locale=en_US&deviceType=BROWSER
    let url = "https://api.tidal.com/v2/search/";

    let response = client
        .get(url)
        .query(&[
            ("limit", "1"),
            ("types", "TRACKS"),
            ("countryCode", "BR"),
            ("locale", "en_US"),
            ("deviceType", "BROWSER"),
            ("query", &format!("{} - {}", song.artist, song.title)),
        ])
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;

    if response.status().is_success() {
        let json_data = response.text().await?;
        // use regex to extract the track ID ({... "id": 123456789})
        let regex = regex::Regex::new(r#""id":\s*(\d+)"#).unwrap();
        let track_id = regex
            .captures(&json_data)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str());
        match track_id {
            Some(id) => return Ok(Some(id.to_string())),
            None => return Ok(None),
        }
    }

    Ok(None)
}
