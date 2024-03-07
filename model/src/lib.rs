use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ShoppingListItem {
    pub title: String,
    pub posted_by: String
}