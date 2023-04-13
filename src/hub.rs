mod api;
mod auth;

use anyhow::{ensure, Context, Result};
use api::DockerAPI;
use auth::AuthDigest;
use http::{header, Client, StatusCode, Url};
use reqwest as http;
use std::path::Path;

const IMAGE_ALIAS_SEPARATOR: char = ':';
const IMAGE_TAG_DEFAULT: &str = "latest";

struct Manifest;

#[tokio::main]
pub async fn fetch<'a>(alias: &str) -> Result<&'a Path> {
    let mut parts = alias.split(IMAGE_ALIAS_SEPARATOR);
    let name = parts.next().context("image name")?;
    let tag = parts.next().unwrap_or(IMAGE_TAG_DEFAULT);

    let _manifest = fetch_manifest(name, tag).await?;

    todo!()
}

async fn fetch_manifest(name: &str, tag: &str) -> Result<Manifest> {
    let api = DockerAPI::ImageManifest { name, tag };
    let client = Client::new();

    let build_req = |client: &Client| {
        let url: Url = api.url()?;
        let req = client
            .get(url)
            .header(
                header::ACCEPT,
                "application/vnd.docker.distribution.manifest.v2+json",
            )
            .header(
                header::ACCEPT,
                "application/vnd.docker.distribution.manifest.list.v2+json",
            );
        anyhow::Ok(req)
    };

    let request = build_req(&client)?;

    let mut response = request.send().await?;
    if response.status() == StatusCode::UNAUTHORIZED {
        let wwwauth = response
            .headers()
            .get(header::WWW_AUTHENTICATE)
            .context("auth digest")?
            .to_str()?;
        let digest = wwwauth.parse::<AuthDigest>()?;
        let token = auth::authorise(digest, &client).await?;

        let request = build_req(&client)?.bearer_auth(token);
        response = request.send().await?;
    }

    dbg!(&response);
    ensure!(response.status().is_success());

    todo!()
}
