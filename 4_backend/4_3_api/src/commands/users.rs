use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::entities::sea_orm_active_enums::SexType;

use super::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
    pub name: String,

    pub sex: Sex,

    pub role: Option<String>,
}

impl Command for Create {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Get {
    pub id: i32,
}
impl Command for Get {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAll {}
impl Command for GetAll {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub id: i32,

    pub name: Option<String>,

    pub sex: Option<Sex>,
}
impl Command for Update {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delete {
    pub id: i32,
}
impl Command for Delete {}

#[derive(Debug, ValueEnum, Clone, SmartDefault, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
    #[default]
    Other,
}

impl Into<SexType> for Sex {
    fn into(self) -> SexType {
        match self {
            Sex::Male => SexType::Male,
            Sex::Female => SexType::Female,
            Sex::Other => SexType::Other,
        }
    }
}
