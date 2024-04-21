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
    let items: Vec<ShoppingListItem> = state.read().unwrap().as_vec(uuid.to_string());

    Json(items)
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

    Json(CreateListResponse { uuid })
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
