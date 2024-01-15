use actix_web::web;

pub mod individual;
pub mod server;

pub fn config(cfg: &mut web::ServiceConfig) {
    individual::user_routes(cfg);
    server::server_routes(cfg);
}
