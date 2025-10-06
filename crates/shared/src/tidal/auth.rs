use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct TidalAuthResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: String,
    pub user_id: u64,
}

pub struct TidalAuth {
    client: Client,
    access_token: String,
    refresh_token: String,
    expires_at: u64,
}

impl TidalAuth {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
            refresh_token,
            expires_at: 0, // Will be set when we validate the token
        }
    }

    /// Validates the current access token by making a request to /oauth2/me
    /// Returns true if the token is valid, false otherwise
    pub async fn validate_token(&self) -> Result<bool> {
        let response = self
            .client
            .get("https://login.tidal.com/oauth2/me")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        let status = response.status();

        Ok(status.is_success())
    }

    /// Refreshes the access token using the refresh token
    /// Returns the new access token if successful
    pub async fn refresh_access_token(&mut self) -> Result<String> {
        let form_data = [
            ("client_id", "49YxDN9a2aFV6RTG"),
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token),
            ("scope", "r_usr+w_usr"),
        ];

        let response = self
            .client
            .post("https://auth.tidal.com/v1/oauth2/token")
            .form(&form_data)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let response_text = response.text().await?;

            let auth_response: TidalAuthResponse =
                serde_json::from_str(&response_text).map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to parse refresh response: {}. Response: {}",
                        e,
                        response_text
                    )
                })?;

            let new_access_token = auth_response.access_token.clone();
            self.access_token = new_access_token.clone();

            // Save the updated access token to config
            let mut config = Config::load()?;
            if let Some(ref mut tidal) = config.tidal {
                tidal.access_token = new_access_token.clone();
                config.save()?;
            }

            // Note: refresh_token doesn't change, so we keep the existing one

            // Set expiration time (current time + expires_in seconds)
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.expires_at = current_time + auth_response.expires_in;

            Ok(new_access_token)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow::anyhow!(
                "Failed to refresh token: {} - {}",
                status,
                error_text
            ))
        }
    }

    /// Ensures we have a valid access token, refreshing if necessary
    pub async fn ensure_valid_token(&mut self) -> Result<()> {
        // First try to validate the current token
        if self.validate_token().await? {
            return Ok(());
        }

        // If validation fails, try to refresh
        if !self.refresh_token.is_empty() {
            self.refresh_access_token().await?;
        } else {
            return Err(anyhow::anyhow!(
                "No refresh token available and access token is invalid"
            ));
        }

        Ok(())
    }

    /// Gets the current access token, ensuring it's valid first
    pub async fn get_valid_access_token(&mut self) -> Result<String> {
        self.ensure_valid_token().await?;
        Ok(self.access_token.clone())
    }

    /// Gets the current refresh token
    pub fn get_refresh_token(&self) -> &str {
        &self.refresh_token
    }

    /// Updates the tokens (useful after a successful refresh)
    pub fn update_tokens(&mut self, access_token: String, refresh_token: String) {
        self.access_token = access_token;
        self.refresh_token = refresh_token;
    }
}
