use statusforge_backend::{router, AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = statusforge_backend::shared::config::Config::from_env()
        .expect("Missing env: SUPABASE_URL, SUPABASE_PUBLISHABLE_KEY");
    let supabase = statusforge_backend::shared::supabase::create_client(&config)
        .expect("Failed to create Supabase client");

    let state = AppState { supabase };
    let app = router(state);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
