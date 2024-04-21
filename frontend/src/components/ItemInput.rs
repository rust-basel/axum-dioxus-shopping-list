use std::collections::HashMap;

use dioxus::prelude::*;

use model::{PostShopItem, ShoppingListItem};

use crate::post_item;

use super::ListChanged::ListChanged;

#[component]
pub fn ItemInput(list_uuid: String, change_signal: Signal<ListChanged>) -> Element {
    let mut item = use_signal(|| "".to_string());
    let mut author = use_signal(|| "".to_string());

    let onsubmit = move |_| {
        spawn({
            let list_uuid = list_uuid.clone();
            async move {
                let item_name = item.read();
                let author = author.read();
                let response = post_item(
                    &list_uuid,
                    &PostShopItem {
                        title: item_name.to_string(),
                        posted_by: author.to_string(),
                    },
                )
                .await;

                if let Ok(response) = response {
                    change_signal.write();
                }
            }
        });
    };

    rsx! {
        div {
            class: "w-300 m-4 mt-16 rounded",
            form { class: "grid grid-cols-1 md:grid-cols-3 gap-2",
                onsubmit: onsubmit,
                div {
                    input {
                        value: "{item}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "next item..",
                        r#type: "text",
                        id: "item_name",
                        name: "item_name",
                        oninput: move |e| item.set(e.data.value().clone())
                    }
                }
                div {
                    input {
                        value: "{author}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "wanted by..",
                        r#type: "text",
                        id: "author",
                        name: "author",
                        oninput: move |e| author.set(e.data.value().clone())
                    }
                }
                button {
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    "Commit"
                }
            }
        }
    }
}
