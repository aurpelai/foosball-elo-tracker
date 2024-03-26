use crate::models::matches::*;
use crate::queries::{matches, players, teams};
use crate::repository::database::Database;

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all(db: web::Data<Database>) -> HttpResponse {
    match matches::get_all_matches(&mut db.pool.get().unwrap()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error loading matches!"),
    }
}

#[post("")]
async fn create(db: web::Data<Database>, data: web::Json<MatchData>) -> HttpResponse {
    match matches::insert(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error while adding match!"),
    }
}

#[get("/{id}")]
async fn get(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match matches::get_match(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => {
            HttpResponse::NotFound().body(format!("Error while getting match with id '{}'", &data))
        }
    }
}

#[get("/{id}/teams")]
async fn get_teams_from_match(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match teams::get_teams_from_match(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError()
            .body(format!("Error getting teams from match '{}'!", data)),
    }
}

#[get("/{id}/players")]
async fn get_players_from_match(db: web::Data<Database>, data: web::Path<i32>) -> HttpResponse {
    match players::get_players_from_match(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError()
            .body(format!("Error getting players from match '{}'!", data)),
    }
}

#[get("/rivalry")]
async fn get_by_rivalry(db: web::Data<Database>, data: web::Json<Vec<i32>>) -> HttpResponse {
    if data.len() != 2 {
        return HttpResponse::NotFound().body(format!(
            "A rivalry must comprise of exactly two teams but tried to search with {}!",
            data.len()
        ));
    }

    match matches::get_matches_between_teams(&mut db.pool.get().unwrap(), &data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => {
            HttpResponse::InternalServerError().body("Error while getting matches for rivalry!")
        }
    }
}

#[delete("/{id}")]
async fn delete(db: web::Data<Database>, match_id: web::Path<i32>) -> HttpResponse {
    match matches::delete(&mut db.pool.get().unwrap(), &match_id) {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting match!"),
    }
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(delete);
    cfg.service(get_by_rivalry);
    cfg.service(get_players_from_match);
    cfg.service(get_teams_from_match);
    cfg.service(get);
    cfg.service(get_all);
}
