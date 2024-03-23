#![allow(non_snake_case)]
use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use model::{CreateListResponse, PostShopItem, PostShopItemResponse, ShoppingListItem};

fn main() {
    dioxus_web::launch(App);
}

fn items_url(list_uuid: &str) -> String {
    format!("{}/{}/items", list_url(), list_uuid)
}

const fn list_url() -> &'static str {
    "http://127.0.0.1:3000/list"
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    LoadOrCreateList {},
    #[route("/list/:uuid")]
    ShoppingList { uuid: String },
    #[route("/profile")]
    Profile {},
}

#[component]
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
#[component]
fn LoadOrCreateList(cx: Scope) -> Element {
    let uuid = use_state(cx, || "9e137e61-08ac-469d-be9d-6b3324dd20ad".to_string());
    let nav = use_navigator(cx);
    let onloadsubmit = move |evt: FormEvent| {
        cx.spawn({
            let nav = nav.clone();
            async move {
                let uuid_value = evt.values["uuid"].first().cloned().unwrap_or_default();
                if !uuid_value.is_empty() {
                    nav.push(Route::ShoppingList { uuid: uuid_value });
                }
            }
        });
    };

    let on_create_list_click = move |_| {
        let nav = nav.clone();
        cx.spawn({
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

    cx.render(rsx! {
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
                                oninput: move |e| uuid.set(e.value.clone())
                            }
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn ShoppingList(cx: Scope, uuid: String) -> Element {
    let displayed_data = use_ref(cx, || HashMap::<String, ShoppingListItem>::new());

    use_effect(cx, (), |_| {
        let items = displayed_data.clone();
        let uuid = uuid.clone();
        async move {
            let fetched_items = get_items(&uuid).await;
            if let Ok(fetched_items) = fetched_items {
                for i in fetched_items {
                    items.write().insert(i.uuid.clone(), i.clone());
                }
            }
        }
    });

    render! {
        ThemeChooserLayout{
            div {
                class: "grid place-items-center min-h-500",
                h1 { class: "m-16 text-xl font-bold leading-none tracking-tight",
                    "Hello, shopping list"
                }
                p{
                    class: "text-xl",
                    "{uuid.clone()}"
                }
                rsx!{
                    ul {
                        class: "menu bg-base-200 w-56 rounded-box",
                        displayed_data.read().iter().map(|(k,v)| {
                            rsx!{
                                li {
                                    key: "{k}",
                                    ListItem {
                                        display_name: v.title.clone(),
                                        list_uuid: uuid.clone(),
                                        item_uuid: k.clone(),
                                        current_items: displayed_data
                                    }
                                }
                            }
                        })
                    }
                }
                ItemInput{
                    list_uuid: uuid.clone(),
                    current_items: displayed_data
                }
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
struct ItemProps<'a> {
    display_name: String,
    list_uuid: String,
    item_uuid: String,
    current_items: &'a UseRef<HashMap<String, ShoppingListItem>>,
}

fn ListItem<'a>(cx: Scope<'a, ItemProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow",
                "{cx.props.display_name}"
            }
            ItemDeleteButton{
                list_uuid: cx.props.list_uuid.to_string(),
                item_uuid: cx.props.item_uuid.to_string(),
                current_items: cx.props.current_items
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct ItemDeleteButtonProps<'a> {
    list_uuid: String,
    item_uuid: String,
    current_items: &'a UseRef<HashMap<String, ShoppingListItem>>,
}

fn ItemDeleteButton<'a>(cx: Scope<'a, ItemDeleteButtonProps<'a>>) -> Element {
    let onclick = move |_| {
        cx.spawn({
            let item_uuid = cx.props.item_uuid.clone();
            let list_uuid = cx.props.list_uuid.clone();
            let current_items = cx.props.current_items.clone();
            async move {
                let response = delete_item(&list_uuid, &item_uuid).await;
                if response.is_ok() {
                    current_items.write().remove(&item_uuid);
                }
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
            &cx.props.children
        }
        Outlet::<Route>{}
    }
}

#[derive(Props, PartialEq)]
struct ItemInputProps<'a> {
    list_uuid: String,
    current_items: &'a UseRef<HashMap<String, ShoppingListItem>>,
}

fn ItemInput<'a>(cx: Scope<'a, ItemInputProps<'a>>) -> Element {
    let item = use_state(cx, || "".to_string());
    let author = use_state(cx, || "".to_string());

    let onsubmit = move |evt: FormEvent| {
        cx.spawn({
            let list_uuid = cx.props.list_uuid.clone();
            let current_items = cx.props.current_items.clone();
            async move {
                let item_name = evt.values["item_name"].first().cloned().unwrap_or_default();
                let author = evt.values["author"].first().cloned().unwrap_or_default();
                let response = post_item(
                    &list_uuid,
                    &PostShopItem {
                        title: item_name,
                        posted_by: author,
                    },
                )
                .await;

                if let Ok(response) = response {
                    current_items.write().insert(
                        response.id.to_string(),
                        ShoppingListItem {
                            title: response.title,
                            posted_by: response.posted_by,
                            uuid: response.id,
                        },
                    );
                }
            }
        });
    };

    cx.render(rsx! {
        div {
            class: "w-56 m-4 rounded",
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

async fn delete_item(list_uuid: &str, item_uuid: &str) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(&format!("{}/{}", items_url(list_uuid), item_uuid))
        .send()
        .await?;

    Ok(())
}

async fn post_item(
    list_uuid: &str,
    item: &PostShopItem,
) -> Result<PostShopItemResponse, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(items_url(list_uuid))
        .json(item)
        .send()
        .await?
        .json::<PostShopItemResponse>()
        .await?;

    Ok(response)
}

async fn create_list() -> Result<CreateListResponse, reqwest::Error> {
    let response = reqwest::Client::new()
        .get(list_url())
        .send()
        .await?
        .json::<CreateListResponse>()
        .await?;

    Ok(response)
}

async fn get_items(list_uuid: &str) -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let list = reqwest::get(items_url(list_uuid))
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;

    list
}
