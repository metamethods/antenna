use actix_web::{App, HttpServer, middleware::from_fn, web};
use antenna::{AppState, middleware, routes};
use migration::{Migrator, MigratorTrait};
use reqwest::Client;
use sea_orm::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv(); // just silently load it in, if it doesnt work then oh well

    let database_url = dotenvy::var("DATABASE_URL").expect("missing DATABASE_URL");
    let hmac_key = dotenvy::var("HMAC_KEY").expect("missing HMAC_KEY");
    let port: u16 = dotenvy::var("PORT")
        .expect("missing PORT")
        .parse::<u16>()
        .expect("failed to parse PORT value to u16");

    let client = Client::new();
    let database = Database::connect(database_url)
        .await
        .expect("failed to connect to database");

    Migrator::up(&database, None)
        .await
        .expect("failed to migrate");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: client.clone(),
                database: database.clone(),
                hmac_key: hmac_key.clone(),
            }))
            .configure(routes::index::config)
            .service(
                web::scope("/api")
                    .wrap(from_fn(middleware::api_key_auth::api_key_auth_middleware))
                    .configure(routes::publish::config),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
