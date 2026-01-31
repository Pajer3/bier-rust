use dioxus::prelude::*;
pub mod view_model;
use view_model::LoginViewModel;
use crate::components::inputs::GlassInput;
use crate::{AuthState, NavState};

#[component]
pub fn LoginScreen() -> Element {
    let vm = use_signal(|| LoginViewModel::new());
    let auth = use_context::<Signal<AuthState>>();
    let mut nav = use_context::<Signal<NavState>>();

    rsx! {
        if (vm().is_loading)() {
            div {
                class: "loading-overlay",
                div { class: "spinner" }
            }
        }

        div {
            class: "login-wrapper",
            style: "width: 100%; max-width: 400px; padding: 20px; padding-bottom: 120px; display: flex; flex-direction: column; align-items: center;",

            h1 {
                style: "align-self: flex-start; margin-bottom: 40px;",
                "inloggen."
            }

            if let Some(msg) = (vm().error_msg)() {
                div {
                    style: "width: 100%; padding: 12px; background: rgba(255,0,0,0.1); border: 1px solid rgba(255,0,0,0.3); border-radius: 8px; color: #ff5555; margin-bottom: 20px; font-size: 14px;",
                    "{msg}"
                }
            }

            div {
                style: "width: 100%; display: flex; flex-direction: column; gap: 16px; margin-bottom: 16px;",

                div { class: "label", "Email" }
                GlassInput {
                    placeholder: "",
                    value: (vm().email)(),
                    oninput: move |evt: FormEvent| {
                        let mut email = vm().email;
                        email.set(evt.value());
                    }
                }

                div { class: "label", "wachtwoord" }
                GlassInput {
                    r#type: "password",
                    placeholder: "",
                    value: (vm().password)(),
                    oninput: move |evt: FormEvent| {
                        let mut password = vm().password;
                        password.set(evt.value());
                    }
                }
            }

            a {
                href: "#",
                style: "align-self: flex-start; margin-bottom: 32px; font-weight: 600; font-size: 13px;",
                onclick: move |_| nav.set(NavState::ForgotPassword),
                "wachtwoord vergeten?"
            }

            button {
                class: "glass-btn",
                onclick: move |_| {
                    spawn(async move {
                        vm().perform_login(auth).await;
                    });
                },
                "LOGIN"
            }

            div {
                style: "margin-top: 24px; font-size: 14px; color: var(--text2);",
                "geen account? "
                a {
                    href: "#",
                    style: "font-weight: 700; color: var(--text);",
                    onclick: move |_| nav.set(NavState::Register),
                    "Nieuw account."
                }
            }
        }
    }
}
