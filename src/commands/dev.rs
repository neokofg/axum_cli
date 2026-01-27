use anyhow::Result;

use crate::utils::{ensure_in_project, print_info, run_command_in_dir};

pub async fn run(port: u16) -> Result<()> {
    let project_root = ensure_in_project()?;

    print_info(&format!("Starting development server on port {}...", port));
    print_info("Press Ctrl+C to stop");

    // Set environment variable for port
    // SAFETY: We're setting env var before spawning any threads
    unsafe {
        std::env::set_var("APP_PORT", port.to_string());
    }

    // Use cargo-watch if available, otherwise just cargo run
    let cargo_watch_available = which::which("cargo-watch").is_ok();

    if cargo_watch_available {
        run_command_in_dir(&project_root, "cargo", &["watch", "-x", "run", "-w", "src"]).await
    } else {
        print_info("Tip: Install cargo-watch for auto-reload: cargo install cargo-watch");
        run_command_in_dir(&project_root, "cargo", &["run"]).await
    }
}
