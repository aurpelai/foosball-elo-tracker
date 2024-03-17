use crate::repository::players;
use crate::{
    models::player::{NewPlayer, Player},
    repository::database::Database,
};
use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("")]
async fn get_players(db: web::Data<Database>) -> HttpResponse {
    let connection = db.pool.get().unwrap();
    let result = players::get_players(connection);
    HttpResponse::Ok().json(result)
}

#[get("/{id}")]
async fn get_player(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let connection = db.pool.get().unwrap();
    let id = path.into_inner();
    let result = players::get_player(connection, id);
    match result {
        Some(player) => HttpResponse::Ok().json(player),
        None => HttpResponse::NotFound().body(format!("Could not find player with id: {id}")),
    }
}

// TODO get all teams player has played in

#[delete("/{id}")]
async fn delete_player(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let connection = db.pool.get().unwrap();
    let result = players::delete_player(connection, path.into_inner());
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("")]
async fn update_player(db: web::Data<Database>, player: web::Json<Player>) -> HttpResponse {
    let connection = db.pool.get().unwrap();
    let result = players::update_player(connection, player.into_inner());
    match result {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("")]
async fn create_player(db: web::Data<Database>, player: web::Json<NewPlayer>) -> HttpResponse {
    let connection = db.pool.get().unwrap();
    let result = players::create_player(connection, player.into_inner());
    match result {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

// TODO allow updating a player using only partial info, i.e. id and the updated data
// This should probably be done using a post call (put is probably more semantic when used with full data)
