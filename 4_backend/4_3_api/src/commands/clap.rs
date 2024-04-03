use anyhow::Ok;
use clap::Subcommand;
use serde::{Deserialize, Serialize};

use crate::commands::roles;

use super::users::{Create, Delete, Get, GetAll, Sex, Update};

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
    pub fn apply(self) -> anyhow::Result<(String, String)> {
        match self {
            UserCommands::Create { name, sex, role } => {
                let cmd = Create { name, sex, role };

                Ok((serde_json::to_string(&cmd)?, "users/create".to_string()))
            }
            UserCommands::Get { id } => {
                let cmd = Get { id };

                Ok((serde_json::to_string(&cmd)?, "users/get".to_string()))
            }
            UserCommands::GetAll => {
                let cmd = GetAll {};

                Ok((serde_json::to_string(&cmd)?, "users/get_all".to_string()))
            }
            UserCommands::Update { id, name, sex } => {
                let cmd = Update { id, name, sex };

                Ok((serde_json::to_string(&cmd)?, "users/update".to_string()))
            }
            UserCommands::Delete { id } => {
                let cmd = Delete { id };

                Ok((serde_json::to_string(&cmd)?, "users/delete".to_string()))
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
    pub fn apply(self) -> anyhow::Result<(String, String)> {
        match self {
            RoleCommands::Create { name, permissions } => {
                let cmd = roles::Create { name, permissions };

                Ok((serde_json::to_string(&cmd)?, "roles/create".to_string()))
            }
            RoleCommands::Get { role } => {
                let cmd = roles::Get { role };

                Ok((serde_json::to_string(&cmd)?, "roles/get".to_string()))
            }
            RoleCommands::GetAll => {
                let cmd = roles::GetAll {};

                Ok((serde_json::to_string(&cmd)?, "roles/get_all".to_string()))
            }
            RoleCommands::Assign { user_id, role } => {
                let cmd = roles::Assign {
                    user_id,
                    role: role,
                };

                Ok((serde_json::to_string(&cmd)?, "roles/assign".to_string()))
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

                Ok((serde_json::to_string(&cmd)?, "roles/update".to_string()))
            }
            RoleCommands::Delete { role } => {
                let cmd = roles::Delete { role };

                Ok((serde_json::to_string(&cmd)?, "roles/delete".to_string()))
            }
        }
    }
}
