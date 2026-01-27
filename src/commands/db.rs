use anyhow::Result;
use dialoguer::Confirm;

use crate::utils::{
    ensure_in_project, print_info, print_success, print_warning, run_command_in_dir,
};

pub async fn migrate() -> Result<()> {
    let project_root = ensure_in_project()?;
    print_info("Running migrations...");
    run_command_in_dir(&project_root, "diesel", &["migration", "run"]).await?;
    print_success("Migrations completed");
    Ok(())
}

pub async fn rollback(step: u32) -> Result<()> {
    let project_root = ensure_in_project()?;
    print_info(&format!("Rolling back {} migration(s)...", step));

    for _ in 0..step {
        run_command_in_dir(&project_root, "diesel", &["migration", "revert"]).await?;
    }

    print_success("Rollback completed");
    Ok(())
}

pub async fn reset(force: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    if !force {
        let confirm = Confirm::new()
            .with_prompt("This will drop and recreate the database. Continue?")
            .default(false)
            .interact()?;

        if !confirm {
            print_info("Aborted");
            return Ok(());
        }
    }

    print_warning("Resetting database...");

    // Drop database
    let _ = run_command_in_dir(&project_root, "diesel", &["database", "drop"]).await;

    // Create database
    run_command_in_dir(&project_root, "diesel", &["database", "setup"]).await?;

    print_success("Database reset completed");
    Ok(())
}

pub async fn create() -> Result<()> {
    let project_root = ensure_in_project()?;
    print_info("Creating database...");
    run_command_in_dir(&project_root, "diesel", &["database", "setup"]).await?;
    print_success("Database created");
    Ok(())
}

pub async fn drop(force: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    if !force {
        let confirm = Confirm::new()
            .with_prompt("This will drop the database. Continue?")
            .default(false)
            .interact()?;

        if !confirm {
            print_info("Aborted");
            return Ok(());
        }
    }

    print_warning("Dropping database...");
    run_command_in_dir(&project_root, "diesel", &["database", "drop"]).await?;
    print_success("Database dropped");
    Ok(())
}

pub async fn status() -> Result<()> {
    let project_root = ensure_in_project()?;
    print_info("Migration status:");
    run_command_in_dir(&project_root, "diesel", &["migration", "list"]).await
}

pub async fn schema() -> Result<()> {
    let project_root = ensure_in_project()?;
    print_info("Regenerating schema.rs...");
    run_command_in_dir(&project_root, "diesel", &["print-schema"]).await?;
    print_success("Schema regenerated");
    Ok(())
}

pub async fn seed() -> Result<()> {
    let project_root = ensure_in_project()?;
    let seed_file = project_root.join("src/db/seed.rs");

    if !seed_file.exists() {
        anyhow::bail!("Seed file not found. Create src/db/seed.rs first.");
    }

    print_info("Seeding database...");
    run_command_in_dir(&project_root, "cargo", &["run", "--bin", "seed"]).await?;
    print_success("Database seeded");
    Ok(())
}
