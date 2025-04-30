use anyhow::Result;
use dioxus::prelude::*;
use models::PingResponse;
use reqwest;

const SERVER_BASE_URI: &str = env!("SERVER_BASE_URI");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

/// Shared layout component
#[component]
fn Layout() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

async fn do_ping() -> Result<String> {
    let request_path = format!("{}/api/v1/ping", SERVER_BASE_URI);
    let response = reqwest::get(request_path)
        .await?
        .json::<PingResponse>()
        .await?;

    return Ok(response.message);
}

/// Home page
#[component]
pub fn Home() -> Element {
    let mut ping_result: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        div {
            button {
                id: "ping-btn",
                onclick: move |_| {
                    spawn(async move{
                        let result = do_ping().await;
                        match result {
                            Ok(message) => {
                                ping_result.set(Some(message));
                            }
                            Err(_) => {
                                ping_result.set(None);
                            }
                        }
                    });
                },
                "Ping it",
            }
            p {
                match ping_result() {
                    Some(message) => rsx! { "{message}" },
                    None => rsx!()
                }
            }
        }
    }
}
