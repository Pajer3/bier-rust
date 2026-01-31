use dioxus::prelude::*;
pub mod view_model;
use view_model::VerifyEmailViewModel;
use crate::components::inputs::GlassInput;
use crate::{NavState, AuthState};

#[component]
pub fn VerifyEmailScreen() -> Element {
    let vm = use_signal(|| VerifyEmailViewModel::new(None));
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
                "verificeren."
            }

            if (vm().success)() {
                div {
                    style: "width: 100%; padding: 20px; background: rgba(50, 215, 75, 0.1); border: 1px solid var(--success); border-radius: 12px; color: var(--success); margin-bottom: 30px; text-align: center;",
                    p { "Je account is geverifieerd! 🎉 Je kunt nu inloggen." }
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

                p {
                    style: "color: var(--text2); margin-bottom: 32px; font-size: 15px;",
                    "Vul de code in die we naar je e-mail hebben gestuurd om je account te activeren."
                }

                div {
                    style: "width: 100%; display: flex; flex-direction: column; gap: 16px; margin-bottom: 24px;",
                    div { class: "label", "Verificatie Code" }
                    GlassInput {
                        placeholder: "plak je code hier",
                        value: (vm().token)(),
                        oninput: move |evt: FormEvent| {
                            let mut token = vm().token;
                            token.set(evt.value());
                        }
                    }
                }

                button {
                    class: "glass-btn",
                    onclick: move |_| {
                        spawn(async move {
                            vm().perform_verify(Some(auth)).await;
                        });
                    },
                    "BEVESTIGEN"
                }

                div {
                    style: "margin-top: 24px; font-size: 14px; color: var(--text2);",
                    "Geen code ontvangen? "
                    a {
                        href: "#",
                        style: "font-weight: 700; color: var(--text);",
                        "Stuur opnieuw."
                    }
                }
            }
        }
    }
}
