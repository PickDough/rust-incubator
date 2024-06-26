//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use paperclip::actix::Apiv2Schema;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Apiv2Schema)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub slug: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub permissions: Json,
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
