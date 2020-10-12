use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use color_eyre::Result;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use user_service::UserService;
mod user_router;
mod user_service;

pub struct ServiceManager {
    user: UserService,
}

impl ServiceManager {
    pub fn new(user: UserService) -> Self {
        ServiceManager { user }
    }
}

pub struct AppState {
    service_manager: ServiceManager,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let client_options = ClientOptions::parse(&database_url).unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
    let db = client.database(&database_name);
    let user_collection_name =
        env::var("USER_COLLECTION_NAME").expect("USER_COLLECTION_NAME is not set in .env file");
    let user_collection = db.collection(&user_collection_name);
    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");

    HttpServer::new(move || {
        let user_service_worker = UserService::new(user_collection.clone());
        let service_manager = ServiceManager::new(user_service_worker);

        let cors_middleware = Cors::new()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .finish();

        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .data(AppState { service_manager })
            .configure(user_router::init)
    })
    .bind(server_url)?
    .run()
    // need question mark after await
    .await?;

    Ok(())
}
