mod controller;
mod database;

use database::InMemoryDatabase;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;

use axum::{routing::delete, routing::get, Router};

use crate::controller::{
    create_shopping_item, create_shopping_list, delete_shopping_item, get_items,
};

type SharedData = Arc<RwLock<InMemoryDatabase>>;

#[tokio::main]
async fn main() {
    let db = SharedData::default();
    let app = Router::new()
        .route("/list", get(create_shopping_list))
        .route(
            "/list/:uuid/items",
            get(get_items).post(create_shopping_item),
        )
        .route(
            "/list/:list_uuid/items/:item_uuid",
            delete(delete_shopping_item),
        )
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
