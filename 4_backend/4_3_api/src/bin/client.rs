use anyhow::Result;
use clap::{command, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use step_4_3::commands::clap::{RoleCommands, UserCommands};

extern crate step_4_3;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    let cmd = cli.command.apply()?;

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/".to_string() + &cmd.1)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(cmd.0)
        .send()
        .await?;

    println!("{:?}", res.text().await?);

    Ok(())
}

/// Cli for defined CRUD operations on `User` and `Role` entities
#[derive(Debug, Parser)]
#[command(version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Commands {
    User {
        #[clap(subcommand)]
        command: UserCommands,
    },
    Role {
        #[clap(subcommand)]
        command: RoleCommands,
    },
}

impl Commands {
    pub fn apply(self) -> Result<(String, String)> {
        match self {
            Commands::User { command } => command.apply(),
            Commands::Role { command } => command.apply(),
        }
    }
}
