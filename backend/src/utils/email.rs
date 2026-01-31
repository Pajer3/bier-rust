use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct EmailPayload {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
}

pub async fn send_verification_email(to_email: &str, token: &str) -> Result<(), String> {
    let api_key = env::var("RESEND_API_KEY").map_err(|_| "RESEND_API_KEY not set")?;
    // Later vervangen door de echte URL (deep link of web)
    let verification_url = format!("http://10.0.2.2:3000/verify?token={}", token);

    let payload = EmailPayload {
        from: "Bier <onboarding@resend.dev>".to_string(),
        to: vec![to_email.to_string()],
        subject: "Bevestig je Bier account! 🍻".to_string(),
        html: format!(
            "<h1>Welkom bij Bier </h1><p>Klik op de onderstaande link om je e-mail te bevestigen:</p><p><a href='{}' style='padding: 10px 20px; background: #0A84FF; color: white; border-radius: 8px; text-decoration: none;'>Email Bevestigen</a></p><p>Of kopieer deze code: <b>{}</b></p>",
            verification_url,
            token
        ),
    };

    send_resend_email(api_key, payload).await
}

pub async fn send_reset_email(to_email: &str, token: &str) -> Result<(), String> {
    let api_key = env::var("RESEND_API_KEY").map_err(|_| "RESEND_API_KEY not set")?;
    let reset_url = format!("http://10.0.2.2:3000/reset-password?token={}", token);

    let payload = EmailPayload {
        from: "Bier <onboarding@resend.dev>".to_string(),
        to: vec![to_email.to_string()],
        subject: "Wachtwoord herstellen - Bier 🔑".to_string(),
        html: format!(
            "<h1>Wachtwoord herstellen</h1><p>Je hebt een aanvraag gedaan om je wachtwoord te herstellen. Klik op de knop hieronder:</p><p><a href='{}' style='padding: 10px 20px; background: #FF453A; color: white; border-radius: 8px; text-decoration: none;'>Wachtwoord Herstellen</a></p><p>Als jij dit niet was, kun je deze mail negeren.</p>",
            reset_url
        ),
    };

    send_resend_email(api_key, payload).await
}

async fn send_resend_email(api_key: String, payload: EmailPayload) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.resend.com/emails")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        Ok(())
    } else {
        let err_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Resend error: {}", err_text))
    }
}
