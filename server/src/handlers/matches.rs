use crate::models::matches::{NewMatch, Rivalry};
use crate::repository::{
    database::Database,
    queries::{matches, player_matches, team_matches, teams},
};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(matches::load_all(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create(db: web::Data<Database>, data: web::Json<NewMatch>) -> HttpResponse {
    match matches::insert(&mut db.pool.get().unwrap(), &data) {
        Ok(match_value) => {
            match team_matches::insert_from_match(&mut db.pool.get().unwrap(), &match_value) {
                Ok(_) => HttpResponse::Ok().json(&match_value),
                Err(_) => {
                    HttpResponse::InternalServerError().body("Error while creating team match!")
                }
            };
            match player_matches::insert_from_match(&mut db.pool.get().unwrap(), &match_value) {
                Ok(_) => HttpResponse::Ok().json(&match_value),
                Err(_) => {
                    HttpResponse::InternalServerError().body("Error while creating player match!")
                }
            };
            HttpResponse::Ok().json(&match_value)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error while creating match!"),
    }
}

#[get("/{id}")]
async fn get(db: web::Data<Database>, match_id: web::Path<i32>) -> HttpResponse {
    match matches::find_by_id(&mut db.pool.get().unwrap(), &match_id) {
        Some(value) => HttpResponse::Ok().json(value),
        None => {
            HttpResponse::NotFound().body(format!("Could not find match with id '{0}'", &match_id))
        }
    }
}

#[get("/rivalry")]
async fn get_by_rivalry(db: web::Data<Database>, data: web::Json<Rivalry>) -> HttpResponse {
    let connection = &mut db.pool.get().unwrap();

    match teams::find_by_id(connection, &data.team_one_id) {
        Some(_) => match teams::find_by_id(connection, &data.team_two_id) {
            Some(_) => HttpResponse::Ok().json(matches::find_by_rivalry(connection, &data)),
            None => HttpResponse::NotFound().body(format!(
                "Could not find team with id '{0}'",
                &data.team_two_id
            )),
        },
        None => HttpResponse::NotFound().body(format!(
            "Could not find team with id '{0}'",
            &data.team_one_id
        )),
    }
}

#[delete("/{id}")]
async fn delete(db: web::Data<Database>, match_id: web::Path<i32>) -> HttpResponse {
    match matches::delete(&mut db.pool.get().unwrap(), &match_id) {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting match!"),
    }
}
