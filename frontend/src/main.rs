#![allow(non_snake_case)]

mod components;
mod layout;

use std::collections::HashMap;

use dioxus::prelude::*;
use model::{CreateListResponse, PostShopItem, PostShopItemResponse, ShoppingListItem};
use components::{LoadOrCreateList, ShoppingList, Profile};

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(app);
}

fn items_url(list_uuid: &str) -> String {
    format!("{}/{}/items", list_url(), list_uuid)
}

const fn list_url() -> &'static str {
    "http://127.0.0.1:3000/list"
}

fn delete_item_url(list_uuid: &str, item_uuid: &str) -> String {
    format!("{}/{}", items_url(list_uuid), item_uuid)
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    LoadOrCreateList {},
    #[route("/list/:uuid")]
    ShoppingList { uuid: String },
    #[route("/profile")]
    Profile {},
}

fn app() -> Element {
    rsx! {
        Router::<Route>{}
    }
}




async fn delete_item(list_uuid: &str, item_uuid: &str) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(&delete_item_url(list_uuid, item_uuid))
        .send()
        .await?;

    Ok(())
}

async fn post_item(
    list_uuid: &str,
    item: &PostShopItem,
) -> Result<PostShopItemResponse, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(items_url(list_uuid))
        .json(item)
        .send()
        .await?
        .json::<PostShopItemResponse>()
        .await?;

    Ok(response)
}

async fn create_list() -> Result<CreateListResponse, reqwest::Error> {
    let response = reqwest::Client::new()
        .get(list_url())
        .send()
        .await?
        .json::<CreateListResponse>()
        .await?;

    Ok(response)
}

async fn get_items(list_uuid: &str) -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let list = reqwest::get(items_url(list_uuid))
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;

    list
}

#[cfg(test)]
mod tests {
    use crate::delete_item_url;

    #[test]
    fn delete_url_given_both_uuids_then_creates_correct_url() {
        // given
        let uuid_1 = "A";
        let uuid_2 = "B";

        // when
        let url = delete_item_url(uuid_1, uuid_2);

        // then
        assert_eq!(url, "http://127.0.0.1:3000/list/A/items/B");
    }
}
