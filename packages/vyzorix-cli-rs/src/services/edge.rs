use crate::types::edge::EdgeDeployment;
use crate::core::net::client::build_client;
use crate::error::VyzoError;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use serde::Deserialize;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncRead;

#[derive(Deserialize)]
pub struct DeployLog {
    pub step: String,
    pub message: String,
}

pub struct ProgressReader<R> {
    inner: R,
    progress: Arc<AtomicU64>,
}

impl<R: AsyncRead + Unpin> AsyncRead for ProgressReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let prev_len = buf.filled().len();
        match Pin::new(&mut self.inner).poll_read(cx, buf) {
            Poll::Ready(Ok(())) => {
                let read = buf.filled().len() - prev_len;
                self.progress.fetch_add(read as u64, Ordering::Relaxed);
                Poll::Ready(Ok(()))
            }
            res => res,
        }
    }
}

pub async fn deploy_bundle_with_progress<F>(path: &str, total_size: u64, on_progress: F) -> Result<EdgeDeployment, VyzoError> 
where F: Fn(u64) + Send + Sync + 'static {
    let client = build_client();
    
    let file = File::open(path).await.map_err(|e| VyzoError::IoError(e))?;
    let progress = Arc::new(AtomicU64::new(0));
    let progress_cloned = Arc::clone(&progress);
    
    // Background reporter
    tokio::spawn(async move {
        loop {
            let current = progress_cloned.load(Ordering::Relaxed);
            on_progress(current);
            if current >= total_size { break; }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    let reader = ProgressReader { inner: file, progress };
    let stream = FramedRead::new(reader, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);

    let res = client
        .post("https://api.vyzorix.com/v1/deploy")
        .body(body)
        .header("Content-Type", "application/gzip")
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let deployment = res.json::<EdgeDeployment>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse deployment response: {}", e)))?;
        Ok(deployment)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

pub async fn deploy_bundle(path: &str) -> Result<EdgeDeployment, VyzoError> {
    deploy_bundle_with_progress(path, 0, |_| {}).await
}

pub async fn fetch_deployment_logs(deployment_id: &str) -> Result<Vec<DeployLog>, VyzoError> {
    let client = build_client();
    let res = client
        .get(format!("https://api.vyzorix.com/v1/deploy/{}/logs", deployment_id))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let logs = res.json::<Vec<DeployLog>>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse logs: {}", e)))?;
        Ok(logs)
    } else {
        Ok(vec![]) // Fallback to empty if not ready
    }
}

pub async fn stream_deployment_logs(deployment_id: &str) -> Result<impl tokio_stream::Stream<Item = Result<DeployLog, VyzoError>>, VyzoError> {
    use tokio_stream::StreamExt;
    use futures_util::StreamExt as _;

    let client = build_client();
    let res = client
        .get(format!("https://api.vyzorix.com/v1/deploy/{}/logs/stream", deployment_id))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    Ok(res.bytes_stream().map(|b| {
        let b = b.map_err(|e| VyzoError::NetworkError(e.to_string()))?;
        serde_json::from_slice::<DeployLog>(&b)
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse log chunk: {}", e)))
    }))
}

