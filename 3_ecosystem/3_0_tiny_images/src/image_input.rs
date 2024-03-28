use std::{
    error::Error,
    fmt::{self, Display},
    path::PathBuf,
    sync::Arc,
};

use image::ImageError;
use tokio::task::JoinHandle;
use tracing::{event, instrument};

#[derive(Debug, Clone)]
pub enum ImageInput {
    File(String, PathBuf),
    Url(String, reqwest::Url),
}

impl ImageInput {
    pub fn name(&self) -> &str {
        match self {
            ImageInput::File(n, _) | ImageInput::Url(n, _) => n,
        }
    }
}
impl<'de> serde::Deserialize<'de> for ImageInput {
    #[instrument(skip(deserializer))]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let name = s.split('/').last().unwrap_or(&s).to_owned();

        reqwest::Url::parse(&s)
            .map(|u| {
                event!(tracing::Level::INFO, "Parsed URL: {}", u);
                ImageInput::Url(name.clone(), u)
            })
            .or_else(|_| {
                let p = PathBuf::from(&s);
                if p.exists() {
                    event!(tracing::Level::INFO, "Parsed File: {}", name);
                    Ok(ImageInput::File(name, p))
                } else {
                    Err(serde::de::Error::custom(format!(
                        "\"{}\" is not a valid path or URL.",
                        &s
                    )))
                }
            })
    }
}

#[derive(Debug)]
pub enum ImageInputError {
    Io(Arc<dyn Error + Send + Sync>),
    Http(reqwest::Error),
}

impl From<ImageError> for ImageInputError {
    fn from(e: ImageError) -> Self {
        ImageInputError::Io(Arc::new(e))
    }
}

impl From<std::io::Error> for ImageInputError {
    fn from(e: std::io::Error) -> Self {
        ImageInputError::Io(Arc::new(e))
    }
}

impl From<reqwest::Error> for ImageInputError {
    fn from(e: reqwest::Error) -> Self {
        ImageInputError::Http(e)
    }
}

impl Error for ImageInputError {}
impl Display for ImageInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageInputError::Io(e) => write!(f, "IO error: {}", e),
            ImageInputError::Http(e) => write!(f, "HTTP error: {}", e),
        }
    }
}

// Idk if it's a good idea to implement this trait for JoinHandle or instead for Box<Future>
impl Into<JoinHandle<Result<Vec<u8>, ImageInputError>>> for ImageInput {
    fn into(self) -> JoinHandle<Result<Vec<u8>, ImageInputError>> {
        match self {
            ImageInput::File(_, p) => {
                tokio::spawn(async { tokio::fs::read(p).await.map_err(|e| e.into()) })
            }
            ImageInput::Url(_, u) => tokio::spawn(async {
                let r = reqwest::get(u).await?;
                Ok(r.bytes().await?.to_vec())
            }),
        }
    }
}
