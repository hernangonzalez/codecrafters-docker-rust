mod api;
mod auth;
mod manifest;

use super::fs;
use anyhow::{ensure, Context, Result};
use api::DockerAPI;
use auth::AuthDigest;
use bytes::Bytes;
use http::{header, Client, Response, StatusCode};
use manifest::{Manifest, MediaType};
use reqwest as http;
use serde::Deserialize;
use std::path::Path;

const IMAGE_ALIAS_SEPARATOR: char = ':';
const IMAGE_TAG_DEFAULT: &str = "latest";

#[tokio::main]
pub async fn pull(alias: &str, dst: &Path) -> Result<()> {
    let mut parts = alias.split(IMAGE_ALIAS_SEPARATOR);
    let repo = parts.next().context("image name")?;
    let tag = parts.next().unwrap_or(IMAGE_TAG_DEFAULT);

    let manifest = fetch_manifest(repo, tag).await?;
    let bytes = fetch_image(repo, manifest).await?;
    fs::unpack(bytes, dst)
}

async fn send_request<'a>(client: &Client, api: DockerAPI<'a>) -> Result<Response> {
    let req = api.build(client)?;
    let mut resp = req.send().await?;
    if resp.status() == StatusCode::UNAUTHORIZED {
        let wwwauth = resp
            .headers()
            .get(header::WWW_AUTHENTICATE)
            .context("auth digest")?
            .to_str()?;
        let digest = wwwauth.parse::<AuthDigest>()?;
        let token = auth::authorise(digest, client).await?;
        let request = api.build(client)?.bearer_auth(token);
        resp = request.send().await?;
    }
    Ok(resp)
}

#[derive(Deserialize, Debug)]
struct ListResponse {
    manifests: Vec<Manifest>,
}

#[derive(Deserialize, Debug)]
struct DetailResponse {
    layers: Vec<Manifest>,
}

async fn fetch_manifest(repo: &str, tag: &str) -> Result<Manifest> {
    let api = DockerAPI::ManifestList { repo, tag };
    let client = Client::new();
    let resp = send_request(&client, api).await?;
    ensure!(resp.status().is_success(), "Could not get manifest list");

    let resp: ListResponse = resp.json().await?;
    let manifest = resp.manifests.first().context("manifest")?;
    let digest = &manifest.digest;

    let api = DockerAPI::ManifestDetail { repo, digest };
    let resp = send_request(&client, api).await?;
    ensure!(resp.status().is_success(), "Could not get manifest detail");

    let resp: DetailResponse = resp.json().await?;
    let manifest = resp
        .layers
        .iter()
        .find(|m| m.media_type == MediaType::ImageTarGZip)
        .context("image manifest")?;

    Ok(manifest.clone())
}

async fn fetch_image<'a>(repo: &str, manifest: Manifest) -> Result<Bytes> {
    let client = Client::new();
    let api = DockerAPI::DownloadBlob {
        repo,
        digest: &manifest.digest,
    };

    let resp = send_request(&client, api).await?;
    ensure!(
        resp.status().is_success(),
        "Failed to download the docker image"
    );

    let bytes = resp.bytes().await?;
    Ok(bytes)
}
