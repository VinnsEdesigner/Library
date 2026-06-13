use crate::types::registry::RegistryPackage;
use crate::core::net::client::build_client;
use crate::error::VyzoError;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn publish_package(path: &str, integrity_hash: &str) -> Result<(), VyzoError> {
    let client = build_client();
    let file = File::open(path).await.map_err(|e| VyzoError::IoError(e))?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);

    let res = client
        .post("https://api.vyzorix.com/v1/registry/packages/publish")
        .header("X-Vyzo-Integrity", integrity_hash)
        .header("Content-Type", "application/gzip")
        .body(body)
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(VyzoError::NetworkError(format!("Failed to publish: {}", res.status())))
    }
}

pub async fn unpublish_package(name: &str) -> Result<(), VyzoError> {
    let client = build_client();
    
    let res = client
        .delete(format!("https://api.vyzorix.com/v1/registry/packages/{}", name))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(VyzoError::NetworkError(format!("Failed to unpublish: {}", res.status())))
    }
}

pub async fn get_package_meta(name: &str) -> Result<RegistryPackage, VyzoError> {
    let client = build_client();
    
    let res = client
        .get(format!("https://api.vyzorix.com/v1/registry/packages/{}", name))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let package = res.json::<RegistryPackage>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse registry package: {}", e)))?;
        Ok(package)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

pub async fn list_packages(query: &str) -> Result<Vec<RegistryPackage>, VyzoError> {
    let client = build_client();
    
    let res = client
        .get("https://api.vyzorix.com/v1/registry/packages")
        .query(&[("q", query)])
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let packages = res.json::<Vec<RegistryPackage>>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse registry list: {}", e)))?;
        Ok(packages)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

pub async fn check_namespace_availability(namespace: &str) -> Result<bool, VyzoError> {
    let client = build_client();
    let res = client
        .get(format!("https://api.vyzorix.com/v1/registry/check/{}", namespace))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    Ok(res.status().is_success())
}

pub async fn download_template(name: &str) -> Result<std::collections::HashMap<String, String>, VyzoError> {
    let client = build_client();
    let res = client
        .get(format!("https://api.vyzorix.com/v1/registry/templates/{}", name))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let files = res.json::<std::collections::HashMap<String, String>>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse template: {}", e)))?;
        Ok(files)
    } else {
        Err(VyzoError::NetworkError(format!("Template '{}' not found", name)))
    }
}

