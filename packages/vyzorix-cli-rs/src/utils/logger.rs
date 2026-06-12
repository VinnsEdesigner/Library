use tracing_subscriber::{fmt, EnvFilter};
use crate::core::env::Environment;

pub fn init(env_mode: &Environment, verbose: bool) {
    // Configure log level based on environment & verbosity flag
    let default_level = if verbose {
        "debug"
    } else {
        match env_mode {
            Environment::Development => "info",
            Environment::Production => "warn",
        }
    };

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_level));

    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(verbose); // Only show module targets if we are in verbose mode

    if *env_mode == Environment::Production && !verbose {
        // In production, we might log JSON, or disable terminal styling
        subscriber.without_time().init();
    } else {
        // In development, keep standard styling
        subscriber.init();
    }
}
