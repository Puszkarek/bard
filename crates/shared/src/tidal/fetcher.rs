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
    // New Tidal OpenAPI URL for track details using the tracks endpoint
    let url = format!("https://openapi.tidal.com/v2/tracks/{}", track_id);

    // Make the request to Tidal API including lyrics in the response
    let response = client
        .get(&url)
        .query(&[
            ("countryCode", "BR"),
            ("include", "lyrics"),
        ])
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response following JSON:API specification
        let json: Value = response.json().await?;
        
        // The lyrics are in the 'included' array when "include=lyrics" is requested
        if let Some(included) = json["included"].as_array() {
            for item in included {
                if item["type"] == "lyrics" {
                    // Try to get lrcText (synced lyrics) first, then fallback to text (plain lyrics)
                    if let Some(lrc) = item["attributes"]["lrcText"].as_str() {
                        if !lrc.is_empty() {
                            return Ok(Some(lrc.to_string()));
                        }
                    }
                    if let Some(text) = item["attributes"]["text"].as_str() {
                        return Ok(Some(text.to_string()));
                    }
                }
            }
        }
    }

    Ok(None)
}

async fn get_track_id(client: &Client, token: &str, song: &SongInfo) -> Result<Option<String>> {
    // Search endpoint URL remains api.tidal.com/v2/search/ for now as it is more stable for general search
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
        // Updated regex to handle both "id": 123456 and "id": "123456" formats
        // This is necessary because some newer Tidal API responses return IDs as strings
        let regex = regex::Regex::new(r#""id":\s*"?(\d+)"?"#).unwrap();
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

