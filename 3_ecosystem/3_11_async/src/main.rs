use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::Parser;
use futures::future;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::create_dir;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::runtime;

/// Program to concurrently download html documents.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[clap(short, long, value_hint = clap::ValueHint::FilePath, value_parser = validator_path_buf)]
    file: PathBuf,

    #[arg(long, default_value_t = std::thread::available_parallelism().unwrap().get())]
    max_threads: usize,
}

pub fn validator_path_buf(s: &str) -> Result<PathBuf, clap::Error> {
    let path = Path::new(s).to_path_buf();

    if path.exists() {
        Ok(path)
    } else {
        let mut err = clap::Error::new(ErrorKind::ValueValidation);
        err.insert(
            ContextKind::InvalidValue,
            ContextValue::String("--file".to_owned()),
        );
        Err(err)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let runtime = runtime::Builder::new_current_thread()
        .worker_threads(cli.max_threads)
        .enable_all()
        .build()?;

    runtime.block_on(async {
        let documents = tokio::fs::read_to_string(&cli.file).await?;
        let documents = documents.lines().map(str::to_owned).collect::<Vec<_>>();

        let mut dir = cli.file.clone();
        dir.pop();
        dir.push("documents");
        let _ = create_dir(dir.as_path()).await;
        let dir = Arc::new(dir);

        let tasks = documents.into_iter().map(|url| {
            tokio::spawn({
                let d = dir.clone();
                async move {
                    let response = reqwest::get(url.as_str()).await?;
                    let doc = response.text().await?;

                    let mut file_name = url.split('/');
                    file_name.next_back();
                    let file_name = file_name.fold(String::new(), |mut acc, s| {
                        acc.push_str(s);
                        acc
                    });

                    let mut file =
                        File::create(format!("{}/{}.index.html", d.to_str().unwrap(), file_name))
                            .await?;

                    file.write_all(doc.as_bytes()).await?;
                    file.flush().await?;

                    Ok::<(), anyhow::Error>(())
                }
            })
        });

        future::join_all(tasks).await;

        Ok(())
    })
}

