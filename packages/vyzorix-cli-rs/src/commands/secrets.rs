use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::cli::SecretAction;
use crate::core::crypto::aes::encrypt_secret;
use crate::core::state::VyzoState;
use crate::core::net::client::build_client;
use serde_json::json;

pub async fn execute(action: SecretAction) -> Result<()> {
    match action {
        SecretAction::Set { key, value } => {
            println!("{}", "Configuring Secure Environment Secrets...".truecolor(225, 29, 72).bold());
            
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.white} {msg}")
                    .unwrap(),
            );
            pb.enable_steady_tick(Duration::from_millis(120));

            pb.set_message("Applying AES-256-GCM encryption protocol...");
            let state = VyzoState::load();
            let machine_key_hex = state.machine_key.unwrap_or_else(|| "00".repeat(32));
            let mut master_key = [0u8; 32];
            if let Ok(decoded) = hex::decode(&machine_key_hex) {
                if decoded.len() == 32 {
                    master_key.copy_from_slice(&decoded);
                }
            }
            
            if let Some((ciphertext, nonce)) = encrypt_secret(&master_key, value.as_bytes()) {
                let encoded_cipher = hex::encode(ciphertext);
                let encoded_nonce = hex::encode(nonce);
                
                pb.set_message(format!("Synchronizing secret '{}' with Vyzorix Secure Enclave...", key));
                
                let client = build_client();
                let payload = json!({
                    "key": key,
                    "cipher": encoded_cipher,
                    "nonce": encoded_nonce
                });
                
                let _ = client.post("https://api.vyzorix.com/v1/secrets/set").json(&payload).send().await;

                pb.finish_with_message(format!(
                    "{} Secret {} securely stored.\n  {}: {}\n  {}: {}", 
                    "✔".truecolor(225, 29, 72), 
                    key.white().bold(),
                    "Ciphertext".black().bright_black(), encoded_cipher.black().bright_black(),
                    "Nonce".black().bright_black(), encoded_nonce.black().bright_black()
                ));
            } else {
                pb.finish_with_message(format!("{} Encryption payload failed.", "✖".red()));
            }
        }
        SecretAction::List => {
            println!("{}", "Retrieving active Vault Registry...".truecolor(225, 29, 72).bold());
            
            match crate::services::secrets::list_secrets().await {
                Ok(secrets) => {
                    println!("\n{}", "Environment Secrets [vyzorix.cloud]:".white().bold());
                    for secret in secrets {
                        println!("  {} {} [{}]", "🔑".truecolor(225, 29, 72), secret.key, secret.provider.black().bright_black());
                    }
                    println!("\n{}", "Use `vyzorix secrets set <KEY> <VALUE>` to inject new variables.".black().bright_black());
                }
                Err(e) => {
                    println!("{} Vault retrieval failed: {}", "✖".red(), e);
                }
            }
        }
    }
    
    Ok(())
}
