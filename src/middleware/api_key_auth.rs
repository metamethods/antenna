use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    middleware::Next,
    web,
};
use entity::{api_key, prelude::*};
use hmac::Mac;
use sea_orm::{entity::*, query::*};

use crate::{AppState, HmacSha256};

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

    let mut hmac = HmacSha256::new_from_slice(data.hmac_key.as_bytes())
        .map_err(|_| ErrorInternalServerError("failed to create hash object"))?;

    hmac.update(header_api_key.as_bytes());

    let hmac_result = hmac.finalize();
    let key_hash_hex = hex::encode(hmac_result.into_bytes());

    let game_model = ApiKey::find()
        .find_also_related(Game)
        .filter(api_key::Column::KeyHash.eq(key_hash_hex))
        .one(&data.database)
        .await
        .map_err(|_| ErrorInternalServerError("unable to reach database"))?
        .ok_or_else(|| ErrorUnauthorized("invalid api key"))?
        .1
        .ok_or_else(|| ErrorInternalServerError("api key does not have a game linked to it"))?;

    request.extensions_mut().insert(game_model);

    next.call(request).await
}
