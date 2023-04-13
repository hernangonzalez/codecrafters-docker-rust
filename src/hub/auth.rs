mod digest;

use super::DockerAPI;
use anyhow::{ensure, Result};
pub use digest::AuthDigest;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AuthResponse {
    token: String,
}

pub async fn authorise(digest: AuthDigest, client: &Client) -> Result<String> {
    let url = DockerAPI::Authorise.url()?;
    let req = client.get(url).query(&digest);
    let response = req.send().await?;
    ensure!(response.status().is_success());
    let body: AuthResponse = response.json().await?;
    Ok(body.token)
}
