use std::collections::HashMap;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_resource, use_signal};
use model::ShoppingListItem;
use dioxus::prelude::*;
use crate::components::ItemInput::ItemInput;
use crate::components::ShoppingListDisplay::ShoppingListDisplay;
use crate::get_items;
use crate::layout::ThemeChooserLayout::ThemeChooserLayout;

#[component]
pub fn ShoppingList(uuid: String) -> Element {
    let mut displayed_data = use_signal(HashMap::<String, ShoppingListItem>::new);
    let uuid_signal = use_signal(|| uuid.clone());

    let future = use_resource(move || async move { get_items(&uuid_signal()).await });

    match &*future.read_unchecked() {
        Some(Ok(list)) => {
            for i in list {
                displayed_data.write().insert(i.uuid.clone(), i.clone());
            }
        }
        _ => {}
    }

    rsx! {
        ThemeChooserLayout{
            div { class: "grid place-items-center min-h-500",
                h1 { class: "m-16 text-xl font-bold leading-none tracking-tight",
                    "{uuid.clone()}"
                }
                ul { class: "menu bg-base-200 w-200 rounded-box gap-1",
                    ShoppingListDisplay{list: displayed_data, uuid: uuid.clone()}
                }
                ItemInput{
                    list_uuid: uuid.clone(),
                    current_items: displayed_data
                }
            }
        }
    }
}
