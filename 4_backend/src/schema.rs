use actix_jwt_auth_middleware::{AuthResult, TokenSigner};
use actix_web::HttpResponse;
use juniper::{
    graphql_object, graphql_value, EmptySubscription, FieldError, FieldResult, GraphQLInputObject,
};
use jwt_compact::alg::Ed25519;
use serde::Deserialize;
use sqlx::{prelude::FromRow, Pool, Postgres};
use uuid::Uuid;

use crate::UserIdentity;

#[derive(Debug, Default, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(GraphQLInputObject, Deserialize)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub password: String,
}

pub struct Context {
    pub pool: Pool<Postgres>,
    pub session_user: UserIdentity,
}
impl juniper::Context for Context {}

fn maybe_internal_error<T, E>(res: Result<T, E>) -> FieldResult<T> {
    if let Ok(val) = res {
        return Ok(val);
    }
    Err(FieldError::new(
        "Internal error",
        graphql_value!({ "internal_error": "internal error" }),
    ))
}

fn parse_uuid(s: &str) -> FieldResult<Uuid> {
    Uuid::parse_str(s).map_err(|_| {
        FieldError::new(
            "Invalid UUID",
            graphql_value!({ "invalid_uuid": "invalid UUID" }),
        )
    })
}

#[graphql_object(Context = Context)]
impl User {
    fn id(&self) -> String {
        self.id.hyphenated().to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn friends(&self, context: &Context) -> FieldResult<Vec<User>> {
        let frnds = sqlx::query_as!(
            User,
            r#"
            select id, name from
            (
                SELECT id, name
                FROM users
                INNER JOIN friends ON users.id = friends.f1_id OR users.id = friends.f2_id
                WHERE friends.f1_id = $1 OR friends.f2_id = $1
            )
            where id != $1
            "#,
            self.id
        )
        .fetch_all(&context.pool)
        .await;

        maybe_internal_error(frnds)
    }
}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    async fn user(context: &Context, id: String) -> FieldResult<User> {
        let uuid = parse_uuid(&id)?;
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name
            FROM users
            WHERE id = $1
            "#,
            &uuid
        )
        .fetch_one(&context.pool)
        .await;

        maybe_internal_error(user)
    }

    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT id, name FROM users")
            .fetch_all(&context.pool)
            .await;

        maybe_internal_error(users)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn add_friend(context: &Context, friend: String) -> FieldResult<bool> {
        let uuid1 = parse_uuid(&friend)?;

        if context.session_user.id == uuid1 {
            return Err(FieldError::new(
                "Cannot add self as friend",
                graphql_value!({ "self_friend": "cannot add self as friend" }),
            ));
        }

        let res = sqlx::query!(
            r#"
            INSERT INTO friends (f1_id, f2_id)
            VALUES ($1, $2)
            "#,
            uuid1,
            context.session_user.id
        )
        .execute(&context.pool)
        .await;

        maybe_internal_error(res).map(|_| true)
    }
}

pub type AuthorizedSchema =
    juniper::RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn schema() -> AuthorizedSchema {
    AuthorizedSchema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
