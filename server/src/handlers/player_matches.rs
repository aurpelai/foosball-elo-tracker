use crate::models::player_matches::NewPlayerMatch;
use crate::repository::{database::Database, queries::player_matches};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(player_matches::load_all(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create(db: web::Data<Database>, data: web::Json<NewPlayerMatch>) -> HttpResponse {
    match player_matches::insert(&mut db.pool.get().unwrap(), &data) {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating player match!"),
    }
}

#[get("/{id}")]
async fn get(db: web::Data<Database>, player_match_id: web::Path<i32>) -> HttpResponse {
    match player_matches::find_by_id(&mut db.pool.get().unwrap(), &player_match_id) {
        Some(value) => HttpResponse::Ok().json(value),
        None => HttpResponse::NotFound().body(format!(
            "Could not find player match with id '{0}'",
            &player_match_id
        )),
    }
}

#[delete("/{id}")]
async fn delete(db: web::Data<Database>, player_match_id: web::Path<i32>) -> HttpResponse {
    match player_matches::delete(&mut db.pool.get().unwrap(), &player_match_id) {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting player match!"),
    }
}

#[get("/player/{id}")]
async fn get_by_player_id(db: web::Data<Database>, player_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().json(player_matches::find_by_player_id(
        &mut db.pool.get().unwrap(),
        &player_id,
    ))
}
