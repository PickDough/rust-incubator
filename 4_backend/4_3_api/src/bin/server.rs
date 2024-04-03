use std::env;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
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
            );
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

async fn create_user(req: web::Json<users::Create>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_user(req: web::Json<users::Get>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_all_users(req: web::Json<users::GetAll>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_user(req: web::Json<users::Update>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_user(req: web::Json<users::Delete>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_role(req: web::Json<roles::Create>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_role(req: web::Json<roles::Get>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn assign_role(req: web::Json<roles::Assign>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_all_roles(req: web::Json<roles::GetAll>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_role(req: web::Json<roles::Update>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_role(req: web::Json<roles::Delete>, state: web::Data<AppState>) -> HttpResponse {
    let res = state.db.handle(req.into_inner()).await;
    match res {
        Ok(data) => HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
