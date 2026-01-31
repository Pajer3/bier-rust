mod components;
mod screens;
mod api;

use dioxus::prelude::*;
use screens::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuthState {
    pub token: Option<String>,
    pub user_id: Option<i32>,
    pub is_verified: bool,
}

#[derive(Clone, PartialEq)]
pub enum NavState {
    Login,
    Register,
    ForgotPassword,
    ResetPassword(String),
    VerifyEmail,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    // Globale auth state die overal in de app beschikbaar is
    let auth = use_context_provider(|| Signal::new(crate::api::storage::load_auth_state()));
    // Navigatie state voor als we nog niet ingelogd zijn
    let nav = use_context_provider(|| Signal::new(NavState::Login));

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }

        div {
            class: "body",
            if auth().token.is_none() {
                match nav() {
                    NavState::Login => rsx! { LoginScreen {} },
                    NavState::Register => rsx! { RegisterScreen {} },
                    NavState::ForgotPassword => rsx! { ForgotPasswordScreen {} },
                    NavState::VerifyEmail => rsx! { VerifyEmailScreen {} },
                    NavState::ResetPassword(token) => rsx! { ResetPasswordScreen { token } },
                }
            } else if !auth().is_verified {
                 VerifyEmailScreen {}
            } else {
                 HomeScreen {}
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
