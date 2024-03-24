use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use model::{CreateListResponse, PostShopItem, PostShopItemResponse, ShoppingListItem};
use uuid::Uuid;

use crate::{database::ShoppingItem, SharedData};

pub async fn get_items(
    Path(uuid): Path<Uuid>,
    State(state): State<SharedData>,
) -> impl IntoResponse {
    let mut result = example_list();
    let mut items: Vec<ShoppingListItem> = state.read().unwrap().as_vec(uuid.to_string());

    result.append(&mut items);

    Json(result)
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
    Path((list_uuid, item_uuid)): Path<(Uuid, Uuid)>,
    State(state): State<SharedData>,
) -> impl IntoResponse {
    state
        .write()
        .unwrap()
        .delete_item(list_uuid.to_string(), item_uuid.to_string());

    StatusCode::NO_CONTENT
}

pub async fn create_shopping_list(State(state): State<SharedData>) -> impl IntoResponse {
    let uuid = Uuid::new_v4().to_string();
    state.write().unwrap().create_list(uuid.clone());

    Json(CreateListResponse { id: uuid })
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
