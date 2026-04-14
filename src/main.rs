mod api;
mod error;
mod generator;
mod schema;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!("🚢 Strait of Swagger server starting on http://127.0.0.1:3000");

    let app = Router::new()
        .route("/health", get(api::handlers::health_check))
        .route("/api/convert", post(api::handlers::convert_schema))
        .fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
