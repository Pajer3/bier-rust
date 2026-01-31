use dioxus::prelude::*;
pub mod view_model;
use view_model::RegisterViewModel;
use crate::components::inputs::GlassInput;
use crate::{AuthState, NavState};

#[component]
pub fn RegisterScreen() -> Element {
    let vm = use_signal(|| RegisterViewModel::new());
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
            style: "width: 100%; max-maxWidth: 400px; padding: 20px; padding-bottom: 120px; display: flex; flex-direction: column; align-items: center;",

            h1 {
                style: "align-self: flex-start; margin-bottom: 40px;",
                "registreren."
            }

            if (vm().success)() {
                div {
                    style: "width: 100%; padding: 20px; background: rgba(50, 215, 75, 0.1); border: 1px solid var(--success); border-radius: 12px; color: var(--success); margin-bottom: 30px; text-align: center;",
                    p { "Account aangemaakt! 🍻" }
                    p {
                        style: "font-size: 14px; margin-top: 8px;",
                        "Check je e-mail voor de verificatiecode."
                    }
                    button {
                        class: "glass-btn",
                        style: "margin-top: 20px;",
                        onclick: move |_| nav.set(NavState::VerifyEmail),
                        "CODE INVULLEN"
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
                    style: "width: 100%; display: flex; flex-direction: column; gap: 16px; margin-bottom: 16px;",

                    div { class: "label", "Gebruikersnaam" }
                    GlassInput {
                        placeholder: "Bijv. JanBier",
                        value: (vm().username)(),
                        oninput: move |evt: FormEvent| {
                            let mut username = vm().username;
                            username.set(evt.value());
                        }
                    }

                    div { class: "label", "Email" }
                    GlassInput {
                        placeholder: "email@adres.nl",
                        value: (vm().email)(),
                        oninput: move |evt: FormEvent| {
                            let mut email = vm().email;
                            email.set(evt.value());
                        }
                    }

                    div { class: "label", "Wachtwoord" }
                    GlassInput {
                        r#type: "password",
                        placeholder: "Minimaal 8 tekens",
                        value: (vm().password)(),
                        oninput: move |evt: FormEvent| {
                            let mut password = vm().password;
                            password.set(evt.value());
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
                    style: "margin-top: 20px;",
                    onclick: move |_| {
                        spawn(async move {
                            vm().perform_register(auth).await;
                        });
                    },
                    "ACCOUNT AANMAKEN"
                }

                div {
                    style: "margin-top: 24px; font-size: 14px; color: var(--text2);",
                    "heb je al een account? "
                    a {
                        style: "font-weight: 700; color: var(--text); cursor: pointer;",
                        onclick: move |evt| {
                            evt.prevent_default();
                            nav.set(NavState::Login);
                        },
                        "Log hier in."
                    }
                }
            }
        }
    }
}
