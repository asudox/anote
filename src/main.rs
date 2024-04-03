mod algorithm;
mod db;
mod routes;
mod schema;
mod structs;
mod templates;
mod training;
mod utils;

use axum::{routing::get, Router};
use std::str::FromStr;
use structs::AppState;
use tower_http::services::ServeDir;
use tracing::Level;

#[tokio::main]
async fn main() {
    let config = utils::get_config();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&config.anote.TRACING_MAXIMUM_LEVEL).unwrap())
        .compact()
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("Tracing set up");

    let db_pool = db::create_pool().await;
    tracing::info!("Database pool created");

    if config.training.BEGIN_TRAINING {
        tracing::info!("Training starting");
        training::begin_training(db_pool).await;
        tracing::info!("Training finished. Exiting");
        return;
    }

    let state = AppState { db_pool };
    let static_files_service = ServeDir::new("static/");
    let app = Router::new()
        .route("/", get(routes::root))
        .route(
            "/get_recommendations",
            get(routes::get_user_recommendations),
        )
        // .route("/recommendations", get(routes::root))
        .route_service("/:filename", static_files_service)
        .with_state(state);
    tracing::info!("Router set up");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
