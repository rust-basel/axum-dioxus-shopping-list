use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use axum::response::IntoResponse;
use axum::{routing::delete, routing::get, Router};
use model::{PostShopItem, PostShopItemResponse, ShoppingListItem};

#[tokio::main]
async fn main() {
    let db = SharedData::default();
    let app = Router::new()
        .route("/items", get(handler).post(create_shopping_item))
        .route("/items/:uuid", delete(delete_shopping_item))
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<SharedData>) -> impl IntoResponse {
    let mut result = example_list();
    let mut other_items: Vec<ShoppingListItem> = state
        .read()
        .unwrap()
        .db
        .iter()
        .map(|(_, shop_item)| ShoppingListItem {
            title: shop_item.title.clone(),
            posted_by: shop_item.creator.clone(),
        })
        .collect();
    result.append(&mut other_items);

    Json(result)
}

type SharedData = Arc<RwLock<InMemoryDatabase>>;

#[derive(Default)]
pub struct InMemoryDatabase {
    db: HashMap<String, ShoppingItem>,
}

struct ShoppingItem {
    title: String,
    creator: String,
}

impl From<PostShopItem> for ShoppingItem {
    fn from(value: PostShopItem) -> Self {
        Self {
            title: value.title,
            creator: value.posted_by,
        }
    }
}

fn example_list() -> Vec<ShoppingListItem> {
    vec![
        ShoppingListItem {
            title: String::from("Coffee"),
            posted_by: String::from("Roland"),
        },
        ShoppingListItem {
            title: String::from("Tomato seeds"),
            posted_by: String::from("Tania"),
        },
    ]
}

pub async fn delete_shopping_item(
    Path(uuid): Path<Uuid>,
    State(state): State<SharedData>,
) -> impl IntoResponse {
    state.write().unwrap().db.remove(&uuid.to_string());

    StatusCode::NO_CONTENT
}

pub async fn create_shopping_item(
    State(state): State<SharedData>,
    Json(request): Json<PostShopItem>,
) -> impl IntoResponse {
    let item = ShoppingItem::from(request.clone());
    let uuid = Uuid::new_v4().to_string();
    state.write().unwrap().db.insert(uuid.clone(), item);

    (
        StatusCode::CREATED,
        Json(PostShopItemResponse {
            id: uuid,
            posted_by: request.posted_by,
            title: request.title,
        }),
    )
}
