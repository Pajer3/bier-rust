use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::api::{GraphQLRequest, config::API_URL};

#[derive(Serialize)]
struct ForgotPasswordVariables {
    email: String,
}

#[derive(Deserialize)]
struct ForgotPasswordResponse {
    data: Option<ForgotPasswordData>,
}

#[derive(Deserialize)]
struct ForgotPasswordData {
    #[serde(rename = "forgotPassword")]
    forgot_password: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ForgotPasswordViewModel {
    pub email: Signal<String>,
    pub is_loading: Signal<bool>,
    pub success: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
}

impl ForgotPasswordViewModel {
    pub fn new() -> Self {
        Self {
            email: Signal::new(String::new()),
            is_loading: Signal::new(false),
            success: Signal::new(false),
            error_msg: Signal::new(None),
        }
    }

    pub async fn request_reset(&self) {
        let mut is_loading = self.is_loading;
        let mut success = self.success;
        let mut error_msg = self.error_msg;

        let email = (self.email)();
        if !email.contains('@') || !email.contains('.') || email.len() < 5 {
            error_msg.set(Some("Oeps! Dat lijkt geen geldig e-mailadres te zijn.".to_string()));
            return;
        }

        is_loading.set(true);
        error_msg.set(None);

        let client = reqwest::Client::new();
        let request = GraphQLRequest {
            query: "mutation($email: String!) { forgotPassword(email: $email) }",
            variables: ForgotPasswordVariables {
                email: (self.email)(),
            },
        };

        match client.post(API_URL)
            .json(&request)
            .send()
            .await 
        {
            Ok(resp) => {
                match resp.json::<ForgotPasswordResponse>().await {
                    Ok(body) => {
                        if let Some(data) = body.data {
                            if data.forgot_password {
                                success.set(true);
                            } else {
                                error_msg.set(Some("E-mailadres niet gevonden of fout".to_string()));
                            }
                        } else {
                            error_msg.set(Some("Fout bij aanvraag".to_string()));
                        }
                    }
                    Err(_) => {
                        error_msg.set(Some("Er is een probleem met de gegevens. Onze fout!".to_string()));
                    }
                }
            }
            Err(_) => {
                error_msg.set(Some("Check je internetverbinding even, we kunnen de server niet bereiken.".to_string()));
            }
        }
        is_loading.set(false);
    }
}
