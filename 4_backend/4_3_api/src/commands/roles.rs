use super::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
    pub name: String,

    pub permissions: Vec<String>,
}
impl Command for Create {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Get {
    pub role: String,
}
impl Command for Get {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAll {}
impl Command for GetAll {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assign {
    pub user_id: i32,

    pub role: String,
}
impl Command for Assign {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub role: String,

    pub name: Option<String>,

    pub permissions: Option<Vec<String>>,
}
impl Command for Update {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delete {
    pub role: String,
}
impl Command for Delete {}
