use dioxus::prelude::*;
use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct WrapperProps {
    children: Element,
}
#[component]
pub fn ThemeChooserLayout(props: WrapperProps) -> Element {
    let mut active_theme = use_signal(|| "sunset");
    let themes = vec![
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
    ];

    const HOME_TEXT: &str = "My Lists";
    const PROFILE_TEXT: &str = "Profile";

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-r from-primary to-accent",
            r#"data-theme"#: "{active_theme}",
            div {
                class: "navbar glass flex",
                div {
                    class: "flex-1 flex flex-row gap-4 p-4",
                    Link { class: "text-primary-content hover:text-secondary",
                        to: Route::LoadOrCreateList{}, {HOME_TEXT}
                    }
                    Link { class: "text-primary-content hover:text-secondary",
                        to: Route::Profile{}, {PROFILE_TEXT}
                    }
                }
                div { class: "flex-none gap-2",
                    ul { class: "menu menu-horizontal px-8 text-sm z-10",
                        li {
                            details {
                                summary {
                                    "Choose a theme..."
                                }
                                ul { class: "bg-base-100 rounded-t-none",
                                    for theme in themes {
                                        li { a { onclick: move |evt| active_theme.set(theme), {theme} } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "container mx-auto max-w-[1024px] p-8",
                Outlet::<Route>{}
                {props.children}
            }
        }
    }
}
