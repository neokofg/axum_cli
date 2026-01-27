use anyhow::{Context, Result};
use std::path::Path;

use crate::utils::{print_info, print_step, print_success, run_command};

pub async fn new_project(name: &str, template: Option<&str>) -> Result<()> {
    let project_path = Path::new(name);

    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }

    let template_url = template.unwrap_or("https://github.com/your-org/axum-template.git");

    print_info(&format!("Creating new project: {}", name));

    // Step 1: Clone template
    print_step(1, 5, "Cloning template...");
    run_command("git", &["clone", "--depth=1", template_url, name]).await?;

    // Step 2: Remove .git directory
    print_step(2, 5, "Cleaning up...");
    let git_dir = project_path.join(".git");
    if git_dir.exists() {
        std::fs::remove_dir_all(&git_dir).context("Failed to remove .git directory")?;
    }

    // Step 3: Update Cargo.toml with new project name
    print_step(3, 5, "Updating project configuration...");
    let cargo_toml_path = project_path.join("Cargo.toml");
    if cargo_toml_path.exists() {
        let content = std::fs::read_to_string(&cargo_toml_path)?;
        let updated = content
            .replace("axum_template", name)
            .replace("axum-template", name);
        std::fs::write(&cargo_toml_path, updated)?;
    }

    // Step 4: Create .env from example
    print_step(4, 5, "Setting up environment...");
    let env_example = project_path.join(".env.example");
    let env_file = project_path.join(".env");
    if env_example.exists() && !env_file.exists() {
        std::fs::copy(&env_example, &env_file)?;
    }

    // Step 5: Initialize git
    print_step(5, 5, "Initializing git repository...");
    std::env::set_current_dir(project_path)?;
    run_command("git", &["init"]).await?;

    print_success(&format!("Project '{}' created successfully!", name));
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  axum docker up -d");
    println!("  axum db migrate");
    println!("  axum dev");

    Ok(())
}
