use crate::handlers::*;

use actix_web::web;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/matches").configure(matches::configuration))
            .service(web::scope("/players").configure(players::configuration))
            .service(web::scope("/teams").configure(teams::configuration)),
    );
}
