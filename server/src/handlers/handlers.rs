use crate::{
    models::player::{NewPlayer, Player},
    repository::database::Database,
};
use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("/players")]
async fn get_players(db: web::Data<Database>) -> HttpResponse {
    let players = db.get_players();
    HttpResponse::Ok().json(players)
}

#[get("/players/{id}")]
async fn get_player(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let player = db.get_player(path.into_inner());
    match player {
        Some(player) => HttpResponse::Ok().json(player),
        None => HttpResponse::NotFound().body("Not Found"),
    }
}

#[delete("/players/{id}")]
async fn delete_player(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let player = db.delete_player(path.into_inner());
    match player {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("/players")]
async fn update_player(db: web::Data<Database>, player: web::Json<Player>) -> HttpResponse {
    let player = db.update_player(player.into_inner());
    match player {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("/players")]
async fn create_player(db: web::Data<Database>, player: web::Json<NewPlayer>) -> HttpResponse {
    let player = db.create_player(player.into_inner());
    match player {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_players)
            .service(get_player)
            .service(create_player)
            .service(delete_player)
            .service(update_player),
    );
}
