use crate::queries::{matches, teams};
use crate::repository::database::Database;

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all(db: web::Data<Database>) -> HttpResponse {
    match teams::get_all_teams(&mut db.pool.get().unwrap()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error loading teams!"),
    }
}

#[post("")]
async fn create(db: web::Data<Database>, data: web::Json<Vec<i32>>) -> HttpResponse {
    match teams::get_or_insert(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating team!"),
    }
}

#[get("/{id}")]
async fn get(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match teams::get_team(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError()
            .body(format!("Error while getting team with id '{}'!", data)),
    }
}

#[delete("/{id}")]
async fn delete(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match teams::delete(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting team!"),
    }
}

#[get("/{id}/matches")]
async fn get_matches(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match matches::get_matches_by_team(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError()
            .body(format!("Error while fetching matches for team '{}'!", data)),
    }
}

#[get("/{id}/rating")]
async fn get_rating(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match teams::find_latest_team_rating(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError()
            .body(format!("Error while fetching rating for team '{}'!", data)),
    }
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(delete);
    cfg.service(get);
    cfg.service(get_all);
    cfg.service(get_rating);
    cfg.service(get_matches);
}
