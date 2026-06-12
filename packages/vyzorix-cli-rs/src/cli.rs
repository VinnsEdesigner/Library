use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "vyzorix")]
#[command(author, version, about = "Vyzorix Workspace Management CLI", long_about = None)]
#[command(propagate_version = true)]
pub struct VyzoCli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force development mode (bypasses production checks)
    #[arg(long, global = true, env = "VYZORIX_DEV_MODE")]
    pub dev: bool,

    /// Enable verbose logging for debugging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scaffolds the environment and injects UI dependencies
    Init {
        #[arg(short, long)]
        force: bool,
    },
    /// Bootstraps local instances of themes configurations
    Theme {
        #[arg(short, long, default_value = "src/themes")]
        out_dir: String,
    },
    /// Asserts the integrity of vyzorix_session handling
    Doctor,
    /// Manage authentication with Vyzorix Cloud
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// Deploy the local workspace to Vyzorix Edge infrastructure
    Deploy {
        /// Target environment (preview, production)
        #[arg(short, long, default_value = "preview")]
        env: String,
    },
    /// Start the Vyzorix local development server & HMR watcher
    Dev {
        /// Port to bind the local server to
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Upgrade the Vyzorix CLI to the latest version via OTA release
    Upgrade {
        /// Force update even if already on the latest version
        #[arg(short, long)]
        force: bool,
    },
    /// Manage encrypted environment secrets
    Secrets {
        #[command(subcommand)]
        action: SecretAction,
    },
    /// Manage Vyzorix CLI extensions and plugins
    Plugin {
        #[command(subcommand)]
        action: PluginAction,
    },
    /// Database Schema and Migrations
    Db {
        #[command(subcommand)]
        action: DbAction,
    },
    /// Multi-Cloud Infrastructure Orchestration
    Infra {
        #[command(subcommand)]
        action: InfraAction,
    },
    /// Vyzorix Edge Analytics & Telemetry
    Analytics {
        #[command(subcommand)]
        action: AnalyticsAction,
    },
    /// CI/CD Pipeline Generators
    Cicd {
        #[command(subcommand)]
        action: CicdAction,
    },
    /// AI Autopilot & Workspace Optimization
    Ai {
        #[command(subcommand)]
        action: AiAction,
    },
    /// Manage Team and Workspaces
    Team {
        #[command(subcommand)]
        action: TeamAction,
    },
    /// Vyzorix Package Registry operations
    Registry {
        #[command(subcommand)]
        action: RegistryAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum AuthAction {
    /// Proceed with login via OAuth/Token
    Login {
        /// Force re-authentication
        #[arg(short, long)]
        force: bool,
    },
    /// Invalidate current session
    Logout,
    /// Check current session status
    Status,
}

#[derive(Subcommand, Debug)]
pub enum TeamAction {
    /// Invite a developer to the team
    Invite { email: String },
    /// List active team members
    List,
    /// Revoke developer access
    Revoke { email: String },
}

#[derive(Subcommand, Debug)]
pub enum RegistryAction {
    /// Publish the current workspace as a package
    Publish,
    /// Unpublish a package from the registry
    Unpublish { package_name: String },
    /// Search the registry for packages
    Search { query: String },
}

#[derive(Subcommand, Debug)]
pub enum DbAction {
    /// Run database migrations
    Migrate {
        /// Preview SQL without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Seed database with mock data
    Seed,
    /// Rollback the last migration
    Rollback,
    /// Check migration status
    Status,
}

#[derive(Subcommand, Debug)]
pub enum InfraAction {
    /// Provision AWS infrastructure
    Aws,
    /// Provision GCP infrastructure
    Gcp,
    /// Provision Azure infrastructure
    Azure,
    /// Check infra status
    Status,
}

#[derive(Subcommand, Debug)]
pub enum AnalyticsAction {
    /// View real-time edge telemetry
    Realtime,
    /// Generate trailing reports
    Report {
        #[arg(short, long, default_value_t = 7)]
        days: u32,
    },
    /// Check active alerts
    Alerts,
    /// Configure alert thresholds
    SetThreshold {
        /// Metric key (e.g., cpu, latency)
        key: String,
        /// Max value before alerting
        val: f64,
    },
}

#[derive(Subcommand, Debug)]
pub enum CicdAction {
    /// Generate GitHub Actions workflows
    Github,
    /// Generate GitLab CI pipelines
    Gitlab,
    /// Generate CircleCI configs
    Circleci,
}

#[derive(Subcommand, Debug)]
pub enum AiAction {
    /// Ask Vyzo AI to generate or modify workspace code
    Autopilot { prompt: String },
    /// Run AI code optimization
    Optimize,
    /// Security audit powered by AI
    Audit,
}

#[derive(Subcommand, Debug)]
pub enum SecretAction {
    /// Set a securely encrypted secret
    Set { key: String, value: String },
    /// List all available secret keys in the current environment
    List,
}

#[derive(Subcommand, Debug)]
pub enum PluginAction {
    /// Install a new CLI plugin
    Add { name: String },
    /// List installed plugins
    List,
}
