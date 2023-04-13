use anyhow::{Context, Result};
use regex::Regex;
use serde::*;
use std::str::FromStr;

#[derive(Serialize, Debug)]
pub struct AuthDigest {
    realm: String,
    service: String,
    scope: String,
}

impl FromStr for AuthDigest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<AuthDigest> {
        let regex = Regex::new(r#"Bearer +realm="([^,]*)",service="([^,]*)",scope="([^,]*)""#)?;
        regex
            .captures(s)
            .and_then(|capture| {
                Some(AuthDigest {
                    realm: capture.get(1)?.as_str().replace(r#"\""#, ""),
                    service: capture.get(2)?.as_str().to_string(),
                    scope: capture.get(3)?.as_str().to_string(),
                })
            })
            .context("regex matching")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let s = r#"Bearer realm="https://auth.docker.io/token",service="registry.docker.io",scope="repository:library/ubuntu:pull""#;
        let d = s.parse::<AuthDigest>().expect("digest");
        assert_eq!(d.realm, "https://auth.docker.io/token");
        assert_eq!(d.service, "registry.docker.io");
        assert_eq!(d.scope, "repository:library/ubuntu:pull");
    }
}
