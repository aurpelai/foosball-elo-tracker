use crate::handlers;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/matches")
                    .service(handlers::matches::create)
                    .service(handlers::matches::delete)
                    .service(handlers::matches::get)
                    .service(handlers::matches::get_all)
                    .service(handlers::matches::get_by_rivalry),
            )
            .service(
                web::scope("/players")
                    .service(handlers::players::create)
                    .service(handlers::players::delete)
                    .service(handlers::players::get)
                    .service(handlers::players::get_all)
                    .service(handlers::players::get_by_match_id)
                    .service(handlers::players::get_teams)
                    .service(handlers::players::update),
            )
            .service(
                web::scope("/player_matches")
                    .service(handlers::player_matches::create)
                    .service(handlers::player_matches::delete)
                    .service(handlers::player_matches::get)
                    .service(handlers::player_matches::get_all)
                    .service(handlers::player_matches::get_by_player_id),
            )
            .service(
                web::scope("/teams")
                    .service(handlers::teams::create)
                    .service(handlers::teams::delete)
                    .service(handlers::teams::get)
                    .service(handlers::teams::get_all)
                    .service(handlers::teams::get_by_match_id)
                    .service(handlers::teams::get_matches),
            )
            .service(
                web::scope("/team_matches")
                    .service(handlers::team_matches::create)
                    .service(handlers::team_matches::delete)
                    .service(handlers::team_matches::get)
                    .service(handlers::team_matches::get_all)
                    .service(handlers::team_matches::get_by_team_id),
            ),
    );
}
