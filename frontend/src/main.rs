#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use model::{PostShopItem, ShoppingListItem};

fn main() {
    dioxus_web::launch(App);
}

const fn items_url() -> &'static str {
    "http://127.0.0.1:3000/items"
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/profile")]
    Profile {},
}

fn Home(cx: Scope) -> Element {
    let backend_data = use_future(cx, (), |_| get_items());

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
                                            ListItem{
                                                display_name: item.title.clone(),
                                                uuid: item.uuid.clone()
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

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route>{}
    }
}

#[derive(PartialEq, Props)]
struct ItemProps {
    display_name: String,
    uuid: String,
}

fn ListItem(cx: Scope<ItemProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow",
                "{cx.props.display_name}"
            }
            ItemDeleteButton{
                uuid: cx.props.uuid.clone()
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct ItemDeleteButtonProps {
    uuid: String,
}

fn ItemDeleteButton(cx: Scope<ItemDeleteButtonProps>) -> Element {
    let onclick = move |_| {
        cx.spawn({
            let uuid = cx.props.uuid.clone();
            async move {
                let _ = delete_item(uuid).await;
            }
        });
    };

    cx.render(rsx! {
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
    })
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

    const HOME_TEXT: &str = "Home";
    const PROFILE_TEXT: &str = "Profile";
    render! {
        div {
            class: "min-h-screen",
            "data-theme": "{active_theme}",
            div {
                class: "navbar bg-base-100",
                div {
                    class: "flex-1",
                    button {
                        class: "btn btn-ghost text-xl",
                        Link {
                            to: Route::Home{}, {HOME_TEXT}
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
            &cx.props.children
        }
        Outlet::<Route>{}
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

fn Profile(cx: Scope) -> Element {
    render! {
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

async fn delete_item(item_uuid: String) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(&format!("{}/{}", items_url(), item_uuid))
        .send()
        .await?;

    Ok(())
}

async fn post_item(item: &PostShopItem) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .post(items_url())
        .json(item)
        .send()
        .await?;

    Ok(())
}

async fn get_items() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let list = reqwest::get(items_url())
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;

    list
}
