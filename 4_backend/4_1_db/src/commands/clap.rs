use anyhow::Ok;
use clap::Subcommand;

use crate::{commands::roles, repo::UserRolesRepository};

use super::{
    users::{Create, Delete, Get, GetAll, Sex, Update},
    CommandHandler,
};

#[derive(Debug, Subcommand)]
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
    pub async fn apply(self, repo: &UserRolesRepository) -> anyhow::Result<()> {
        match self {
            UserCommands::Create { name, sex, role } => {
                let cmd = Create { name, sex, role };
                let res = repo.handle(cmd).await?;

                println!("Created user: {:?}", res);

                Ok(())
            }
            UserCommands::Get { id } => {
                let cmd = Get { id };
                let res = repo.handle(cmd).await?;

                println!("User: {:?}", res);

                Ok(())
            }
            UserCommands::GetAll => {
                let cmd = GetAll {};
                let res = repo.handle(cmd).await?;

                println!("Users: {:?}", res);

                Ok(())
            }
            UserCommands::Update { id, name, sex } => {
                let cmd = Update { id, name, sex };
                let res = repo.handle(cmd).await?;

                println!("Updated user: {:?}", res);

                Ok(())
            }
            UserCommands::Delete { id } => {
                let cmd = Delete { id };
                repo.handle(cmd).await?;

                println!("Deleted user: id {}", id);

                Ok(())
            }
        }
    }
}

#[derive(Debug, Subcommand)]
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
    pub async fn apply(self, repo: &UserRolesRepository) -> anyhow::Result<()> {
        match self {
            RoleCommands::Create { name, permissions } => {
                let cmd = roles::Create { name, permissions };
                let res = repo.handle(cmd).await?;

                println!("Created role: {:?}", res);

                Ok(())
            }
            RoleCommands::Get { role } => {
                let cmd = roles::Get { role };
                let res = repo.handle(cmd).await?;

                println!("Role: {:?}", res);

                Ok(())
            }
            RoleCommands::GetAll => {
                let cmd = roles::GetAll {};
                let res = repo.handle(cmd).await?;

                println!("Roles: {:?}", res);

                Ok(())
            }
            RoleCommands::Assign { user_id, role } => {
                let cmd = roles::Assign {
                    user_id,
                    role: role.clone(),
                };
                repo.handle(cmd).await?;

                println!("Assigned role: {} to user: {}", role, user_id);

                Ok(())
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

                println!("Updated role: {:?}", res);

                Ok(())
            }
            RoleCommands::Delete { role } => {
                let cmd = roles::Delete { role: role.clone() };
                repo.handle(cmd).await?;

                println!("Deleted role: {}", role);

                Ok(())
            }
        }
    }
}
