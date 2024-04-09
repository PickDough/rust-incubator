use anyhow::Ok;
use clap::Subcommand;
use serde::{Deserialize, Serialize};

use crate::{commands::roles, repo::UserRolesRepository};

use super::{
    users::{Create, Delete, Get, GetAll, Sex, Update},
    CommandHandler,
};

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum UserCommands {
    Create {
        #[clap(short, long, required = true)]
        name: String,

        #[clap(short, long, default_value_t, value_enum)]
        sex: Sex,

        #[clap(short, long)]
        role: Option<String>,
    },
    Get {
        #[clap(short, required = true)]
        id: i32,
    },
    GetAll,
    Update {
        #[clap(short, long, required = true)]
        id: i32,

        #[clap(short, long)]
        name: Option<String>,

        #[clap(short, long, value_enum)]
        sex: Option<Sex>,
    },
    Delete {
        #[clap(short, long, required = true)]
        id: i32,
    },
}

impl UserCommands {
    pub async fn apply(self, repo: &UserRolesRepository) -> anyhow::Result<String> {
        match self {
            UserCommands::Create { name, sex, role } => {
                let cmd = Create { name, sex, role };
                let res = repo.handle(cmd).await?;

                Ok(format!("Created user: {:?}", res))
            }
            UserCommands::Get { id } => {
                let cmd = Get { id };
                let res = repo.handle(cmd).await?;

                Ok(format!("User: {:?}", res))
            }
            UserCommands::GetAll => {
                let cmd = GetAll {};
                let res = repo.handle(cmd).await?;

                Ok(format!("Users: {:?}", res))
            }
            UserCommands::Update { id, name, sex } => {
                let cmd = Update { id, name, sex };
                let res = repo.handle(cmd).await?;

                Ok(format!("Updated user: {:?}", res))
            }
            UserCommands::Delete { id } => {
                let cmd = Delete { id };
                repo.handle(cmd).await?;

                Ok(format!("Deleted user: id {}", id))
            }
        }
    }
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum RoleCommands {
    Create {
        #[clap(short, long, required = true)]
        name: String,

        #[clap(short, long, num_args = 1.., value_delimiter = ' ', required = true)]
        permissions: Vec<String>,
    },
    Get {
        #[clap(short, long, required = true)]
        role: String,
    },
    GetAll,
    Assign {
        #[clap(short, long, required = true)]
        user_id: i32,

        #[clap(short, long, required = true)]
        role: String,
    },
    Update {
        #[clap(short, long, required = true)]
        role: String,

        #[clap(short, long)]
        name: Option<String>,

        #[clap(short, long)]
        permissions: Option<Vec<String>>,
    },
    Delete {
        #[clap(short, long, required = true)]
        role: String,
    },
}

impl RoleCommands {
    pub async fn apply(self, repo: &UserRolesRepository) -> anyhow::Result<String> {
        match self {
            RoleCommands::Create { name, permissions } => {
                let cmd = roles::Create { name, permissions };
                let res = repo.handle(cmd).await?;

                Ok(format!("Created role: {:?}", res))
            }
            RoleCommands::Get { role } => {
                let cmd = roles::Get { role };
                let res = repo.handle(cmd).await?;

                Ok(format!("Role: {:?}", res))
            }
            RoleCommands::GetAll => {
                let cmd = roles::GetAll {};
                let res = repo.handle(cmd).await?;

                Ok(format!("Roles: {:?}", res))
            }
            RoleCommands::Assign { user_id, role } => {
                let cmd = roles::Assign {
                    user_id,
                    role: role.clone(),
                };
                repo.handle(cmd).await?;

                Ok(format!("Assigned role: {} to user: {}", role, user_id))
            }
            RoleCommands::Update {
                role,
                name,
                permissions,
            } => {
                let cmd = roles::Update {
                    role,
                    name,
                    permissions,
                };
                let res = repo.handle(cmd).await?;

                Ok(format!("Updated role: {:?}", res))
            }
            RoleCommands::Delete { role } => {
                let cmd = roles::Delete { role: role.clone() };
                repo.handle(cmd).await?;

                Ok(format!("Deleted role: {}", role))
            }
        }
    }
}
