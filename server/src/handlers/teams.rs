use crate::models::team::NewTeam;
use crate::repository::{database::Database, queries::teams};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all_teams(db: web::Data<Database>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = teams::load_teams(&mut connection);
    HttpResponse::Ok().json(result)
}

#[post("")]
async fn create_team(db: web::Data<Database>, data: web::Json<NewTeam>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = teams::find_or_insert_team(&mut connection, data.into_inner());
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        // TODO be more descriptive
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

// TODO allow searching for teams with player ids

#[get("/{id}")]
async fn get_team_by_id(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let id = path.into_inner();
    let result = teams::find_team_by_id(&mut connection, id);
    match result {
        Some(team) => HttpResponse::Ok().json(team),
        None => HttpResponse::NotFound().body(format!("Could not find team with id '{id}'")),
    }
}

#[delete("/{id}")]
async fn delete_team_by_id(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let mut connection = db.pool.get().unwrap();
    let result = teams::delete_team(&mut connection, path.into_inner());
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        // TODO be more descriptive
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
