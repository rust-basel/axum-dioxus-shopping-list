use std::collections::HashMap;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use model::ShoppingListItem;
use crate::components::ItemDeleteButton::ItemDeleteButton;

#[component]
pub fn ListItem(
    display_name: String,
    list_uuid: String,
    item_uuid: String,
    posted_by: String,
    current_items: Signal<HashMap<String, ShoppingListItem>>,
) -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow text-2xl",
                "{display_name}"
            }
            span {
                "posted by {posted_by}"
            }
            ItemDeleteButton{
                list_uuid,
                item_uuid,
                current_items
            }
        }
    }
}
