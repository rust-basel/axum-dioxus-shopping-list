use std::collections::HashMap;
use dioxus::prelude::*;
use model::ShoppingListItem;
use crate::components::ListItem::ListItem;

#[component]
pub fn ShoppingListDisplay(list: Signal<HashMap<String, ShoppingListItem>>, uuid: String) -> Element {
    rsx! {
        {
        list().iter().map(|(k,v)| {
            rsx!{
                li {
                    key: "{k}",
                    ListItem {
                        display_name: v.title.clone(),
                        posted_by: v.posted_by.clone(),
                        list_uuid: uuid.clone(),
                        item_uuid: k.clone(),
                        current_items: list.clone()
                    }
                }
            }
        })
        }
    }
}
