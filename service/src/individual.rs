use actix_web::{web, HttpResponse};
use io_hub::individual::{SaveIndividualRequest, SaveIndividualResponse};
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
