use crate::components::inputs::GlassInput;
use dioxus::prelude::*;

#[component]
pub fn Login(on_login: EventHandler<()>) -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        div {
            class: "login-wrapper",
            style: "width: 100%; max-width: 400px; padding: 20px; padding-bottom: 120px; display: flex; flex-direction: column; align-items: center;",

            // Header text
            h1 {
                style: "align-self: flex-start; margin-bottom: 40px;",
                "inloggen."
            }

            // Input fields
            div {
                style: "width: 100%; display: flex; flex-direction: column; gap: 16px; margin-bottom: 16px;",

                div {
                    class: "label",
                    "Email"
                }
                GlassInput {
                    placeholder: "",
                    value: "{email}",
                    oninput: move |evt: FormEvent| email.set(evt.value())
                }

                div {
                    class: "label",
                    "wachtwoord"
                }
                GlassInput {
                    r#type: "password",
                    placeholder: "",
                    value: "{password}",
                    oninput: move |evt: FormEvent| password.set(evt.value())
                }
            }

            // Forgot password link
            a {
                href: "#",
                style: "align-self: flex-start; margin-bottom: 32px; font-weight: 600; font-size: 13px;",
                "wachtwoord vergeten?"
            }

            // Login Button
            button {
                class: "glass-btn",
                onclick: move |_| on_login.call(()),
                "LOGIN"
            }

            // Create account link
            div {
                style: "margin-top: 24px; font-size: 14px; color: var(--text2);",
                "geen account? "
                a {
                    href: "#",
                    style: "font-weight: 700; color: var(--text);",
                    "Nieuw account."
                }
            }

            // Decoratieve afbeelding (Onderin en full width)
            img {
                src: asset!("/assets/login_image.png"),
                style: "position: fixed; bottom: 0; left: 0; width: 100%; opacity: 0.8;",
            }
        }
    }
}
