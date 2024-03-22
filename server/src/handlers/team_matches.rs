use crate::models::team_matches::NewTeamMatch;
use crate::repository::{database::Database, queries::team_matches};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(team_matches::load_all(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create(db: web::Data<Database>, data: web::Json<NewTeamMatch>) -> HttpResponse {
    match team_matches::insert(&mut db.pool.get().unwrap(), &data) {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating team match!"),
    }
}

#[get("/{id}")]
async fn get(db: web::Data<Database>, team_match_id: web::Path<i32>) -> HttpResponse {
    match team_matches::find_by_id(&mut db.pool.get().unwrap(), &team_match_id) {
        Some(value) => HttpResponse::Ok().json(value),
        None => HttpResponse::NotFound().body(format!(
            "Could not find team match with id '{0}'",
            &team_match_id
        )),
    }
}

#[delete("/{id}")]
async fn delete(db: web::Data<Database>, match_id: web::Path<i32>) -> HttpResponse {
    match team_matches::delete(&mut db.pool.get().unwrap(), &match_id) {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting team match!"),
    }
}

#[get("/team/{id}")]
async fn get_by_team_id(db: web::Data<Database>, team_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().json(team_matches::filter_by_team_id(
        &mut db.pool.get().unwrap(),
        &team_id,
    ))
}
