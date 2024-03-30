use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ShoppingListItem {
    pub title: String,
    pub posted_by: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostShopItem {
    pub title: String,
    pub posted_by: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostShopItemResponse {
    pub id: String,
    pub title: String,
    pub posted_by: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateListResponse {
    pub id: String,
}
