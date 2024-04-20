use dioxus::prelude::*;
use crate::layout::ThemeChooserLayout::ThemeChooserLayout;

#[component]
pub fn Profile() -> Element {
    rsx! {
        ThemeChooserLayout{
            div {
            div {
                class: "flex flex-col gap-4 w-52",
                div {
                    class: "flex gap-4 items-center",
                    div {
                        class: "skeleton w-16 h-16 rounded-full shrink-0"
                    }
                    div {
                        class: "flex flex-col hap-4",
                        div {
                            class: "skeleton h-4 w-20"
                        }
                        div {
                            class: "skeleton h-4 w-28"
                        }
                    }
                }
                div {
                    class: "skeleton h-32 w-full"
                }
            }}
        }
    }
}