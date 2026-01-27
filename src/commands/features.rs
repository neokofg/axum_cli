use anyhow::Result;
use colored::Colorize;
use std::fs;

use crate::utils::{ensure_in_project, print_info};

pub async fn list() -> Result<()> {
    let project_root = ensure_in_project()?;
    let features_dir = project_root.join("src/features");

    if !features_dir.exists() {
        anyhow::bail!("Features directory not found");
    }

    print_info("Project features:");
    println!();

    let mut features: Vec<_> = fs::read_dir(&features_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name()?.to_str()?.to_string();
                if !name.starts_with('_') && name != "mod.rs" {
                    return Some(name);
                }
            }
            None
        })
        .collect();

    features.sort();

    for feature in &features {
        let feature_path = features_dir.join(feature);

        let has_model = feature_path.join("model.rs").exists();
        let has_routes = feature_path.join("routes.rs").exists();
        let has_tests = feature_path.join("tests.rs").exists();

        let status = format!(
            "[{}{}{}]",
            if has_model { "M" } else { "-" },
            if has_routes { "R" } else { "-" },
            if has_tests { "T" } else { "-" }
        );

        println!("  {} {} {}", "•".cyan(), feature.bold(), status.dimmed());
    }

    println!();
    println!("{}", "Legend: M=Model, R=Routes, T=Tests".dimmed());
    println!();
    println!("Total: {} features", features.len());

    Ok(())
}
