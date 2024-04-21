use crate::components::ItemDeleteButton::ItemDeleteButton;
use dioxus::prelude::*;
use model::ShoppingListItem;
use std::collections::HashMap;

use super::ListChanged::ListChanged;

#[component]
pub fn ListItem(
    display_name: String,
    list_uuid: String,
    item_uuid: String,
    posted_by: String,
    change_signal: Signal<ListChanged>,
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
                change_signal
            }
        }
    }
}
