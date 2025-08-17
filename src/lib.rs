use reqwest::Client;
use sea_orm::DatabaseConnection;

pub mod middleware;
pub mod routes;

pub struct AppState {
    pub client: Client,
    pub database: DatabaseConnection,
}
