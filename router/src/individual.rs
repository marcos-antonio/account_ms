use actix_web::web;
use handler::individual::{create_user_handler, get_user_handler};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/individual")
            .route("/", web::get().to(get_user_handler))
            .route("/", web::post().to(create_user_handler)),
    );
}
