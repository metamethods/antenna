use actix_web::{HttpMessage, HttpResponse, Responder, error::ErrorInternalServerError, post, web};
use entity::{game, prelude::*};
use futures::future::join_all;
use reqwest::StatusCode;
use sea_orm::entity::*;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize, Deserialize)]
struct Author {
    id: String,
    username: String,
    display_name: String,
}

#[derive(Deserialize)]
struct Message {
    author: Author,
    content: String,
}

#[derive(Serialize)]
struct PublishedMessageData {
    author: Author,
    game: String,
    content: String,
}

#[derive(Serialize)]
struct PublishUniverseMessageData {
    topic: String,
    message: String,
}

#[post("/message/{topic}")]
async fn publish_message(
    path: web::Path<(String,)>,
    body: web::Json<Message>,
    data: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> actix_web::Result<impl Responder> {
    let extensions = request.extensions();

    let original_game = extensions
        .get::<game::Model>()
        .ok_or_else(|| ErrorInternalServerError("game missing"))?;

    let games = Game::find()
        .all(&data.database)
        .await
        .map_err(|_| ErrorInternalServerError("unable to query database"))?;

    let body = body.into_inner();
    let message_data_serialized = serde_json::to_string(&PublishedMessageData {
        author: body.author,
        game: original_game.name.clone(),
        content: body.content,
    })
    .map_err(|_| ErrorInternalServerError("failed to serialize"))?;
    let publish_universe_message_data_serialized = PublishUniverseMessageData {
        topic: path.into_inner().0,
        message: message_data_serialized,
    };

    let futures: Vec<_> = games
        .iter()
        .map(|game| {
            data.client
                .post(format!(
                    "https://apis.roblox.com/cloud/v2/universes/{}:publishMessage",
                    game.universe_id
                ))
                .header("x-api-key", &game.open_cloud_api_key)
                .header("Content-Type", "application/json")
                .json(&publish_universe_message_data_serialized)
                .send()
        })
        .collect::<_>();

    let error_count = join_all(futures)
        .await
        .iter()
        .filter(|response| {
            if let Ok(response) = response {
                match response.status() {
                    StatusCode::OK => false,
                    _ => false,
                }
            } else {
                true
            }
        })
        .count();

    if error_count > 0 {
        Ok(HttpResponse::MultiStatus().body(format!("failed to send to {error_count} game(s)")))
    } else {
        Ok(HttpResponse::Ok().body("all messages sent successfully"))
    }
}
