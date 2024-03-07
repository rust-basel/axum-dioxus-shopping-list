#![allow(non_snake_case)]
use dioxus::prelude::*;
use model::ShoppingListItem;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {

    let backend_data = use_future(cx, (), |_| get_hello_world());

    match backend_data.value() {
        Some(Ok(items)) => {
            render! {
                ul {
                    for item in items {
                        li {
                           item.title.clone()
                        }
                    }
                }
            }
        }
        Some(Err(err)) => {
            render! {"An error occured while fetching from backend: {err}"}
        }
        None => {
            render! {"Loading world..."}
        }
    }
}

async fn get_hello_world() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let list = reqwest::get("http://127.0.0.1:3000")
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;
    list
}