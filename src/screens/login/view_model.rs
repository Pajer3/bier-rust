use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::AuthState;
use crate::api::{GraphQLRequest, config::API_URL};

#[derive(Serialize)]
struct LoginVariables {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    data: Option<LoginData>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize)]
struct LoginData {
    #[serde(rename = "loginUser")]
    login_user: Option<LoginSuccess>,
}

#[derive(Deserialize)]
struct LoginSuccess {
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
pub struct LoginViewModel {
    pub email: Signal<String>,
    pub password: Signal<String>,
    pub error_msg: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
}

impl LoginViewModel {
    pub fn new() -> Self {
        Self {
            email: Signal::new(String::new()),
            password: Signal::new(String::new()),
            error_msg: Signal::new(None),
            is_loading: Signal::new(false),
        }
    }

    pub async fn perform_login(&self, mut auth: Signal<AuthState>) {
        let mut is_loading = self.is_loading;
        let mut error_msg = self.error_msg;

        is_loading.set(true);
        error_msg.set(None);

        let client = reqwest::Client::new();
        
        let request = GraphQLRequest {
            query: "mutation($email: String!, $password: String!) { loginUser(email: $email, password: $password) { token user { id isVerified } } }",
            variables: LoginVariables {
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
                match resp.json::<LoginResponse>().await {
                    Ok(body) => {
                        if let Some(data) = body.data {
                            if let Some(success) = data.login_user {
                                let mut auth_write = auth.write();
                                auth_write.token = Some(success.token);
                                auth_write.user_id = Some(success.user.id);
                                auth_write.is_verified = success.user.is_verified;
                                crate::api::storage::save_auth_state(&auth_write);
                            }
                        } else if let Some(errors) = body.errors {
                            error_msg.set(Some(errors[0].message.clone()));
                        } else {
                            error_msg.set(Some("Onbekende server fout".to_string()));
                        }
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Data fout: {}", e)));
                    }
                }
            }
            Err(e) => {
                error_msg.set(Some(format!("Netwerk fout: {}", e)));
            }
        }
        is_loading.set(false);
    }
}
