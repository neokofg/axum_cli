use anyhow::Result;

use crate::utils::{ensure_in_project, print_info, print_success, run_command_in_dir};

pub async fn up(detach: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    print_info("Starting Docker containers...");

    let mut args = vec!["compose", "up"];
    if detach {
        args.push("-d");
    }

    run_command_in_dir(&project_root, "docker", &args).await?;

    if detach {
        print_success("Containers started in background");
        println!();
        println!("Services:");
        println!("  PostgreSQL: localhost:5432");
        println!("  Redis:      localhost:6379");
    }

    Ok(())
}

pub async fn down(volumes: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    print_info("Stopping Docker containers...");

    let mut args = vec!["compose", "down"];
    if volumes {
        args.push("-v");
    }

    run_command_in_dir(&project_root, "docker", &args).await?;
    print_success("Containers stopped");

    Ok(())
}

pub async fn status() -> Result<()> {
    let project_root = ensure_in_project()?;

    print_info("Container status:");
    run_command_in_dir(&project_root, "docker", &["compose", "ps"]).await
}

pub async fn logs(service: Option<&str>, follow: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    let mut args = vec!["compose", "logs"];

    if follow {
        args.push("-f");
    }

    if let Some(svc) = service {
        args.push(svc);
    }

    run_command_in_dir(&project_root, "docker", &args).await
}

pub async fn restart() -> Result<()> {
    let project_root = ensure_in_project()?;

    print_info("Restarting Docker containers...");
    run_command_in_dir(&project_root, "docker", &["compose", "restart"]).await?;
    print_success("Containers restarted");

    Ok(())
}
