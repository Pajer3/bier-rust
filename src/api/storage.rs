use std::fs;
use std::path::PathBuf;
use crate::AuthState;
use directories::ProjectDirs;

fn get_storage_path() -> Option<PathBuf> {
    // Gebruik package qualifier "com", organization "bier", application "app"
    // Op Android zal dit waarschijnlijk mappen naar app-specifieke storage.
    if let Some(proj_dirs) = ProjectDirs::from("com", "bier", "app") {
        let data_dir = proj_dirs.data_dir();
        if !data_dir.exists() {
            let _ = fs::create_dir_all(data_dir);
        }
        return Some(data_dir.join("session.json"));
    }
    None
}

pub fn save_auth_state(state: &AuthState) {
    if let Some(path) = get_storage_path() {
        if let Ok(json) = serde_json::to_string(state) {
            let _ = fs::write(path, json);
        }
    }
}

pub fn load_auth_state() -> AuthState {
    if let Some(path) = get_storage_path() {
        if let Ok(json) = fs::read_to_string(path) {
            if let Ok(state) = serde_json::from_str::<AuthState>(&json) {
                return state;
            }
        }
    }
    AuthState::default()
}

pub fn clear_auth_state() {
    if let Some(path) = get_storage_path() {
        let _ = fs::remove_file(path);
    }
}
