use anyhow::Result;

use crate::utils::{ensure_in_project, print_info, run_command_in_dir};

pub async fn run(pattern: Option<&str>, unit: bool, integration: bool) -> Result<()> {
    let project_root = ensure_in_project()?;

    let mut args = vec!["test"];

    if unit && !integration {
        args.push("--lib");
        print_info("Running unit tests...");
    } else if integration && !unit {
        args.push("--test");
        args.push("*");
        print_info("Running integration tests...");
    } else {
        print_info("Running all tests...");
    }

    if let Some(p) = pattern {
        args.push(p);
    }

    // Add -- to pass additional args
    args.push("--");
    args.push("--nocapture");

    run_command_in_dir(&project_root, "cargo", &args).await
}
