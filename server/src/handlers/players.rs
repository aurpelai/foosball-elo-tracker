use crate::models::player::{NewPlayer, Player};
use crate::repository::{database::Database, queries::players};

use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("")]
async fn get_all_players(db: web::Data<Database>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = players::load_players(&mut connection);
    HttpResponse::Ok().json(result)
}

#[post("")]
async fn create_player(db: web::Data<Database>, data: web::Json<NewPlayer>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = players::insert_player(&mut connection, data.into_inner());
    match result {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("")]
async fn update_player(db: web::Data<Database>, data: web::Json<Player>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = players::update_player(&mut connection, data.into_inner());
    match result {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[get("/{id}")]
async fn get_player_by_id(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let id = path.into_inner();
    let result = players::find_player_by_id(&mut connection, id);
    match result {
        Some(player) => HttpResponse::Ok().json(player),
        None => HttpResponse::NotFound().body(format!("Could not find player with id: '{id}'")),
    }
}

// TODO get all teams player has played in

#[delete("/{id}")]
async fn delete_player_by_id(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = players::delete_player(&mut connection, path.into_inner());
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
