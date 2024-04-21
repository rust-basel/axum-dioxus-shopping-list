use crate::components::ListItem::ListItem;
use dioxus::prelude::*;
use model::ShoppingListItem;
use std::collections::HashMap;

use super::ListChanged::ListChanged;

#[component]
pub fn ShoppingListDisplay(
    list: Vec<ShoppingListItem>,
    uuid: String,
    change_signal: Signal<ListChanged>,
) -> Element {
    rsx! {
        {
        list.iter().map(|item| {
            rsx!{
                li {
                    key: "{item.uuid}",
                    ListItem {
                        display_name: item.title.clone(),
                        posted_by: item.posted_by.clone(),
                        list_uuid: uuid.clone(),
                        item_uuid: item.uuid.clone(),
                        change_signal
                    }
                }
            }
        })
        }
    }
}
