use crate::handlers;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/players")
                    .service(handlers::players::get_all_players)
                    .service(handlers::players::get_player_by_id)
                    .service(handlers::players::get_all_teams_by_player_id)
                    .service(handlers::players::create_player)
                    .service(handlers::players::delete_player_by_id)
                    .service(handlers::players::update_player),
            )
            .service(
                web::scope("/teams")
                    .service(handlers::teams::get_all_teams)
                    .service(handlers::teams::get_team_by_id)
                    .service(handlers::teams::create_team)
                    .service(handlers::teams::delete_team_by_id),
            ),
    );
}
