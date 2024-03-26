use crate::models::players::*;
use crate::queries::{players, teams};
use crate::repository::database::Database;

use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("")]
async fn get_all(db: web::Data<Database>) -> HttpResponse {
    match players::get_all_players(&mut db.pool.get().unwrap()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error loading players!"),
    }
}

#[post("")]
async fn create(db: web::Data<Database>, data: web::Json<PlayerUpsert>) -> HttpResponse {
    match players::insert(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating player!"),
    }
}

#[put("")]
async fn update(db: web::Data<Database>, data: web::Json<Player>) -> HttpResponse {
    match players::update(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error while updating player!"),
    }
}

#[get("/{id}")]
async fn get(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match players::get_player(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError()
            .body(format!("Error while getting player with id '{}'!", data)),
    }
}

#[delete("/{id}")]
async fn delete(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match players::delete(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting player!"),
    }
}

#[get("/{id}/teams")]
async fn get_teams(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match teams::get_teams_of_a_player(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError()
            .body(format!("Error while fetching teams for player '{}'!", data)),
    }
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(delete);
    cfg.service(get);
    cfg.service(get_all);
    cfg.service(get_teams);
    cfg.service(update);
}
