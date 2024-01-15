use actix_web::{web, HttpRequest, HttpResponse, Responder};
use io_hub::individual::SaveIndividualRequest;
use state::AppState;

pub async fn get_user_handler(_req: HttpRequest) -> impl Responder {
    "get_user_handler is alive and well"
}

pub async fn create_user_handler(
    app_state: web::Data<AppState>,
    web::Json(req): web::Json<SaveIndividualRequest>,
) -> impl Responder {
    service::individual::save(app_state, req)
        .await
        .unwrap_or_else(|err| HttpResponse::InternalServerError().json(format!("{:?}", err)))
}
