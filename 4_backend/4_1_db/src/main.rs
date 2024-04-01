use anyhow::Ok;
use clap::{Parser, Subcommand};
use commands::clap::{RoleCommands, UserCommands};
use repo::UserRolesRepository;

mod commands;
mod entities;
mod repo;

/// Cli for defined CRUD operations on `User` and `Role` entities
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
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
    async fn apply(self, db: &UserRolesRepository) -> anyhow::Result<()> {
        match self {
            Commands::User { command } => command.apply(db).await,
            Commands::Role { command } => command.apply(db).await,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();
    let db = UserRolesRepository::new(&std::env::var("DATABASE_URL")?).await?;

    cli.command.apply(&db).await?;

    Ok(())
}
