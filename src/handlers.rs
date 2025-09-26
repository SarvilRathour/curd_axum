use axum::{extract::State,Json};
use serde_json::{json,Value};
use crate::state::AppState;
#[tokio::main]
pub async fn something_check(State(state):State<AppState>)->Json<Value>{
  match sqlx::query("SELECT 1").execute(&state.db).await{
    Ok(_)=>Json(json!({
      "status":"ok",
      "database":"connected"
    })),
    Err(e)=>{
    eprintln!("database error {}",e);
    Json(json!({
      "status":"error",
      "database":"disconnected",
      "error":e.to_string()
    }))
    }
  }
}
