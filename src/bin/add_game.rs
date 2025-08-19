use std::env;

use antenna::HmacSha256;
use entity::{api_key, game, prelude::*};
use hmac::Mac;
use migration::{Migrator, MigratorTrait};
use rand::distr::{Alphanumeric, SampleString};
use sea_orm::{ActiveValue, Database, EntityTrait};

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();

    let game_name = args
        .get(1)
        .expect("missing game name argument at position 1");
    let game_universe_id = args
        .get(2)
        .expect("missing game universe id argument at position 2");
    let game_opencloud_api_key = args
        .get(3)
        .expect("missing game opencloud api key argument at position 3");

    dotenvy::dotenv().expect("failed to load .env file");

    let database_url = dotenvy::var("DATABASE_URL").expect("missing DATABASE_URL");
    let hmac_key = dotenvy::var("hmac_key").expect("missing hmac_key");

    let database = Database::connect(database_url)
        .await
        .expect("failed to connect to database");

    Migrator::up(&database, None)
        .await
        .expect("failed to migrate");

    let api_key = Alphanumeric.sample_string(&mut rand::rng(), 256);

    let mut hmac =
        HmacSha256::new_from_slice(hmac_key.as_bytes()).expect("unable to create new HmacSha256");

    hmac.update(api_key.as_bytes());

    let hmac_result = hmac.finalize();
    let key_hash = hex::encode(hmac_result.into_bytes());

    let game_model = Game::insert(game::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(game_name.clone()),
        universe_id: ActiveValue::Set(game_universe_id.clone()),
        open_cloud_api_key: ActiveValue::Set(game_opencloud_api_key.clone()),
    })
    .exec_with_returning(&database)
    .await
    .expect("failed to insert game model to database");

    ApiKey::insert(api_key::ActiveModel {
        id: ActiveValue::NotSet,
        key_hash: ActiveValue::Set(key_hash),
        game_id: ActiveValue::Set(game_model.id),
    })
    .exec(&database)
    .await
    .expect("failed to add api key to database");

    println!("successfuly added {game_name} to the database with the api key of\n{api_key}");
}
