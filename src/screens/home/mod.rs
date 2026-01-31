use dioxus::prelude::*;
use crate::AuthState;

#[component]
pub fn HomeScreen() -> Element {
    let mut auth = use_context::<Signal<AuthState>>();

    rsx! {
        div {
            class: "glass",
            style: "padding: 24px; color: var(--text); display: flex; flex-direction: column; gap: 20px;",
            h1 { "Welkom!" }
            p { "Je bent succesvol ingelogd." }
            div {
                style: "font-family: monospace; font-size: 12px; opacity: 0.7; word-break: break-all;",
                "Token: {auth().token.as_deref().unwrap_or(\"\")}"
            }
            button {
                class: "glass-btn",
                style: "max-width: 200px;",
                onclick: move |_| auth.write().token = None,
                "LOGOUT"
            }
        }
    }
}
