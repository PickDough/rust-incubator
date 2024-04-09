use actix_cors::Cors;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{Authority, FromRequest, TokenSigner};
use actix_web::post;
use actix_web::web::Json;
use actix_web::{
    get,
    middleware::Logger,
    route,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use schema::{Context, UserInput};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;

use crate::schema::AuthorizedSchema;

mod schema;

/// GraphQL endpoint
#[route("/", method = "GET", method = "POST")]
pub async fn graphql(
    session_user: UserIdentity,
    pool: web::Data<Pool<Postgres>>,
    schema: web::Data<AuthorizedSchema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        pool: pool.get_ref().to_owned(),
        session_user: session_user.to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

/// GraphiQL UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

pub fn config_graphql(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(schema::schema()))
        .service(graphql)
        .service(graphql_playground);
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRequest)]
struct UserIdentity {
    id: Uuid,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rust:incubator@localhost/graphql")
        .await?;
    let KeyPair {
        pk: public_key,
        sk: secret_key,
    } = KeyPair::generate();

    HttpServer::new(move || {
        let authority = Authority::<UserIdentity, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(secret_key.clone())
                    .algorithm(Ed25519)
                    .build()
                    .expect(""),
            ))
            .verifying_key(public_key)
            .build()
            .expect("");

        App::new()
            .app_data(Data::new(pool.clone()))
            .use_jwt(authority, web::scope("/graphql").configure(config_graphql))
            .service(login)
            .service(register)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[post("/login")]
async fn login(
    pool: web::Data<Pool<Postgres>>,
    cookie_signer: web::Data<TokenSigner<UserIdentity, Ed25519>>,
    user_input: Json<UserInput>,
) -> HttpResponse {
    let user = sqlx::query!(
        r#"
        SELECT id, name, password
        FROM users
        WHERE name = $1
        "#,
        user_input.name
    )
    .fetch_one(pool.get_ref())
    .await;
    if user.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let user = user.unwrap();

    if !pwhash::bcrypt::verify(&user_input.password, &user.password) {
        return HttpResponse::Unauthorized().finish();
    }

    let user = UserIdentity { id: user.id };

    HttpResponse::Ok()
        .cookie(cookie_signer.create_access_cookie(&user).unwrap())
        .cookie(cookie_signer.create_refresh_cookie(&user).unwrap())
        .body("You are now logged in")
}

#[post("/register")]
async fn register(
    pool: web::Data<Pool<Postgres>>,
    cookie_signer: web::Data<TokenSigner<UserIdentity, Ed25519>>,
    user_input: Json<UserInput>,
) -> HttpResponse {
    let id = Uuid::new_v4();
    let password = pwhash::bcrypt::hash(&user_input.password);
    if password.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let password = password.unwrap();

    let user = sqlx::query!(
        r#"
        INSERT INTO users (id, name, password)
        VALUES ($1, $2, $3)
        RETURNING id, name
        "#,
        id,
        user_input.name,
        password,
    )
    .fetch_one(pool.get_ref())
    .await;

    if user.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let user = user.unwrap();

    let user = UserIdentity { id: user.id };

    HttpResponse::Ok()
        .cookie(cookie_signer.create_access_cookie(&user).unwrap())
        .cookie(cookie_signer.create_refresh_cookie(&user).unwrap())
        .body("You are now logged in")
}
