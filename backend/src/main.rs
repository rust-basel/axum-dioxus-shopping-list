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

async fn get_items(Path(uuid): Path<Uuid>, State(state): State<SharedData>) -> impl IntoResponse {
    let mut result = example_list();
    let mut items: Vec<ShoppingListItem> = state.read().unwrap().as_vec(uuid.to_string());

    result.append(&mut items);

    Json(result)
}

type SharedData = Arc<RwLock<InMemoryDatabase>>;

#[derive(Default)]
pub struct ShoppingList {
    list: HashMap<String, ShoppingItem>,
}

pub struct InMemoryDatabase {
    db: HashMap<String, ShoppingList>,
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        let mut db = HashMap::new();
        db.insert(
            "9e137e61-08ac-469d-be9d-6b3324dd20ad".to_string(),
            ShoppingList::default(),
        );
        InMemoryDatabase { db }
    }
}

impl InMemoryDatabase {
    fn insert_item(&mut self, list_uuid: String, item_uuid: String, shopping_item: ShoppingItem) {
        self.db
            .get_mut(&list_uuid)
            .and_then(|list| list.list.insert(item_uuid, shopping_item));
    }

    fn get_item(&self, list_uuid: String, item_uuid: String) -> Option<&ShoppingItem> {
        self.db
            .get(&list_uuid)
            .and_then(|list| list.list.get(&item_uuid))
    }

    fn delete_item(&mut self, list_uuid: String, item_uuid: String) {
        self.db
            .get_mut(&list_uuid)
            .and_then(|list| list.list.remove(&item_uuid));
    }

    fn create_list(&mut self, list_uuid: String) {
        self.db.insert(list_uuid, ShoppingList::default());
    }

    fn get_list(&self, list_uuid: String) -> Option<&ShoppingList> {
        self.db.get(&list_uuid)
    }

    fn delete_list(&mut self, list_uuid: String) {
        self.db.remove(&list_uuid);
    }

    fn as_vec(&self, list_uuid: String) -> Vec<ShoppingListItem> {
        let list = self.get_list(list_uuid);
        match list {
            Some(list) => list
                .list
                .iter()
                .map(|(key, item)| ShoppingListItem {
                    title: item.title.clone(),
                    posted_by: item.creator.clone(),
                    uuid: key.clone(),
                })
                .collect(),
            None => Vec::default(),
        }
    }
}

#[derive(Clone)]
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
            uuid: "6a363dc3-b34c-43e3-8ea0-919b25b57c43".to_string(),
        },
        ShoppingListItem {
            title: String::from("Tomato seeds"),
            posted_by: String::from("Tania"),
            uuid: "4e5548aa-4525-45c4-82c9-61d8919f940d".to_string(),
        },
    ]
}

pub async fn delete_shopping_item(
    Path(list_uuid): Path<Uuid>,
    Path(item_uuid): Path<Uuid>,
    State(state): State<SharedData>,
) -> impl IntoResponse {
    state
        .write()
        .unwrap()
        .delete_item(list_uuid.to_string(), item_uuid.to_string());

    StatusCode::NO_CONTENT
}

pub async fn create_shopping_item(
    Path(list_uuid): Path<Uuid>,
    State(state): State<SharedData>,
    Json(request): Json<PostShopItem>,
) -> impl IntoResponse {
    let item = ShoppingItem::from(request.clone());
    let uuid = Uuid::new_v4().to_string();
    state
        .write()
        .unwrap()
        .insert_item(list_uuid.to_string(), uuid.to_string(), item);
    (
        StatusCode::CREATED,
        Json(PostShopItemResponse {
            id: uuid,
            posted_by: request.posted_by,
            title: request.title,
        }),
    )
}
