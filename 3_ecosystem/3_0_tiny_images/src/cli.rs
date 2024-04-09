use std::path::Path;

use clap::{error::ErrorKind, Parser};
use config::{ConfigError, Source};
use serde::{de::IntoDeserializer, Deserialize};

use crate::image_input::ImageInput;

fn validator_path_buf(s: &str) -> Result<String, clap::Error> {
    let path = Path::new(s).to_path_buf();

    if path.exists() {
        Ok(s.to_string())
    } else {
        let err = clap::Error::new(ErrorKind::InvalidValue);
        Err(err)
    }
}

fn validator_images(s: &str) -> Result<ImageInput, clap::Error> {
    let de: Result<_, config::ConfigError> = ImageInput::deserialize(s.into_deserializer());

    de.map_err(|_| {
        clap::Error::raw(
            ErrorKind::InvalidValue,
            format!("[\"{}\"] image can either be an `url` or valid `path`", s),
        )
    })
}

#[derive(Debug, Parser, Clone)]
pub struct Cli {
    #[clap(short, long, default_value = "./3_ecosystem/3_0_tiny_images/tiny_images", value_parser = validator_path_buf)]
    output: String,

    #[clap(short, long, default_value_t = 16, value_parser = clap::value_parser!(u8).range(1..))]
    pub parallelism: u8,

    #[clap(short, long, default_value_t = 75, value_parser = clap::value_parser!(u8).range(1..100))]
    quality: u8,

    #[clap(short, long, default_value = None, value_parser = validator_path_buf)]
    pub config: Option<String>,

    #[clap(short, long, value_delimiter = ' ', num_args = 1.., value_parser = validator_images)]
    images: Option<Vec<String>>,

    #[clap(hide = true, default_value = "cli")]
    source: String,
}

impl Source for Cli {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, ConfigError> {
        let mut map = config::Map::new();
        map.insert(
            "output".to_string(),
            config::Value::new(
                Some(&self.source),
                config::ValueKind::String(self.output.clone()),
            ),
        );
        map.insert(
            "quality".to_string(),
            config::Value::new(
                Some(&self.source),
                config::ValueKind::U64(self.quality as u64),
            ),
        );
        map.insert(
            "images".to_string(),
            config::Value::new(
                Some(&self.source),
                config::ValueKind::Array(
                    self.images
                        .clone()
                        .map(|v| {
                            v.iter()
                                .map(|i| {
                                    config::Value::new(
                                        Some(&self.source),
                                        config::ValueKind::String(i.clone()),
                                    )
                                })
                                .collect()
                        })
                        .unwrap_or(vec![]),
                ),
            ),
        );
        Ok(map)
    }
}
