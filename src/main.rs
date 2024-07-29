use shuttle_axum::ShuttleAxum;
use sqlx::PgPool;

pub mod controllers;
mod db;
mod middleware;
mod models;
mod routing;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub api_key: String,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> ShuttleAxum {
    db::run_migrations(&pool).await;
    let api_key = secrets.get("API_KEY").expect("secret not found");
    let state = AppState { pool, api_key };
    Ok(routing::routing(state).await)
}
