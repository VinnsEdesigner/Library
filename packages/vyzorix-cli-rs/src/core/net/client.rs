use crate::core::state::VyzoState;
use reqwest::{Client, header};
use std::time::Duration;

pub fn build_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("vyzorix-cli/1.0"));
    
    let state = VyzoState::load();
    if let Some(token) = state.session_token {
        let auth_val = format!("Bearer {}", token);
        if let Ok(v) = header::HeaderValue::from_str(&auth_val) {
            headers.insert(header::AUTHORIZATION, v);
        }
    }
    
    Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to construct HTTP client context")
}
