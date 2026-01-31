use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::api::{GraphQLRequest, config::API_URL};

#[derive(Serialize)]
struct ResetPasswordVariables {
    token: String,
    #[serde(rename = "newPassword")]
    new_password: String,
}

#[derive(Deserialize)]
struct ResetPasswordResponse {
    data: Option<ResetPasswordData>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize)]
struct ResetPasswordData {
    #[serde(rename = "resetPassword")]
    reset_password: bool,
}

#[derive(Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ResetPasswordViewModel {
    pub token: Signal<String>,
    pub new_password: Signal<String>,
    pub confirm_password: Signal<String>,
    pub is_loading: Signal<bool>,
    pub success: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
}

impl ResetPasswordViewModel {
    pub fn new(token: String) -> Self {
        Self {
            token: Signal::new(token),
            new_password: Signal::new(String::new()),
            confirm_password: Signal::new(String::new()),
            is_loading: Signal::new(false),
            success: Signal::new(false),
            error_msg: Signal::new(None),
        }
    }

    pub async fn perform_reset(&self) {
        let mut is_loading = self.is_loading;
        let mut success = self.success;
        let mut error_msg = self.error_msg;

        if (self.new_password)().len() < 8 {
            error_msg.set(Some("Maak je wachtwoord wat sterker (minstens 8 tekens).".to_string()));
            return;
        }
        if (self.new_password)() != (self.confirm_password)() {
            error_msg.set(Some("De wachtwoorden komen niet overeen. Probeer het nog eens.".to_string()));
            return;
        }

        is_loading.set(true);
        error_msg.set(None);

        let client = reqwest::Client::new();
        let request = GraphQLRequest {
            query: "mutation($token: String!, $newPassword: String!) { resetPassword(token: $token, newPassword: $newPassword) }",
            variables: ResetPasswordVariables {
                token: (self.token)(),
                new_password: (self.new_password)(),
            },
        };

        match client.post(API_URL)
            .json(&request)
            .send()
            .await 
        {
            Ok(resp) => {
                match resp.json::<ResetPasswordResponse>().await {
                    Ok(body) => {
                        if let Some(data) = body.data {
                            if data.reset_password {
                                success.set(true);
                            } else {
                                error_msg.set(Some("Wachtwoord herstellen mislukt".to_string()));
                            }
                        } else if let Some(errors) = body.errors {
                            error_msg.set(Some(errors[0].message.clone()));
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
