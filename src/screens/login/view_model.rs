use dioxus::prelude::*;
use serde_json::json;
use crate::AuthState;

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
        let email = self.email;
        let password = self.password;

        is_loading.set(true);
        error_msg.set(None);

        let client = reqwest::Client::new();
        let query = json!({
            "query": "mutation($email: String!, $password: String!) { loginUser(email: $email, password: $password) { token user { id } } }",
            "variables": {
                "email": email(),
                "password": password()
            }
        });

        match client.post("http://10.0.2.2:3000/graphql")
            .json(&query)
            .send()
            .await 
        {
            Ok(resp) => {
                let body_res: Result<serde_json::Value, _> = resp.json().await;
                if let Ok(body) = body_res {
                    if let Some(token) = body["data"]["loginUser"]["token"].as_str() {
                        let user_id = body["data"]["loginUser"]["user"]["id"].as_i64().map(|id| id as i32);
                        
                        let mut auth_write = auth.write();
                        auth_write.token = Some(token.to_string());
                        auth_write.user_id = user_id;
                    } else if let Some(errors) = body["errors"].as_array() {
                        let msg = errors[0]["message"].as_str().unwrap_or("Onbekende fout");
                        error_msg.set(Some(msg.to_string()));
                    } else {
                        error_msg.set(Some("Server fout".to_string()));
                    }
                } else {
                    error_msg.set(Some("Kan antwoord niet lezen".to_string()));
                }
            }
            Err(e) => {
                error_msg.set(Some(format!("Connectie fout: {}", e)));
            }
        }
        is_loading.set(false);
    }
}
