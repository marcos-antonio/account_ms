use actix_web::{web, HttpResponse};
use error::AppResult;
use io_hub::individual::SaveIndividualRequest;
use state::AppState;

pub async fn get_user_handler(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> AppResult<HttpResponse> {
    Ok(service::individual::get_individual(app_state, path).await?)
}

pub async fn create_user_handler(
    app_state: web::Data<AppState>,
    web::Json(req): web::Json<SaveIndividualRequest>,
) -> AppResult<HttpResponse> {
    Ok(service::individual::save(app_state, req).await?)
}
