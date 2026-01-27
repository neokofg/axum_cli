use anyhow::Result;
use chrono::Utc;
use std::fs;

use crate::utils::{ensure_in_project, print_info, print_success, to_snake_case};

pub async fn generate(name: &str) -> Result<()> {
    let project_root = ensure_in_project()?;
    let migration_name = to_snake_case(name);

    let timestamp = Utc::now().format("%Y-%m-%d-%H%M%S");
    let dir_name = format!("{}_{}", timestamp, migration_name);

    let migrations_dir = project_root.join("migrations");
    let migration_dir = migrations_dir.join(&dir_name);

    if !migrations_dir.exists() {
        fs::create_dir_all(&migrations_dir)?;
    }

    fs::create_dir_all(&migration_dir)?;

    print_info(&format!("Creating migration: {}", dir_name));

    // Create up.sql
    let up_content = format!(
        r#"-- Migration: {}
-- Created at: {}

-- Add your migration SQL here
"#,
        migration_name,
        Utc::now().to_rfc3339()
    );
    fs::write(migration_dir.join("up.sql"), up_content)?;

    // Create down.sql
    let down_content = format!(
        r#"-- Rollback migration: {}

-- Add your rollback SQL here
"#,
        migration_name
    );
    fs::write(migration_dir.join("down.sql"), down_content)?;

    print_success(&format!("Migration created: migrations/{}", dir_name));
    println!();
    println!("Next steps:");
    println!("  1. Edit migrations/{}/up.sql", dir_name);
    println!("  2. Edit migrations/{}/down.sql", dir_name);
    println!("  3. Run: axum db migrate");

    Ok(())
}
