use dioxus::prelude::*;
use crate::{create_list, Route};

#[component]
pub fn LoadOrCreateList() -> Element {
    let mut uuid = use_signal(|| "9e137e61-08ac-469d-be9d-6b3324dd20ad".to_string());
    let nav = use_navigator();
    let onloadsubmit = move |evt: FormEvent| {
        spawn({
            async move {
                let uuid_value = evt.data.values()["uuid"]
                    .first()
                    .cloned()
                    .unwrap_or_default();
                if !uuid_value.is_empty() {
                    nav.push(Route::ShoppingList { uuid: uuid_value });
                }
            }
        });
    };

    let on_create_list_click = move |_| {
        let nav = nav.clone();
        spawn({
            async move {
                let response = create_list().await;
                if let Ok(created_list) = response {
                    nav.push(Route::ShoppingList {
                        uuid: created_list.id,
                    });
                }
            }
        });
    };

    rsx! {
        div{
            class: "grid place-content-evently grid-cols-1 md:grid-cols-2 w-full gap-4",
            div {
                class: "card glass min-h-500 flex flex-col content-end gap-4 p-4",
                button{
                    class: "btn btn-primary",
                    onclick: on_create_list_click,
                    "Create new List"
                }
            }
            div { class: "card glass min-h-500",
                form {
                    onsubmit: onloadsubmit,
                    div {
                        class: "flex flex-col gap-4 p-4",
                        input{
                            class:"input input-bordered",
                            r#type:"text",
                            placeholder:"Enter UUID here...",
                            id: "uuid",
                            name: "uuid",
                            oninput: move |e| uuid.set(e.data.value())
                        }
                        button{
                            class: "btn btn-primary",
                            r#type: "submit",
                            "Load existing List"
                        }
                    }
                }
            }
        }
    }
}
