use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    middleware::Next,
    web,
};
use entity::{api_key, prelude::*};
use sea_orm::{entity::*, query::*};

use crate::AppState;

pub async fn api_key_auth_middleware(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let data = request
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorInternalServerError("app state is missing"))?
        .clone();

    let header_api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|k| k.to_str().ok())
        .map(|k| k.to_string())
        .ok_or_else(|| ErrorBadRequest("missing x-api-key"))?;

    let (_, game_model) = ApiKey::find()
        .find_also_related(Game)
        .filter(api_key::Column::Key.eq(header_api_key))
        .one(&data.database)
        .await
        .map_err(|_| ErrorInternalServerError("unable to reach database"))?
        .ok_or_else(|| ErrorUnauthorized("invalid api key"))?;

    let game_model = game_model
        .ok_or_else(|| ErrorInternalServerError("api key does not have a game linked to it"))?;

    request.extensions_mut().insert(game_model);

    next.call(request).await
}
