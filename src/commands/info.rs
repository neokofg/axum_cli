use anyhow::Result;
use colored::Colorize;
use std::fs;

use crate::utils::ensure_in_project;

pub async fn show() -> Result<()> {
    let project_root = ensure_in_project()?;

    // Read Cargo.toml
    let cargo_toml_path = project_root.join("Cargo.toml");
    let cargo_content = fs::read_to_string(&cargo_toml_path)?;
    let cargo: toml::Value = cargo_content.parse()?;

    let name = cargo
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unknown");

    let version = cargo
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0");

    println!();
    println!("{}", "Project Information".bold().underline());
    println!();
    println!("  {} {}", "Name:".cyan(), name);
    println!("  {} {}", "Version:".cyan(), version);
    println!("  {} {}", "Path:".cyan(), project_root.display());
    println!();

    // Count features
    let features_dir = project_root.join("src/features");
    if features_dir.exists() {
        let feature_count = fs::read_dir(&features_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_dir()
                    && !e
                        .file_name()
                        .to_str()
                        .map(|s| s.starts_with('_'))
                        .unwrap_or(true)
            })
            .count();
        println!("  {} {}", "Features:".cyan(), feature_count);
    }

    // Count migrations
    let migrations_dir = project_root.join("migrations");
    if migrations_dir.exists() {
        let migration_count = fs::read_dir(&migrations_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .count();
        println!("  {} {}", "Migrations:".cyan(), migration_count);
    }

    // Check for Docker
    let docker_compose = project_root.join("docker-compose.yml");
    if docker_compose.exists() {
        println!("  {} {}", "Docker:".cyan(), "configured".green());
    }

    // Check for .env
    let env_file = project_root.join(".env");
    if env_file.exists() {
        println!("  {} {}", "Environment:".cyan(), ".env present".green());
    } else {
        println!(
            "  {} {}",
            "Environment:".cyan(),
            ".env missing (copy from .env.example)".yellow()
        );
    }

    println!();

    Ok(())
}
