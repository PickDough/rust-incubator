use sea_orm::{ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set, TryIntoModel};
use slug::slugify;

use crate::{
    commands::{
        roles::{Assign, Create, Delete, Get, GetAll, Update},
        CommandHandler,
    },
    entities::{
        prelude::{RoleUser, Roles},
        role_user,
        roles::{self, Model},
    },
};

use super::UserRolesRepository;

impl CommandHandler<Create> for UserRolesRepository {
    type Output = anyhow::Result<Model>;

    async fn handle(&self, command: Create) -> Self::Output {
        let model = roles::ActiveModel {
            name: Set(command.name.clone()),
            slug: Set(slugify(command.name)),
            permissions: Set(serde_json::to_value(&command.permissions)?),
        };

        Roles::insert(model.clone()).exec(&self.0).await?;

        Ok(model.try_into_model()?)
    }
}

impl CommandHandler<Get> for UserRolesRepository {
    type Output = anyhow::Result<Option<Model>>;

    async fn handle(&self, command: Get) -> Self::Output {
        let role = Roles::find()
            .filter(roles::Column::Slug.eq(command.role))
            .one(&self.0)
            .await?;

        Ok(role)
    }
}

impl CommandHandler<GetAll> for UserRolesRepository {
    type Output = anyhow::Result<Vec<Model>>;

    async fn handle(&self, _: GetAll) -> Self::Output {
        let roles = Roles::find().all(&self.0).await?;

        Ok(roles)
    }
}

impl CommandHandler<Update> for UserRolesRepository {
    type Output = anyhow::Result<Model>;

    async fn handle(&self, command: Update) -> Self::Output {
        let role = Roles::find()
            .filter(roles::Column::Slug.eq(command.role))
            .one(&self.0)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Role not found"))?;

        let mut role: roles::ActiveModel = role.into();

        if let Some(name) = command.name {
            role.name = Set(name.clone());
            role.slug = Set(slugify(name));
        }

        if let Some(permissions) = command.permissions {
            role.permissions = Set(serde_json::to_value(&permissions)?);
        }

        Roles::update(role.clone()).exec(&self.0).await?;

        Ok(role.try_into_model()?)
    }
}

impl CommandHandler<Delete> for UserRolesRepository {
    type Output = anyhow::Result<()>;

    async fn handle(&self, command: Delete) -> Self::Output {
        Roles::delete_by_id(command.role).exec(&self.0).await?;

        Ok(())
    }
}

impl CommandHandler<Assign> for UserRolesRepository {
    type Output = anyhow::Result<()>;

    async fn handle(&self, command: Assign) -> Self::Output {
        RoleUser::insert(role_user::ActiveModel {
            id: NotSet,
            user_id: Set(command.user_id),
            role_slug: Set(command.role),
        })
        .exec(&self.0)
        .await?;

        Ok(())
    }
}
