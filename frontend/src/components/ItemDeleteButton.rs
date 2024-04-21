use std::collections::HashMap;

use dioxus::prelude::*;

use model::ShoppingListItem;

use crate::delete_item;

use super::ListChanged::ListChanged;

#[component]
pub fn ItemDeleteButton(
    list_uuid: String,
    item_uuid: String,
    change_signal: Signal<ListChanged>,
) -> Element {
    let onclick = move |_| {
        spawn({
            let item_uuid = item_uuid.clone();
            let list_uuid = list_uuid.clone();
            async move {
                let response = delete_item(&list_uuid, &item_uuid).await;
                if response.is_ok() {
                    change_signal.write();
                }
            }
        });
    };

    rsx! {
    button {
        onclick: onclick,
        class: "btn btn-circle",
            svg {
                class: "h-6 w-6 text-red",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                fill: "none",
                path {
                    d: "M6 18L18 6M6 6l12 12"
                }
            }
        }
    }
}
