use dioxus::prelude::*;
use crate::AuthState;

#[component]
pub fn ProfileScreen() -> Element {
    let mut auth = use_context::<Signal<AuthState>>();

    rsx! {
        div {
            class: "glass",
            style: "margin: 20px; padding: 20px;",
            h1 { "Mijn Profiel 👤" }
            p { "Statistieken en instellingen..." }
            button {
                class: "glass-btn",
                onclick: move |_| auth.write().token = None,
                "Uitloggen"
            }
        }
    }
}
