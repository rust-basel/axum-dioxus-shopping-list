#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let backend_data = use_future(cx, (), |_| get_hello_world());

    match backend_data.value() {
        Some(Ok(text)) => {
            render! {
                div {
                    {text.clone()}
                }
            }
        }
        Some(Err(err)) => {
            render! {"An error occured while fetching from backend: {err}"}
        }
        None => {
            render! {"Loading world..."}
        }
    }
}

async fn get_hello_world() -> Result<String, reqwest::Error> {
    let hello = reqwest::get("http://127.0.0.1:3000").await?.text();
    hello.await
}