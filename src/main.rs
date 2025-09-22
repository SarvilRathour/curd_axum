use axum::{response::Json,routing::get,Router};
use serde_json::{json,Value};
mod state;
#[tokio::main]
async fn main() {
    let app=Router::new().route("/something",get(something));
    let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
async fn something()->Json<Value>{
    Json(json!({"status":"ok","message":"server is running"}))
}
