use crate::models::player::{NewPlayer, Player};
use crate::repository::{
    database::Database,
    queries::{players, teams},
};

use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("")]
async fn get_all(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(players::load_all(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create(db: web::Data<Database>, data: web::Json<NewPlayer>) -> HttpResponse {
    match players::insert(&mut db.pool.get().unwrap(), &data) {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating player!"),
    }
}

#[put("")]
async fn update(db: web::Data<Database>, data: web::Json<Player>) -> HttpResponse {
    match players::update(&mut db.pool.get().unwrap(), &data) {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Error while updating player!"),
    }
}

#[get("/{id}")]
async fn get(db: web::Data<Database>, player_id: web::Path<i32>) -> HttpResponse {
    match players::find_by_id(&mut db.pool.get().unwrap(), &player_id) {
        Some(player) => HttpResponse::Ok().json(player),
        None => HttpResponse::NotFound()
            .body(format!("Could not find player with id: '{0}'", &player_id)),
    }
}

#[delete("/{id}")]
async fn delete(db: web::Data<Database>, player_id: web::Path<i32>) -> HttpResponse {
    match players::delete(&mut db.pool.get().unwrap(), &player_id) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting player!"),
    }
}

#[get("/{id}/teams")]
async fn get_teams(db: web::Data<Database>, player_id: web::Path<i32>) -> HttpResponse {
    match players::find_by_id(&mut db.pool.get().unwrap(), &player_id) {
        Some(_) => HttpResponse::Ok().json(teams::filter_by_player_id(
            &mut db.pool.get().unwrap(),
            &player_id,
        )),
        None => HttpResponse::NotFound()
            .body(format!("Could not find player with id '{0}'", &player_id)),
    }
}

#[get("/match/{id}")]
async fn get_by_match_id(db: web::Data<Database>, match_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().json(players::filter_by_match_id(
        &mut db.pool.get().unwrap(),
        &match_id,
    ))
}
