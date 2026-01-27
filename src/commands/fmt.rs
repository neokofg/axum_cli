use anyhow::Result;

use crate::utils::{ensure_in_project, print_info, print_success, run_command_in_dir};

pub async fn run(check: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    if check {
        print_info("Checking code formatting...");
        run_command_in_dir(&project_root, "cargo", &["fmt", "--check"]).await?;
        print_success("Code is properly formatted");
    } else {
        print_info("Formatting code...");
        run_command_in_dir(&project_root, "cargo", &["fmt"]).await?;
        print_success("Code formatted");
    }

    Ok(())
}
