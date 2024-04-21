use crate::components::ItemInput::ItemInput;
use crate::components::ListChanged::ListChanged;
use crate::components::ShoppingListDisplay::ShoppingListDisplay;
use crate::get_items;
use dioxus::prelude::*;
use model::ShoppingListItem;
use std::collections::HashMap;

#[component]
pub fn ShoppingList(uuid: String) -> Element {
    let uuid_signal = use_signal(|| uuid.clone());
    let change_signal = use_signal(|| ListChanged);

    let future = use_resource(move || async move {
        change_signal.read();
        get_items(&uuid_signal()).await
    });

    match &*future.read_unchecked() {
        Some(Ok(list)) => {
            rsx! {
                div { class: "grid place-items-center min-h-500",
                    h1 { class: "m-16 text-xl text-primary-content font-bold leading-none tracking-tight",
                        "{uuid.clone()}"
                    }
                    ul { class: "menu bg-base-200 w-200 rounded-box gap-1",
                        ShoppingListDisplay{list: list.clone(), uuid: uuid.clone(), change_signal}
                    }
                    ItemInput{
                        list_uuid: uuid.clone(),
                        change_signal
                    }
                }
            }
        }
        Some(Err(_)) => {
            rsx! {
                "Failed fetching data..."
            }
        }
        None => {
            rsx! {
                "Loading Data.."
            }
        }
    }
}
