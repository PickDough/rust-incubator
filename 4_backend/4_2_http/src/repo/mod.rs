use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{prelude::Roles, roles as db_roles};

pub mod roles;
pub mod users;

#[derive(Debug, Clone)]
pub struct UserRolesRepository(DatabaseConnection);

impl UserRolesRepository {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let conn = Database::connect(url).await?;
        Ok(Self(conn))
    }

    pub async fn default_role(&self) -> anyhow::Result<db_roles::Model> {
        let role = Roles::find()
            .filter(db_roles::Column::Slug.eq("default"))
            .one(&self.0)
            .await?;

        Ok(role.unwrap())
    }
}
