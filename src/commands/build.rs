use anyhow::Result;

use crate::utils::{ensure_in_project, print_info, print_success, run_command_in_dir};

pub async fn run(release: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    let mode = if release { "release" } else { "debug" };
    print_info(&format!("Building project in {} mode...", mode));

    let mut args = vec!["build"];
    if release {
        args.push("--release");
    }

    run_command_in_dir(&project_root, "cargo", &args).await?;
    print_success(&format!("Build completed ({})", mode));

    Ok(())
}
