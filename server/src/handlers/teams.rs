use crate::models::team::NewTeam;
use crate::repository::{
    database::Database,
    queries::{matches, teams},
};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all_teams(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(teams::load_teams(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create_team(db: web::Data<Database>, data: web::Json<NewTeam>) -> HttpResponse {
    match teams::find_or_insert_team(&mut db.pool.get().unwrap(), &data) {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating team!"),
    }
}

#[get("/{id}")]
async fn get_team_by_id(db: web::Data<Database>, team_id: web::Path<i32>) -> HttpResponse {
    match teams::find_team_by_id(&mut db.pool.get().unwrap(), &team_id) {
        Some(team) => HttpResponse::Ok().json(team),
        None => {
            HttpResponse::NotFound().body(format!("Could not find team with id '{0}'", &team_id))
        }
    }
}

#[delete("/{id}")]
async fn delete_team_by_id(db: web::Data<Database>, team_id: web::Path<i32>) -> HttpResponse {
    match teams::delete_team(&mut db.pool.get().unwrap(), &team_id) {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting team!"),
    }
}

#[get("/{id}/matches")]
async fn get_all_matches_by_team_id(
    db: web::Data<Database>,
    team_id: web::Path<i32>,
) -> HttpResponse {
    match teams::find_team_by_id(&mut db.pool.get().unwrap(), &team_id) {
        Some(_) => HttpResponse::Ok().json(matches::find_matches_by_team_id(
            &mut db.pool.get().unwrap(),
            &team_id,
        )),
        None => {
            HttpResponse::NotFound().body(format!("Could not find team with id '{0}'", &team_id))
        }
    }
}