use anyhow::{Context, Result};
use reqwest as http;
use std::path::Path;

#[allow(dead_code)]
#[tokio::main]
pub async fn fetch(alias: &str) -> Result<Box<Path>> {
    let mut parts = alias.split(':');
    let name = parts.next().context("image name")?;
    let tag = parts.next().context("image tag")?;
    let url = format!("https://registry.hub.docker.com/v2/library/{name}/manifests/{tag}");

    let fetch = http::get(url).await?;
    dbg!(fetch);

    todo!()
}
