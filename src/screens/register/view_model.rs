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
    id: i32,
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
            error_msg: Signal::new(None),
            is_loading: Signal::new(false),
            success: Signal::new(false),
        }
    }

    pub async fn perform_register(&self, mut auth: Signal<AuthState>) {
        let mut is_loading = self.is_loading;
        let mut error_msg = self.error_msg;
        let mut success = self.success;

        is_loading.set(true);
        error_msg.set(None);

        let client = reqwest::Client::new();
        
        let request = GraphQLRequest {
            query: "mutation($username: String!, $email: String!, $password: String!) { registerUser(username: $username, email: $email, password: $password) { token user { id } } }",
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
                            if data.register_user.is_some() {
                                success.set(true);
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
