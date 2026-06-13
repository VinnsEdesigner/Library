use tracing::info;
use colored::*;

pub async fn stream_events() {
    let ts = chrono::Utc::now().timestamp_millis();
    let method = "GET".green();
    let path = "/api/v1/auth/session".white();
    let status = "200 OK".truecolor(225, 29, 72);
    let ms = "43ms".black().bright_black();
    
    println!("  [{}] {} {} - {} ({})", ts, method, path, status, ms);
}
