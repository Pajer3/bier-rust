use tokio::fs;
use base64::{engine::general_purpose, Engine as _};

pub async fn load_default_avatar() -> String {
    let img_path = "../assets/user_data/default/default_avatar.png";
    match fs::read(img_path).await {
        Ok(bytes) => {
            format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(bytes))
        }
        Err(e) => {
            tracing::error!("Kon default avatar niet laden op pad {}: {}", img_path, e);
            "".to_string()
        }
    }
}

pub async fn create_user_directory(user_id: i32) -> Result<String, String> {
    let user_dir = format!("../assets/user_data/{}", user_id);
    fs::create_dir_all(&user_dir).await.map_err(|e| {
        tracing::error!("Kon user directory niet maken: {}", e);
        format!("Filesystem error: {}", e)
    })?;
    Ok(user_dir)
}

pub async fn save_encrypted_metadata(user_dir: &str, encrypted_hex: &str) -> Result<(), String> {
    let metadata_path = format!("{}/metadata.bin", user_dir);
    fs::write(metadata_path, encrypted_hex).await.map_err(|e| {
        format!("Failed to write metadata: {}", e)
    })
}

pub async fn load_encrypted_metadata(user_id: i32) -> Result<String, String> {
    let metadata_path = format!("../assets/user_data/{}/metadata.bin", user_id);
    fs::read_to_string(metadata_path).await.map_err(|e| {
        format!("Failed to read metadata file: {}", e)
    })
}
