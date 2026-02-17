pub mod shared;
mod features;

use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};
use supabase::Client;

pub use shared::ApiError;

#[derive(Clone)]
pub struct AppState {
    pub supabase: Client,
}

async fn root() -> Result<Json<Value>, ApiError> {
    if true {
        return Err(ApiError::NotFound);
    }
    Ok(Json(json!({ "message": "Hello, World!" })))
}

async fn health_check(State(state): State<AppState>) -> Json<Value> {
    let supabase_ok = state.supabase.health_check().await.is_ok();
    Json(json!({
        "app": "healthy",
        "supabase": if supabase_ok { "healthy" } else { "unhealthy" },
    }))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/auth", features::auth::routes())
        .nest("/organizations", features::organizations::routes())
        .nest("/projects", features::projects::routes())
        .nest("/monitors", features::monitors::routes())
        .nest("/ingest", features::ingest::routes())
        .with_state(state)
}
