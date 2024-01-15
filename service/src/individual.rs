use actix_web::{web, HttpResponse};
use error::AppResult;
use io_hub::individual::{GetIndividualResponse, SaveIndividualRequest, SaveIndividualResponse};
use state::AppState;

pub async fn save(
    app_state: web::Data<AppState>,
    req: SaveIndividualRequest,
) -> Result<HttpResponse, sqlx::Error> {
    let individual = query::individual::save(&app_state.db, &req.into()).await?;
    let account = query::account::save(&app_state.db, individual.id).await?;
    query::balance::save(&app_state.db, account.id).await?;

    Ok(HttpResponse::Ok().json(SaveIndividualResponse::from(&individual)))
}

pub async fn get_individual(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> AppResult<HttpResponse> {
    let individual_id = params.into_inner();
    let individual = query::individual::get_by_id(&app_state.db, individual_id).await?;
    Ok(HttpResponse::Ok().json(GetIndividualResponse::from(&individual)))
}
