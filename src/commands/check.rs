use anyhow::Result;

use crate::utils::{ensure_in_project, print_info, print_success, run_command_in_dir};

pub async fn run(fix: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    // Run clippy
    print_info("Running clippy...");

    let mut clippy_args = vec!["clippy", "--all-targets", "--all-features"];
    if fix {
        clippy_args.extend(["--fix", "--allow-dirty", "--allow-staged"]);
    }
    clippy_args.extend(["--", "-D", "warnings"]);

    run_command_in_dir(&project_root, "cargo", &clippy_args).await?;
    print_success("Clippy passed");

    // Run fmt check
    print_info("Checking formatting...");

    let fmt_args = if fix {
        vec!["fmt"]
    } else {
        vec!["fmt", "--check"]
    };

    run_command_in_dir(&project_root, "cargo", &fmt_args).await?;
    print_success("Formatting check passed");

    print_success("All checks passed!");

    Ok(())
}
