use crate::cli::VyzoCli;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
}

pub fn determine_environment(args: &VyzoCli) -> Environment {
    // Attempt to load .env variables, ignore if not present
    let _ = dotenvy::dotenv();

    // CLI args take precedence over system environment values
    if args.dev {
        return Environment::Development;
    }

    if let Ok(val) = std::env::var("NODE_ENV") {
        if val.to_lowercase() == "development" {
            return Environment::Development;
        }
    }

    // Default to Production if nothing overrides
    Environment::Production
}
