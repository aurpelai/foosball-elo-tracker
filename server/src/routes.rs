use crate::handlers;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/matches")
                    .service(handlers::matches::create_match)
                    .service(handlers::matches::delete_match_by_id)
                    .service(handlers::matches::get_all_matches)
                    .service(handlers::matches::get_all_matches_between_teams)
                    .service(handlers::matches::get_match_by_id),
            )
            .service(
                web::scope("/players")
                    .service(handlers::players::create_player)
                    .service(handlers::players::delete_player_by_id)
                    .service(handlers::players::get_all_players)
                    .service(handlers::players::get_all_teams_by_player_id)
                    .service(handlers::players::get_player_by_id)
                    .service(handlers::players::update_player),
            )
            .service(
                web::scope("/teams")
                    .service(handlers::teams::create_team)
                    .service(handlers::teams::delete_team_by_id)
                    .service(handlers::teams::get_all_matches_by_team_id)
                    .service(handlers::teams::get_all_teams)
                    .service(handlers::teams::get_team_by_id),
            ),
    );
}
