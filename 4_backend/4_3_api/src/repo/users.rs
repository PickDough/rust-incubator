use sea_orm::{ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set, TryIntoModel};

use crate::{
    commands::{
        users::{Create, Delete, Get, GetAll, Update},
        CommandHandler,
    },
    entities::{prelude::*, role_user, users},
};

use super::UserRolesRepository;

impl CommandHandler<Create> for UserRolesRepository {
    type Output = anyhow::Result<users::Model>;

    async fn handle(&self, command: Create) -> Self::Output {
        let mut model = users::ActiveModel {
            id: NotSet,
            name: Set(command.name),
            sex: Set(command.sex.into()),
        };
        let user = Users::insert(model.clone()).exec(&self.0).await?;
        model.id = Set(user.last_insert_id);

        let role_slug = if let Some(role) = command.role {
            role
        } else {
            self.default_role().await?.slug
        };

        RoleUser::insert(role_user::ActiveModel {
            id: NotSet,
            user_id: Set(user.last_insert_id),
            role_slug: Set(role_slug.clone()),
        })
        .exec(&self.0)
        .await?;

        Ok(model.try_into_model()?)
    }
}

impl CommandHandler<Get> for UserRolesRepository {
    type Output = anyhow::Result<Option<(users::Model, Vec<role_user::Model>)>>;

    async fn handle(&self, command: Get) -> Self::Output {
        let user = Users::find()
            .filter(users::Column::Id.eq(command.id))
            .find_with_related(RoleUser)
            .all(&self.0)
            .await?;

        Ok(user.into_iter().next())
    }
}

impl CommandHandler<GetAll> for UserRolesRepository {
    type Output = anyhow::Result<Vec<(users::Model, Vec<role_user::Model>)>>;

    async fn handle(&self, _: GetAll) -> Self::Output {
        let users = Users::find()
            .find_with_related(RoleUser)
            .all(&self.0)
            .await?;

        Ok(users)
    }
}

impl CommandHandler<Update> for UserRolesRepository {
    type Output = anyhow::Result<users::Model>;

    async fn handle(&self, command: Update) -> Self::Output {
        let user = Users::find_by_id(command.id).one(&self.0).await?;

        let mut user: users::ActiveModel = user.unwrap().into();

        user.name = if let Some(name) = command.name {
            Set(name)
        } else {
            user.name
        };

        user.sex = if let Some(sex) = command.sex {
            Set(sex.into())
        } else {
            user.sex
        };

        Users::update(user.clone()).exec(&self.0).await?;

        Ok(user.try_into_model()?)
    }
}

impl CommandHandler<Delete> for UserRolesRepository {
    type Output = anyhow::Result<()>;

    async fn handle(&self, command: Delete) -> Self::Output {
        Users::delete_by_id(command.id).exec(&self.0).await?;

        Ok(())
    }
}
