mod components;

use crate::components::login::Login;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Login,
    Hero,
}

#[component]
fn App() -> Element {
    let mut current_page = use_signal(|| Page::Login);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }

        div {
            class: "body",
            match current_page() {
                Page::Login => rsx! {
                    Login {
                        on_login: move |_| current_page.set(Page::Hero)
                    }
                },
                Page::Hero => rsx! {
                    div {
                        class: "glass",
                        style: "padding: 20px; color: var(--text);",
                        "Ingelogd! (Hier komt de homepagina)"
                    }
                }
            }
        }
    }
}
