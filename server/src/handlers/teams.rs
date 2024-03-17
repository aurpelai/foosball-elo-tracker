use crate::repository::teams;
use crate::{models::team::NewTeam, repository::database::Database};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_teams(db: web::Data<Database>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = teams::get_teams(&mut connection);
    HttpResponse::Ok().json(result)
}

// TODO allow searching for teams with player ids

#[get("/{id}")]
async fn get_team(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let id = path.into_inner();
    let result = teams::get_team(&mut connection, id);
    match result {
        Some(team) => HttpResponse::Ok().json(team),
        None => HttpResponse::NotFound().body(format!("Could not find team with id '{id}'")),
    }
}

#[delete("/{id}")]
async fn delete_team(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = teams::delete_team(&mut connection, path.into_inner());
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        // TODO be more descriptive
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("")]
async fn create_team(db: web::Data<Database>, data: web::Json<NewTeam>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = teams::create_or_get_team(&mut connection, data.into_inner());
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        // TODO be more descriptive
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
