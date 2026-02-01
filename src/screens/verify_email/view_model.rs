use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::AuthState;
use crate::api::{GraphQLRequest, config::API_URL};

#[derive(Serialize)]
struct VerifyEmailVariables {
    token: String,
}

#[derive(Deserialize)]
struct VerifyEmailResponse {
    data: Option<VerifyEmailData>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize)]
struct VerifyEmailData {
    #[serde(rename = "verifyEmail")]
    verify_email: bool,
}

#[derive(Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Clone, Copy, PartialEq)]
pub struct VerifyEmailViewModel {
    pub token: Signal<String>,
    pub is_loading: Signal<bool>,
    pub success: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
}

impl VerifyEmailViewModel {
    pub fn new(token: Option<String>) -> Self {
        Self {
            token: Signal::new(token.unwrap_or_default()),
            is_loading: Signal::new(false),
            success: Signal::new(false),
            error_msg: Signal::new(None),
        }
    }

    pub async fn perform_verify(&self, auth: Option<Signal<AuthState>>) {
        let mut is_loading = self.is_loading;
        let mut success = self.success;
        let mut error_msg = self.error_msg;

        if (self.token)().trim().is_empty() {
            error_msg.set(Some("Vergeet de code niet in te vullen!".to_string()));
            return;
        }

        is_loading.set(true);
        error_msg.set(None);

        let client = reqwest::Client::new();
        let request = GraphQLRequest {
            query: "mutation($token: String!) { verifyEmail(token: $token) }",
            variables: VerifyEmailVariables {
                token: (self.token)(),
            },
        };

        match client.post(API_URL)
            .json(&request)
            .send()
            .await 
        {
            Ok(resp) => {
                match resp.json::<VerifyEmailResponse>().await {
                    Ok(body) => {
                        if let Some(data) = body.data {
                            if data.verify_email {
                                success.set(true);
                                if let Some(mut a) = auth {
                                    a.write().is_verified = true;
                                    crate::api::storage::save_auth_state(&a.read());
                                }
                            } else {
                                error_msg.set(Some("Verificatie mislukt".to_string()));
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
