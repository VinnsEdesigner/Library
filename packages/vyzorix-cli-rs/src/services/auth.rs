use crate::core::net::client::build_client;
use crate::core::state::VyzoState;
use crate::error::VyzoError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use axum::{routing::get, Router, extract::Query};

#[derive(Deserialize)]
pub struct AuthCallback {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionStatus {
    pub authenticated: bool,
    pub user: Option<String>,
    pub organization: Option<String>,
}

pub async fn check_session() -> Result<SessionStatus, VyzoError> {
    let mut state = VyzoState::load();
    let client = build_client();
    
    let token = match &state.session_token {
        Some(t) => t,
        None => return Ok(SessionStatus { authenticated: false, user: None, organization: None }),
    };

    let res = client
        .get("https://api.vyzorix.com/v1/auth/session")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let status = res.json::<SessionStatus>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse session: {}", e)))?;
        Ok(status)
    } else {
        // Clear expired token
        state.session_token = None;
        let _ = state.save();
        Ok(SessionStatus { authenticated: false, user: None, organization: None })
    }
}

pub async fn login_device(_email: &str) -> Result<String, VyzoError> {
    let (tx, rx) = oneshot::channel();
    
    let app = Router::new().route("/callback", get(|query: Query<AuthCallback>| async move {
        let _ = tx.send(query.code.clone());
        "Authentication successful! You can close this window."
    }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9988));
    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| VyzoError::NetworkError(format!("Failed to bind loopback: {}", e)))?;

    // In a real scenario, we'd open the browser here
    println!("{} Open your browser to: https://auth.vyzorix.com/authorize?redirect_uri=http://localhost:9988/callback", "ℹ".blue());
    
    let server = axum::serve(listener, app);
    
    tokio::select! {
        _ = server => {
            Err(VyzoError::NetworkError("Auth server stopped unexpectedly".into()))
        }
        code = rx => {
            let code = code.map_err(|_| VyzoError::NetworkError("Failed to receive code".into()))?;
            let token = exchange_code_for_token(&code).await?;
            
            let mut state = VyzoState::load();
            state.session_token = Some(token.clone());
            state.save().map_err(|e| VyzoError::IoError(e))?;
            
            Ok(token)
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(300)) => {
            Err(VyzoError::NetworkError("Authentication timed out".into()))
        }
    }
}

async fn exchange_code_for_token(code: &str) -> Result<String, VyzoError> {
    let client = build_client();
    let res = client
        .post("https://api.vyzorix.com/v1/auth/token")
        .json(&json!({ "code": code }))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        Ok("access_token_v1".to_string())
    } else {
        Err(VyzoError::NetworkError(format!("Token exchange failed: {}", res.status())))
    }
}

pub async fn logout_device() -> Result<(), VyzoError> {
    let mut state = VyzoState::load();
    let token = state.session_token.clone();
    
    if let Some(token) = token {
        let client = build_client();
        let _ = client
            .post("https://api.vyzorix.com/v1/auth/logout")
            .bearer_auth(token)
            .send()
            .await;
    }

    state.session_token = None;
    state.save().map_err(|e| VyzoError::IoError(e))?;
    Ok(())
}
