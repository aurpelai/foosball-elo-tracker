use crate::models::r#match::{NewMatch, Rivalry};
use crate::repository::{
    database::Database,
    queries::{matches, teams},
};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("")]
async fn get_all_matches(db: web::Data<Database>) -> HttpResponse {
    HttpResponse::Ok().json(matches::load_matches(&mut db.pool.get().unwrap()))
}

#[post("")]
async fn create_match(db: web::Data<Database>, data: web::Json<NewMatch>) -> HttpResponse {
    match matches::insert_match(&mut db.pool.get().unwrap(), &data) {
        Ok(r#match) => HttpResponse::Ok().json(r#match),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating match!"),
    }
}

#[get("/{id}")]
async fn get_match_by_id(db: web::Data<Database>, match_id: web::Path<i32>) -> HttpResponse {
    match matches::find_match_by_id(&mut db.pool.get().unwrap(), &match_id) {
        Some(r#match) => HttpResponse::Ok().json(r#match),
        None => {
            HttpResponse::NotFound().body(format!("Could not find match with id '{0}'", &match_id))
        }
    }
}

#[get("/rivalry")]
async fn get_all_matches_between_teams(
    db: web::Data<Database>,
    data: web::Json<Rivalry>,
) -> HttpResponse {
    let connection = &mut db.pool.get().unwrap();

    match teams::find_team_by_id(connection, &data.team_one_id) {
        Some(_) => match teams::find_team_by_id(connection, &data.team_two_id) {
            Some(_) => HttpResponse::Ok().json(matches::find_matches_between_teams(
                connection,
                &data.team_one_id,
                &data.team_two_id,
            )),
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
async fn delete_match_by_id(db: web::Data<Database>, match_id: web::Path<i32>) -> HttpResponse {
    match matches::delete_match(&mut db.pool.get().unwrap(), &match_id) {
        Ok(r#match) => HttpResponse::Ok().json(r#match),
        Err(_) => HttpResponse::InternalServerError().body("Error while deleting match!"),
    }
}
