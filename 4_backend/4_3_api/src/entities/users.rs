//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::SexType;
use paperclip::actix::Apiv2Schema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Apiv2Schema)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub sex: SexType,
}

#[derive(Apiv2Schema, Serialize)]
pub struct UserWithRoles {
    user: Model,
    roles: Vec<super::role_user::Model>,
}

impl UserWithRoles {
    pub fn new(user: Model, roles: Vec<super::role_user::Model>) -> Self {
        Self { user, roles }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_user::Entity")]
    RoleUser,
}

impl Related<super::role_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}