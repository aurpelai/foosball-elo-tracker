use crate::{models::team::NewTeam, repository::database::Database};
use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_teams(db: web::Data<Database>) -> HttpResponse {
    let result = db.get_teams();
    HttpResponse::Ok().json(result)
}

#[get("/{id}")]
async fn get_team(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let result = db.get_team(id);
    match result {
        Some(team) => HttpResponse::Ok().json(team),
        None => HttpResponse::NotFound().body(format!("Could not find team with id: {id}")),
    }
}

#[delete("/{id}")]
async fn delete_team(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let result = db.delete_team(path.into_inner());
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("")]
async fn create_team(db: web::Data<Database>, team: web::Json<NewTeam>) -> HttpResponse {
    // TODO
    // first check if team with the same player ids exists
    // - yes: return Ok(old_team)
    // - no:  create the team

    let result = db.create_team(team.into_inner());
    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
