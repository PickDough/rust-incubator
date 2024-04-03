use std::env;

use actix_cors::Cors;
use actix_web::error::{self};
use actix_web::web::service;
use actix_web::Error;
use actix_web::{App, HttpResponse, HttpServer};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
    Apiv2Schema, OpenApiExt,
};
use step_4_3::entities::users::UserWithRoles;
use step_4_3::entities::{self, role_user};
use step_4_3::{commands::*, repo::UserRolesRepository};
mod client;

struct AppState {
    db: UserRolesRepository,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let db = UserRolesRepository::new(&env::var("DATABASE_URL")?).await?;
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard();
        let app = App::new()
            .wrap_api()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(cors)
            .service(
                web::scope("/users")
                    .route("create", web::post().to(create_user))
                    .route("get", web::post().to(get_user))
                    .route("get_all", web::post().to(get_all_users))
                    .route("update", web::post().to(update_user))
                    .route("delete", web::post().to(delete_user)),
            )
            .service(
                web::scope("/roles")
                    .route("create", web::post().to(create_role))
                    .route("get", web::post().to(get_role))
                    .route("assign", web::post().to(assign_role))
                    .route("get_all", web::post().to(get_all_roles))
                    .route("update", web::post().to(update_role))
                    .route("delete", web::post().to(delete_role)),
            )
            .with_json_spec_at("/api/spec/v2")
            .with_swagger_ui_at("/api/spec/v2/swagger");
        app.build()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[api_v2_operation]
async fn create_user(
    req: web::Json<users::Create>,
    state: web::Data<AppState>,
) -> Result<Json<entities::users::Model>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}

#[api_v2_operation]
async fn get_user(
    req: web::Json<users::Get>,
    state: web::Data<AppState>,
) -> Result<Json<UserWithRoles>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    if let Ok(maybe) = res {
        if let Some(m) = maybe {
            Ok(Json(UserWithRoles::new(m.0, m.1)))
        } else {
            Err(error::ErrorNotFound("Role not found"))
        }
    } else {
        Err(error::ErrorInternalServerError("Internal Server Error"))
    }
}

#[api_v2_operation]
async fn get_all_users(
    req: web::Json<users::GetAll>,
    state: web::Data<AppState>,
) -> Result<Json<Vec<UserWithRoles>>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    let res = res.map(|vec| {
        vec.into_iter()
            .map(|(u, r)| UserWithRoles::new(u, r))
            .collect::<Vec<_>>()
    });
    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}

#[api_v2_operation]
async fn update_user(
    req: web::Json<users::Update>,
    state: web::Data<AppState>,
) -> Result<Json<entities::users::Model>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}

#[api_v2_operation]
async fn delete_user(
    req: web::Json<users::Delete>,
    state: web::Data<AppState>,
) -> Result<Json<()>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}

#[api_v2_operation]
async fn create_role(
    req: web::Json<roles::Create>,
    state: web::Data<AppState>,
) -> Result<Json<entities::roles::Model>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}

#[api_v2_operation]
async fn get_role(
    req: web::Json<roles::Get>,
    state: web::Data<AppState>,
) -> Result<Json<entities::roles::Model>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    if let Ok(maybe) = res {
        if let Some(m) = maybe {
            Ok(Json(m))
        } else {
            Err(error::ErrorNotFound("Role not found"))
        }
    } else {
        Err(error::ErrorInternalServerError("Internal Server Error"))
    }
}

#[api_v2_operation]
async fn assign_role(
    req: web::Json<roles::Assign>,
    state: web::Data<AppState>,
) -> Result<Json<()>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}

#[api_v2_operation]
async fn get_all_roles(
    req: web::Json<roles::GetAll>,
    state: web::Data<AppState>,
) -> Result<Json<Vec<entities::roles::Model>>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}

#[api_v2_operation]
async fn update_role(
    req: web::Json<roles::Update>,
    state: web::Data<AppState>,
) -> Result<Json<entities::roles::Model>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}

#[api_v2_operation]
async fn delete_role(
    req: web::Json<roles::Delete>,
    state: web::Data<AppState>,
) -> Result<Json<()>, Error> {
    let res = state.db.handle(req.into_inner()).await;

    res.map(Json)
        .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))
}
