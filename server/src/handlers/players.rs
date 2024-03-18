use crate::models::player::{NewPlayer, Player};
use crate::repository::{database::Database, queries::players};

use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("")]
async fn get_all_players(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(players::load_players(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create_player(db: web::Data<Database>, data: web::Json<NewPlayer>) -> HttpResponse {
    match players::insert_player(&mut db.pool.get().unwrap(), &data.into_inner()) {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("")]
async fn update_player(db: web::Data<Database>, data: web::Json<Player>) -> HttpResponse {
    match players::update_player(&mut db.pool.get().unwrap(), &data.into_inner()) {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[get("/{id}")]
async fn get_player_by_id(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    match players::find_player_by_id(&mut db.pool.get().unwrap(), &id) {
        Some(player) => HttpResponse::Ok().json(player),
        None => HttpResponse::NotFound().body(format!("Could not find player with id: '{0}'", &id)),
    }
}

// TODO get all teams player has played in

#[delete("/{id}")]
async fn delete_player_by_id(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    match players::delete_player(&mut db.pool.get().unwrap(), &path.into_inner()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
