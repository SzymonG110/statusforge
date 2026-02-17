mod shared;

use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};
use shared::ApiError;
use supabase::Client;

#[derive(Clone)]
struct AppState {
    supabase: Client,
}

async fn root() -> Result<Json<Value>, ApiError> {
    if true {
        return Err(ApiError::NotFound);
    }

    Ok(Json(json!({
        "message": "Hello, World!",
    })))
}

async fn health_check(State(state): State<AppState>) -> Json<Value> {
    let supabase_ok = state.supabase.health_check().await.unwrap_or(false);
    Json(json!({
        "status": if supabase_ok { "healthy" } else { "degraded" },
        "message": if supabase_ok { "OK" } else { "Supabase unreachable" }
    }))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = shared::config::Config::from_env().expect("Missing env: SUPABASE_URL, SUPABASE_PUBLISHABLE_KEY");
    let supabase = shared::supabase::create_client(&config).expect("Failed to create Supabase client");

    let state = AppState { supabase };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
