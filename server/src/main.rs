use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use env_logger::Env;
use serde::Serialize;

mod handlers;
mod models;
mod queries;
mod repository;
mod routes;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}

async fn route_not_found_error() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(Response {
        status: "error".to_string(),
        message: "Route not found".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let tracker_db = repository::database::Database::new();
    let app_data = web::Data::new(tracker_db);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(routes::configuration)
            .service(health)
            .default_service(web::route().to(route_not_found_error))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((
        std::env::var("SERVER_ADDRESS")
            .expect("Environment variable 'SERVER_ADDRESS' must be set!"),
        std::env::var("SERVER_PORT")
            .expect("Environment variable 'SERVER_PORT' must be set!")
            .parse()
            .unwrap(),
    ))?
    .run()
    .await
}
