use hmac::Hmac;
use reqwest::Client;
use sea_orm::DatabaseConnection;
use sha2::Sha256;

pub mod middleware;
pub mod routes;

pub type HmacSha256 = Hmac<Sha256>;

pub struct AppState {
    pub client: Client,
    pub database: DatabaseConnection,
    pub hmac_key: String,
}
