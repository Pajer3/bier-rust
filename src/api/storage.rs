use std::fs;
use crate::AuthState;

const STORAGE_FILE: &str = "session.json";

pub fn save_auth_state(state: &AuthState) {
    if let Ok(json) = serde_json::to_string(state) {
        let _ = fs::write(STORAGE_FILE, json);
    }
}

pub fn load_auth_state() -> AuthState {
    if let Ok(json) = fs::read_to_string(STORAGE_FILE) {
        if let Ok(state) = serde_json::from_str::<AuthState>(&json) {
            return state;
        }
    }
    AuthState::default()
}

pub fn clear_auth_state() {
    let _ = fs::remove_file(STORAGE_FILE);
}
