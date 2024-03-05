use axum::{response::Html, routing::get, Router};
use axum::response::IntoResponse;
use axum::http::HeaderMap;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap()); // CORS allow-all for localhost development
    (headers, "Hello from the backend")
}