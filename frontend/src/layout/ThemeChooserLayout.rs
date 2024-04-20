use dioxus::prelude::*;
use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct WrapperProps {
    children: Element,
}
#[component]
pub fn ThemeChooserLayout(props: WrapperProps) -> Element {
    let mut active_theme = use_signal(|| "dark");
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

    const HOME_TEXT: &str = "Home";
    const PROFILE_TEXT: &str = "Profile";

    rsx! {
        div {
            class: "min-h-screen",
            r#"data-theme"#: "{active_theme}",
            div {
                class: "navbar bg-base-100",
                div {
                    class: "flex-1",
                    button {
                        class: "btn btn-ghost text-xl",
                        Link {
                            to: Route::LoadOrCreateList{}, {HOME_TEXT}
                        }
                    }
                }
                div { class: "flex-none gap-2",
                    ul { class: "menu menu-horizontal px-8",
                        li {
                            details {
                                summary {
                                    "Choose a theme..."
                                }
                                ul { class: "bg-base-100 rounded-t-none",
                                    for theme in themes {
                                        li { a { onclick: move |_| active_theme.set(theme), {theme} } }
                                    }
                                }
                            }
                        }
                    }
                    button {
                        class: "btn",
                        Link {
                            to: Route::Profile{}, {PROFILE_TEXT}
                        }
                    }
                }
            }
            {props.children}
        }
        Outlet::<Route>{}
    }
}
