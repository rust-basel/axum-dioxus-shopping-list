use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::events::FormEvent;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use crate::{create_list, Route};
use crate::layout::ThemeChooserLayout::ThemeChooserLayout;

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
        ThemeChooserLayout{
            div{
                class: "grid place-items-center min-h-500",
                div{
                    class: "flex justify-content",
                    button{
                        class: "btn m-4",
                        onclick: on_create_list_click,
                        "Create new List"
                    }
                    form {
                        onsubmit: onloadsubmit,
                        div {
                            class: "flex flex-col",
                            button{
                                class: "btn m-4",
                                r#type: "submit",
                                "Load existing List"
                            }
                            input{
                                class:"input input-bordered",
                                r#type:"text",
                                placeholder:"Type here the uuid",
                                id: "uuid",
                                name: "uuid",
                                oninput: move |e| uuid.set(e.data.value())
                            }
                        }
                    }
                }
            }
        }
    }
}
