use clap::ValueEnum;
use smart_default::SmartDefault;

use crate::entities::sea_orm_active_enums::SexType;

use super::Command;

pub struct Create {
    pub name: String,

    pub sex: Sex,

    pub role: Option<String>,
}

impl Command for Create {}

pub struct Get {
    pub id: i32,
}
impl Command for Get {}

pub struct GetAll {}
impl Command for GetAll {}

pub struct Update {
    pub id: i32,

    pub name: Option<String>,

    pub sex: Option<Sex>,
}
impl Command for Update {}

pub struct Delete {
    pub id: i32,
}
impl Command for Delete {}

#[derive(Debug, ValueEnum, Clone, SmartDefault)]
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
