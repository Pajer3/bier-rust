mod components;
mod screens;

use dioxus::prelude::*;
use screens::{LoginScreen, HomeScreen};

#[derive(Clone, Default)]
pub struct AuthState {
    pub token: Option<String>,
    pub user_id: Option<i32>,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    // Globale auth state die overal in de app beschikbaar is
    let auth = use_context_provider(|| Signal::new(AuthState::default()));

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }

        div {
            class: "body",
            // Switch tussen Login en Home op basis van auth token
            if auth().token.is_none() {
                LoginScreen {}
            } else {
                HomeScreen {}
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
