use axum::{Router, response::Json, routing::get};
use serde_json::{Value, json};
use std::env;
mod state;
use state::AppState;
mod handlers;
use handlers::something_check;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");
    let app_state = AppState::new_database(&database_url)
        .await
        .expect("failed to connect");
        println!("database connection saved succesfully");
    let app = Router::new().route("/something_check", get(something)).with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn something() -> Json<Value> {
    Json(json!({"status":"ok","message":"server is running"}))
}
