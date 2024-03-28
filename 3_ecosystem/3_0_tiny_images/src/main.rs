use clap::Parser;
use config::{Config, ConfigError, Environment};

use instant_layer::InstantLayer;
use serde::{de::IntoDeserializer, Deserialize};
use std::{
    io::{self, Write},
    path::PathBuf,
};
use tokio::task::JoinHandle;
use tracing::{event, instrument, span};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod cli;
mod image_input;
mod instant_layer;

use cli::Cli;

use image_input::{ImageInput, ImageInputError};

#[derive(Debug, Deserialize)]
struct Settings {
    output: PathBuf,
    quality: u8,
    images: Vec<ImageInput>,
}

impl Settings {
    #[instrument(name = "Settings::new")]
    fn new(cli: &Cli) -> anyhow::Result<Self> {
        let mut settings = Config::builder()
            .add_source(cli.clone())
            .add_source(Environment::with_prefix("tiny_"));

        if let Some(ref config) = cli.config {
            event!(tracing::Level::INFO, "Loading config from file: {}", config);
            settings = settings.add_source(config::File::from(PathBuf::from(config)));
        }

        Ok(settings.build()?.try_deserialize()?)
    }
}

fn main() -> anyhow::Result<()> {
    // export RUST_LOG="warn,step_3=info,step_3::instant_layer=debug,step_3::image_input=info"
    tracing_subscriber::registry()
        .with(fmt::layer().pretty())
        .with(EnvFilter::from_default_env())
        .with(InstantLayer)
        .init();

    let cli = Cli::parse();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(cli.parallelism as usize)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        let s = span!(tracing::Level::INFO, "Total Runtime");
        let _e = s.enter();
        let mut settings = Settings::new(&cli)?;
        settings.images.extend(collect_stdin()?);

        let out = settings.output.clone();
        let tasks_set = settings.images.into_iter().map(|i| {
            let out = out.clone();
            tokio::spawn(async move {
                let (name, img) = compress_image(i, settings.quality).await?;
                save_compressed_image(name, img, out).await?;

                anyhow::Ok(())
            })
        });

        futures::future::try_join_all(tasks_set).await?;

        Ok(())
    })
}

fn collect_stdin() -> anyhow::Result<Vec<ImageInput>> {
    let mut stdin_images = String::new();

    print!("Enter image URLs or paths whitespace separated: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut stdin_images)?;

    let images = stdin_images
        .split_whitespace()
        .filter_map(|s| {
            let de: Result<_, ConfigError> = ImageInput::deserialize(s.into_deserializer());

            de.ok()
        })
        .collect::<Vec<ImageInput>>();
    println!(
        "Following images added: [{}]",
        images
            .iter()
            .fold(String::new(), |acc, i| acc + &format!("{:?} ", i))
    );

    Ok(images)
}

#[instrument]
async fn compress_image(img: ImageInput, quality: u8) -> anyhow::Result<(String, Vec<u8>)> {
    let name = img.name().to_owned();
    let jpeg_data = Into::<JoinHandle<Result<Vec<u8>, ImageInputError>>>::into(img).await??;

    let image = turbojpeg::decompress(&jpeg_data, turbojpeg::PixelFormat::RGB)?;
    Ok((
        name,
        turbojpeg::compress(image.as_deref(), quality as i32, turbojpeg::Subsamp::Sub2x2)
            .unwrap()
            .as_ref()
            .to_vec(),
    ))
}

#[instrument]
async fn save_compressed_image(
    name: String,
    img: Vec<u8>,
    output_dir: PathBuf,
) -> anyhow::Result<()> {
    tokio::fs::write(output_dir.join(name), img).await?;
    Ok(())
}
