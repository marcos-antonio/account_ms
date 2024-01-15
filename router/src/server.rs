use actix_web::web;

pub fn server_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/server/health_check",
        web::get().to(handler::server::health_check),
    );
}
