use crate::models::teams::NewTeam;
use crate::repository::{
    database::Database,
    queries::{matches, teams},
};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(teams::load_all(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create(db: web::Data<Database>, data: web::Json<NewTeam>) -> HttpResponse {
    match teams::find_or_insert(&mut db.pool.get().unwrap(), &data) {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating team!"),
    }
}

#[get("/{id}")]
async fn get(db: web::Data<Database>, team_id: web::Path<i32>) -> HttpResponse {
    match teams::find_by_id(&mut db.pool.get().unwrap(), &team_id) {
        Some(team) => HttpResponse::Ok().json(team),
        None => {
            HttpResponse::NotFound().body(format!("Could not find team with id '{0}'", &team_id))
        }
    }
}

#[delete("/{id}")]
async fn delete(db: web::Data<Database>, team_id: web::Path<i32>) -> HttpResponse {
    match teams::delete(&mut db.pool.get().unwrap(), &team_id) {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting team!"),
    }
}

#[get("/{id}/matches")]
async fn get_matches(db: web::Data<Database>, team_id: web::Path<i32>) -> HttpResponse {
    match teams::find_by_id(&mut db.pool.get().unwrap(), &team_id) {
        Some(_) => HttpResponse::Ok().json(matches::find_by_team_id(
            &mut db.pool.get().unwrap(),
            &team_id,
        )),
        None => {
            HttpResponse::NotFound().body(format!("Could not find team with id '{0}'", &team_id))
        }
    }
}

#[get("/match/{id}")]
async fn get_by_match_id(db: web::Data<Database>, match_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().json(teams::filter_by_match_id(
        &mut db.pool.get().unwrap(),
        &match_id,
    ))
}
