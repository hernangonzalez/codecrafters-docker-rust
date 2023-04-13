use anyhow::Result;
use reqwest::{
    header::{self, HeaderMap},
    Client, Method, RequestBuilder, Url,
};

const AUTH_URL: &str = "https://auth.docker.io/";
const REGISTRY_URL: &str = "https://registry.hub.docker.com/";

pub enum DockerAPI<'a> {
    ImageManifest { name: &'a str, tag: &'a str },
    Authorise,
}

impl DockerAPI<'_> {
    fn url(&self) -> Result<Url> {
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

    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> Result<HeaderMap> {
        let mut map = HeaderMap::default();
        if let Self::ImageManifest { .. } = self {
            map.append(
                header::ACCEPT,
                "application/vnd.docker.distribution.manifest.v2+json".parse()?,
            );
            map.insert(
                header::ACCEPT,
                "application/vnd.docker.distribution.manifest.list.v2+json".parse()?,
            );
        };
        Ok(map)
    }

    pub fn build(&self, client: &Client) -> Result<RequestBuilder> {
        let url: Url = self.url()?;
        let req = client.request(self.method(), url).headers(self.headers()?);
        Ok(req)
    }
}