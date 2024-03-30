#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use model::{CreateListResponse, PostShopItem, PostShopItemResponse, ShoppingListItem};

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(app);
}

fn items_url(list_uuid: &str) -> String {
    format!("{}/{}/items", list_url(), list_uuid)
}

const fn list_url() -> &'static str {
    "http://127.0.0.1:3000/list"
}

fn delete_item_url(list_uuid: &str, item_uuid: &str) -> String {
    format!("{}/{}", items_url(list_uuid), item_uuid)
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
fn Profile() -> Element {
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

#[component]
fn LoadOrCreateList() -> Element {
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

#[component]
fn ShoppingListDisplay(list: Signal<HashMap<String, ShoppingListItem>>, uuid: String) -> Element {
    rsx! {
        {
        list().iter().map(|(k,v)| {
            rsx!{
                li {
                    key: "{k}",
                    ListItem {
                        display_name: v.title.clone(),
                        posted_by: v.posted_by.clone(),
                        list_uuid: uuid.clone(),
                        item_uuid: k.clone(),
                        current_items: list.clone()
                    }
                }
            }
        })
        }
    }
}

#[component]
fn ShoppingList(uuid: String) -> Element {
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
                    "Hello, shopping list"
                }
                p{ class: "text-xl",
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

fn app() -> Element {
    rsx! {
        Router::<Route>{}
    }
}

#[component]
fn ListItem(
    display_name: String,
    list_uuid: String,
    item_uuid: String,
    posted_by: String,
    current_items: Signal<HashMap<String, ShoppingListItem>>,
) -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow text-2xl",
                "{display_name}"
            }
            span {
                "posted by {posted_by}"
            }
            ItemDeleteButton{
                list_uuid,
                item_uuid,
                current_items
            }
        }
    }
}

#[component]
fn ItemDeleteButton(
    list_uuid: String,
    item_uuid: String,
    mut current_items: Signal<HashMap<String, ShoppingListItem>>,
) -> Element {
    let onclick = move |_| {
        spawn({
            let item_uuid = item_uuid.clone();
            let list_uuid = list_uuid.clone();
            async move {
                let response = delete_item(&list_uuid, &item_uuid).await;
                if response.is_ok() {
                    current_items.write().remove(&item_uuid);
                }
            }
        });
    };

    rsx! {
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
    }
}

#[derive(Props, Clone, PartialEq)]
struct WrapperProps {
    children: Element,
}

#[component]
fn ThemeChooserLayout(props: WrapperProps) -> Element {
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

#[component]
fn ItemInput(
    list_uuid: String,
    mut current_items: Signal<HashMap<String, ShoppingListItem>>,
) -> Element {
    let mut item = use_signal(|| "".to_string());
    let mut author = use_signal(|| "".to_string());

    let onsubmit = move |evt: FormEvent| {
        spawn({
            let list_uuid = list_uuid.clone();
            async move {
                let item_name = evt.data.values()["item_name"]
                    .first()
                    .cloned()
                    .unwrap_or_default();
                let author = evt.data.values()["author"]
                    .first()
                    .cloned()
                    .unwrap_or_default();
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

    rsx! {
        div {
            class: "w-300 m-4 mt-16 rounded",
            form { class: "grid grid-cols-3 gap-2",
                onsubmit: onsubmit,
                div {
                    input {
                        value: "{item}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "next item..",
                        r#type: "text",
                        id: "item_name",
                        name: "item_name",
                        oninput: move |e| item.set(e.data.value().clone())
                    }
                }
                div {
                    input {
                        value: "{author}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "wanted by..",
                        r#type: "text",
                        id: "author",
                        name: "author",
                        oninput: move |e| author.set(e.data.value().clone())
                    }
                }
                button {
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    "Commit"
                }
            }
        }
    }
}

async fn delete_item(list_uuid: &str, item_uuid: &str) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(&delete_item_url(list_uuid, item_uuid))
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

#[cfg(test)]
mod tests {
    use crate::delete_item_url;

    #[test]
    fn delete_url_given_both_uuids_then_creates_correct_url() {
        // given
        let uuid_1 = "A";
        let uuid_2 = "B";

        // when
        let url = delete_item_url(uuid_1, uuid_2);

        // then
        assert_eq!(url, "http://127.0.0.1:3000/list/A/items/B");
    }
}
