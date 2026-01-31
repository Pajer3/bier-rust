use dioxus::prelude::*;
pub mod view_model;
use view_model::ResetPasswordViewModel;
use crate::components::inputs::GlassInput;
use crate::NavState;

#[component]
pub fn ResetPasswordScreen(token: String) -> Element {
    let vm = use_signal(|| ResetPasswordViewModel::new(token));
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
                "nieuw wachtwoord."
            }

            if (vm().success)() {
                div {
                    style: "width: 100%; padding: 20px; background: rgba(50, 215, 75, 0.1); border: 1px solid var(--success); border-radius: 12px; color: var(--success); margin-bottom: 30px; text-align: center;",
                    p { "Je wachtwoord is aangepast! 🔐" }
                    button {
                        class: "glass-btn",
                        style: "margin-top: 20px;",
                        onclick: move |_| nav.set(NavState::Login),
                        "NAAR LOGIN"
                    }
                }
            } else {
                if let Some(msg) = (vm().error_msg)() {
                    div {
                        style: "width: 100%; padding: 12px; background: rgba(255,0,0,0.1); border: 1px solid rgba(255,0,0,0.3); border-radius: 8px; color: #ff5555; margin-bottom: 20px; font-size: 14px;",
                        "{msg}"
                    }
                }

                div {
                    style: "width: 100%; display: flex; flex-direction: column; gap: 16px; margin-bottom: 24px;",
                    div { class: "label", "Nieuw Wachtwoord" }
                    GlassInput {
                        r#type: "password",
                        placeholder: "minimaal 8 tekens",
                        value: (vm().new_password)(),
                        oninput: move |evt: FormEvent| {
                            let mut pass = vm().new_password;
                            pass.set(evt.value());
                        }
                    }

                    div { class: "label", "Wachtwoord bevestigen" }
                    GlassInput {
                        r#type: "password",
                        placeholder: "Herhaal je wachtwoord",
                        value: (vm().confirm_password)(),
                        oninput: move |evt: FormEvent| {
                            let mut confirm = vm().confirm_password;
                            confirm.set(evt.value());
                        }
                    }
                }

                button {
                    class: "glass-btn",
                    onclick: move |_| {
                        spawn(async move {
                            vm().perform_reset().await;
                        });
                    },
                    "WACHTWOORD OPSLAAN"
                }
            }
        }
    }
}
