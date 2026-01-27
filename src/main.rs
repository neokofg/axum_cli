mod commands;
mod generators;
mod templates;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "axum")]
#[command(author, version, about = "CLI tool for Axum Template development", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project from template
    New {
        /// Project name
        name: String,
        /// Template repository path (optional)
        #[arg(short, long)]
        template: Option<String>,
    },

    /// Generate project components
    #[command(subcommand)]
    Generate(GenerateCommands),

    /// Alias for generate
    #[command(subcommand, name = "g")]
    G(GenerateCommands),

    /// Database operations
    #[command(subcommand)]
    Db(DbCommands),

    /// Run development server with auto-reload
    Dev {
        /// Port to run on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },

    /// Run tests
    Test {
        /// Test filter pattern
        pattern: Option<String>,
        /// Run only unit tests
        #[arg(long)]
        unit: bool,
        /// Run only integration tests
        #[arg(long)]
        integration: bool,
    },

    /// Docker operations
    #[command(subcommand)]
    Docker(DockerCommands),

    /// Check code quality (clippy + fmt)
    Check {
        /// Auto-fix issues
        #[arg(long)]
        fix: bool,
    },

    /// Format code
    Fmt {
        /// Check only, don't modify
        #[arg(long)]
        check: bool,
    },

    /// Build project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
    },

    /// List all features in the project
    Features,

    /// Show project info
    Info,
}

#[derive(Subcommand, Clone)]
enum GenerateCommands {
    /// Generate a new feature module
    Feature {
        /// Feature name (e.g., "posts", "comments")
        name: String,
    },

    /// Generate a database migration
    Migration {
        /// Migration name (e.g., "create_posts")
        name: String,
    },

    /// Generate a model
    Model {
        /// Model name (e.g., "Post")
        name: String,
        /// Feature to add model to
        #[arg(short, long)]
        feature: String,
        /// Fields in format "name:type" (e.g., "title:String content:Text")
        #[arg(short = 'f', long = "field", num_args = 1..)]
        fields: Vec<String>,
    },

    /// Generate a handler
    Handler {
        /// Handler name
        name: String,
        /// Feature to add handler to
        #[arg(short, long)]
        feature: String,
    },

    /// Generate CRUD for a feature
    Crud {
        /// Feature name
        name: String,
        /// Fields in format "name:type"
        #[arg(short = 'f', long = "field", num_args = 1..)]
        fields: Vec<String>,
    },

    /// Generate a worker/background job
    Worker {
        /// Worker name
        name: String,
    },

    /// Generate a middleware
    Middleware {
        /// Middleware name
        name: String,
    },
}

#[derive(Subcommand, Clone)]
enum DbCommands {
    /// Run pending migrations
    Migrate,

    /// Rollback last migration
    Rollback {
        /// Number of migrations to rollback
        #[arg(short, long, default_value = "1")]
        step: u32,
    },

    /// Reset database (drop + create + migrate)
    Reset {
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Create database
    Create,

    /// Drop database
    Drop {
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Show migration status
    Status,

    /// Generate schema.rs from database
    Schema,

    /// Seed database with test data
    Seed,
}

#[derive(Subcommand, Clone)]
enum DockerCommands {
    /// Start Docker containers
    Up {
        /// Run in detached mode
        #[arg(short, long)]
        detach: bool,
    },

    /// Stop Docker containers
    Down {
        /// Remove volumes
        #[arg(short, long)]
        volumes: bool,
    },

    /// Show container status
    Status,

    /// Show container logs
    Logs {
        /// Service name (postgres, redis)
        service: Option<String>,
        /// Follow logs
        #[arg(short, long)]
        follow: bool,
    },

    /// Restart containers
    Restart,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name, template } => commands::new_project(&name, template.as_deref()).await,
        Commands::Generate(cmd) | Commands::G(cmd) => handle_generate(cmd).await,
        Commands::Db(cmd) => handle_db(cmd).await,
        Commands::Dev { port } => commands::dev::run(port).await,
        Commands::Test {
            pattern,
            unit,
            integration,
        } => commands::test::run(pattern.as_deref(), unit, integration).await,
        Commands::Docker(cmd) => handle_docker(cmd).await,
        Commands::Check { fix } => commands::check::run(fix).await,
        Commands::Fmt { check } => commands::fmt::run(check).await,
        Commands::Build { release } => commands::build::run(release).await,
        Commands::Features => commands::features::list().await,
        Commands::Info => commands::info::show().await,
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    Ok(())
}

async fn handle_generate(cmd: GenerateCommands) -> Result<()> {
    match cmd {
        GenerateCommands::Feature { name } => generators::feature::generate(&name).await,
        GenerateCommands::Migration { name } => generators::migration::generate(&name).await,
        GenerateCommands::Model {
            name,
            feature,
            fields,
        } => generators::model::generate(&name, &feature, &fields).await,
        GenerateCommands::Handler { name, feature } => {
            generators::handler::generate(&name, &feature).await
        }
        GenerateCommands::Crud { name, fields } => generators::crud::generate(&name, &fields).await,
        GenerateCommands::Worker { name } => generators::worker::generate(&name).await,
        GenerateCommands::Middleware { name } => generators::middleware::generate(&name).await,
    }
}

async fn handle_db(cmd: DbCommands) -> Result<()> {
    match cmd {
        DbCommands::Migrate => commands::db::migrate().await,
        DbCommands::Rollback { step } => commands::db::rollback(step).await,
        DbCommands::Reset { force } => commands::db::reset(force).await,
        DbCommands::Create => commands::db::create().await,
        DbCommands::Drop { force } => commands::db::drop(force).await,
        DbCommands::Status => commands::db::status().await,
        DbCommands::Schema => commands::db::schema().await,
        DbCommands::Seed => commands::db::seed().await,
    }
}

async fn handle_docker(cmd: DockerCommands) -> Result<()> {
    match cmd {
        DockerCommands::Up { detach } => commands::docker::up(detach).await,
        DockerCommands::Down { volumes } => commands::docker::down(volumes).await,
        DockerCommands::Status => commands::docker::status().await,
        DockerCommands::Logs { service, follow } => {
            commands::docker::logs(service.as_deref(), follow).await
        }
        DockerCommands::Restart => commands::docker::restart().await,
    }
}
