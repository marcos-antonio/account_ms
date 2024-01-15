use actix_web::HttpResponse;
use error::AppResult;

pub async fn health_check() -> AppResult<HttpResponse> {
    Ok(HttpResponse::Ok().json("server is up!"))
}
