use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum MediaType {
    #[serde(rename = "application/vnd.docker.distribution.manifest.v2+json")]
    ManifestDetail,

    #[serde(rename = "application/vnd.docker.distribution.manifest.list.v2+json")]
    ManifestList,

    #[serde(rename = "application/vnd.docker.image.rootfs.diff.tar.gzip")]
    ImageTarGZip,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Manifest {
    pub digest: String,
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    pub size: u32,
}
