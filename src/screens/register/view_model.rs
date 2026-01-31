use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::AuthState;
use crate::api::{GraphQLRequest, config::API_URL};

#[derive(Serialize)]
struct RegisterVariables {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterResponse {
    data: Option<RegisterData>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize)]
struct RegisterData {
    #[serde(rename = "registerUser")]
    register_user: Option<RegisterSuccess>,
}

#[derive(Deserialize)]
struct RegisterSuccess {
    token: String,
    user: UserSmall,
}

#[derive(Deserialize)]
struct UserSmall {
    pub id: i32,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
}

#[derive(Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Clone, Copy, PartialEq)]
pub struct RegisterViewModel {
    pub username: Signal<String>,
    pub email: Signal<String>,
    pub password: Signal<String>,
    pub confirm_password: Signal<String>,
    pub error_msg: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    pub success: Signal<bool>,
}

impl RegisterViewModel {
    pub fn new() -> Self {
        Self {
            username: Signal::new(String::new()),
            email: Signal::new(String::new()),
            password: Signal::new(String::new()),
            confirm_password: Signal::new(String::new()),
            error_msg: Signal::new(None),
            is_loading: Signal::new(false),
            success: Signal::new(false),
        }
    }

    pub async fn perform_register(&self, mut auth: Signal<AuthState>) {
        let mut is_loading = self.is_loading;
        let mut error_msg = self.error_msg;
        let mut success = self.success;

        let username = (self.username)();
        let email = (self.email)();
        let password = (self.password)();

        // 🛡️ Frontend Validatie
        if username.trim().len() < 3 {
            error_msg.set(Some("Kies een gebruikersnaam van minstens 3 tekens.".to_string()));
            return;
        }
        if !email.contains('@') || !email.contains('.') || email.len() < 5 {
            error_msg.set(Some("Oeps! Dat lijkt geen geldig e-mailadres te zijn.".to_string()));
            return;
        }
        if password.len() < 8 {
            error_msg.set(Some("Maak je wachtwoord wat sterker (minstens 8 tekens).".to_string()));
            return;
        }
        if password != (self.confirm_password)() {
            error_msg.set(Some("De wachtwoorden komen niet overeen. Probeer het nog eens.".to_string()));
            return;
        }

        is_loading.set(true);
        error_msg.set(None);

        let client = reqwest::Client::new();
        
        let request = GraphQLRequest {
            query: "mutation($username: String!, $email: String!, $password: String!) { registerUser(username: $username, email: $email, password: $password) { token user { id isVerified } } }",
            variables: RegisterVariables {
                username: (self.username)(),
                email: (self.email)(),
                password: (self.password)(),
            },
        };

        match client.post(API_URL)
            .json(&request)
            .send()
            .await 
        {
            Ok(resp) => {
                match resp.json::<RegisterResponse>().await {
                    Ok(body) => {
                        if let Some(data) = body.data {
                            if let Some(reg_success) = data.register_user {
                                let mut auth_write = auth.write();
                                auth_write.token = Some(reg_success.token);
                                auth_write.user_id = Some(reg_success.user.id);
                                auth_write.is_verified = reg_success.user.is_verified;
                                crate::api::storage::save_auth_state(&auth_write);
                                success.set(true);
                            }
                        } else if let Some(errors) = body.errors {
                            error_msg.set(Some(errors[0].message.clone()));
                        } else {
                            error_msg.set(Some("Oeps, er ging iets mis bij de server. Probeer het later nog eens.".to_string()));
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
