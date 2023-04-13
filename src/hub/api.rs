use anyhow::Result;
use reqwest::Url;

const AUTH_URL: &str = "https://auth.docker.io/";
const REGISTRY_URL: &str = "https://registry.hub.docker.com/";

pub enum DockerAPI<'a> {
    ImageManifest { name: &'a str, tag: &'a str },
    Authorise,
}

impl DockerAPI<'_> {
    pub fn url(&self) -> Result<Url> {
        match self {
            Self::ImageManifest { name, tag } => {
                // ie. https://registry.hub.docker.com/v2/library/alpine/manifests/latest
                let path = format!("v2/library/{name}/manifests/{tag}");
                let url = Url::parse(REGISTRY_URL)?.join(&path)?;
                Ok(url)
            }
            Self::Authorise => {
                let path = "token";
                let url = Url::parse(AUTH_URL)?.join(path)?;
                Ok(url)
            }
        }
    }
}
