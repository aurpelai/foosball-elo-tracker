use crate::models::team::NewTeam;
use crate::repository::{database::Database, queries::teams};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all_teams(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(teams::load_teams(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create_team(db: web::Data<Database>, data: web::Json<NewTeam>) -> HttpResponse {
    match teams::find_or_insert_team(&mut db.pool.get().unwrap(), &data.into_inner()) {
        Ok(team) => HttpResponse::Ok().json(team),
        // TODO be more descriptive
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

// TODO allow searching for teams with player ids

#[get("/{id}")]
async fn get_team_by_id(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let id = &path.into_inner();
    match teams::find_team_by_id(&mut db.pool.get().unwrap(), id) {
        Some(team) => HttpResponse::Ok().json(team),
        None => HttpResponse::NotFound().body(format!("Could not find team with id '{0}'", id)),
    }
}

#[delete("/{id}")]
async fn delete_team_by_id(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    match teams::delete_team(&mut db.pool.get().unwrap(), &path.into_inner()) {
        Ok(team) => HttpResponse::Ok().json(team),
        // TODO be more descriptive
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
