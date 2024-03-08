#![allow(non_snake_case)]
use dioxus::prelude::*;
use model::ShoppingListItem;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {

    let backend_data = use_future(cx, (), |_| get_hello_world());

    render! {
        ThemeChooserLayout{
            div {
                class: "grid place-items-center min-h-500",
                h1 { class: "m-16 text-4xl font-bold leading-none tracking-tight",
                    "Hello, shopping list!"
                }
                match backend_data.value() {
                    Some(Ok(items)) => {
                        rsx! {
                                ul { class: "menu bg-base-200 w-56 rounded-box",
                                    for item in items {
                                        li {
                                            a {
                                               item.title.clone()
                                            }
                                        }
                                    }
                                }
                        }
                    }
                    Some(Err(err)) => {
                        rsx! {"An error occured while fetching from backend: {err}"}
                    }
                    None => {
                        rsx! {"Loading world..."}
                    }
                }
            }
        }
    }
}

#[derive(Props)]
struct PureWrapProps<'a> {
    children: Element<'a>
}
fn ThemeChooserLayout<'a>(cx: Scope<'a,PureWrapProps<'a>>) -> Element {
    let mut active_theme = use_state(cx, || "dark");
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
    render! {
        div {
            class: "min-h-screen",
            "data-theme": "{active_theme}",
            div {
                class: "navbar bg-base-100",
                div { class: "flex-none",
                    ul { class: "menu menu-horizontal px-1",
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
                }
            }
            &cx.props.children
        }
    }
}

async fn get_hello_world() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let list = reqwest::get("http://127.0.0.1:3000")
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;
    list
}