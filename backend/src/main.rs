use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use model::{PostShopItem, PostShopItemResponse, ShoppingListItem};

#[tokio::main]
async fn main() {
    let db = SharedData::default();
    let app = Router::new()
        .route("/", get(handler))
        .route("/items", post(create_shopping_item))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap()); // CORS allow-all for localhost development

    let result = example_list();
    let serialized = serde_json::to_string(&result).unwrap();

    (headers, serialized)
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
