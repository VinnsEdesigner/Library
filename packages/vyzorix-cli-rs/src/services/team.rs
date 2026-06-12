use serde_json::json;
use crate::types::team::TeamMember;
use crate::core::net::client::build_client;
use crate::error::VyzoError;

pub async fn invite_member(email: &str, role: &str) -> Result<(), VyzoError> {
    let client = build_client();
    let res = client
        .post("https://api.vyzorix.com/v1/team/invite")
        .json(&json!({ "email": email, "role": role }))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(VyzoError::NetworkError(format!("Failed to invite: {}", res.status())))
    }
}

pub async fn revoke_member(email: &str) -> Result<(), VyzoError> {
    let client = build_client();
    let res = client
        .post("https://api.vyzorix.com/v1/team/revoke")
        .json(&json!({ "email": email }))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(VyzoError::NetworkError(format!("Failed to revoke: {}", res.status())))
    }
}

pub async fn fetch_team_members() -> Result<Vec<TeamMember>, VyzoError> {
    let client = build_client();
    
    let res = client
        .get("https://api.vyzorix.com/v1/team/members")
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let members = res.json::<Vec<TeamMember>>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse team members: {}", e)))?;
        Ok(members)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

pub async fn fetch_roles() -> Result<Vec<String>, VyzoError> {
    let client = build_client();
    let res = client
        .get("https://api.vyzorix.com/v1/iam/roles")
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let roles = res.json::<Vec<String>>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse roles: {}", e)))?;
        Ok(roles)
    } else {
        Ok(vec!["admin".into(), "developer".into(), "viewer".into()])
    }
}
