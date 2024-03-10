#![allow(non_snake_case)]
use dioxus::prelude::*;
use model::{PostShopItem, ShoppingListItem};

fn main() {
    dioxus_web::launch(App);
}

const fn items_url() -> &'static str {
    "http://127.0.0.1:3000/items"
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
                ItemInput{}
            }
        }
    }
}

#[derive(Props)]
struct PureWrapProps<'a> {
    children: Element<'a>,
}
fn ThemeChooserLayout<'a>(cx: Scope<'a, PureWrapProps<'a>>) -> Element {
    let active_theme = use_state(cx, || "dark");
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

fn ItemInput(cx: Scope) -> Element {
    let item = use_state(cx, || "".to_string());
    let author = use_state(cx, || "".to_string());

    let onsubmit = move |evt: FormEvent| {
        cx.spawn(async move {
            let item_name = evt.values["item_name"].first().cloned().unwrap_or_default();
            let author = evt.values["author"].first().cloned().unwrap_or_default();
            let _ = post_item(&PostShopItem {
                title: item_name,
                posted_by: author,
            })
            .await;
        });
    };

    cx.render(rsx! {
        div {
            class: "w-56 m-4 rounded shadow",
            form {
                onsubmit: onsubmit,
                input {
                    value: "{item}",
                    class: "input input-bordered input-primary w-full",
                    placeholder: "next item..",
                    r#type: "text",
                    id: "item_name",
                    name: "item_name",
                    oninput: move |e| item.set(e.value.clone())
                }
                input {
                    value: "{author}",
                    class: "input input-bordered input-primary w-full",
                    placeholder: "wanted by..",
                    r#type: "text",
                    id: "author",
                    name: "author",
                    oninput: move |e| author.set(e.value.clone())
                }
                button {
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    "Commit"
                }
            }
        }
    })
}

async fn post_item(item: &PostShopItem) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post(items_url())
        .json(item)
        .send()
        .await
        .map(|_| Ok(()))?
}

async fn get_hello_world() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let list = reqwest::get(items_url())
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;
    list
}
