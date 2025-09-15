use state::AppState;
use tower_http::trace::TraceLayer;

mod api;
mod db;
mod error;
mod middleware;
mod models;
mod pages;
mod repo;
mod state;
mod utils;
mod web;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Starting PyE TIMES server...");

    // Create tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv::dotenv().ok();

    let app_sate = AppState::from_env()
        .await
        .expect("Failing creating the initial AppState");

    let app = api::routes()
        .merge(pages::routes())
        .with_state(app_sate)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
