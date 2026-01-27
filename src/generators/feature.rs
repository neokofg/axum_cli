use anyhow::Result;
use std::fs;

use crate::templates;
use crate::utils::{ensure_in_project, print_info, print_step, print_success, to_snake_case};

pub async fn generate(name: &str) -> Result<()> {
    let project_root = ensure_in_project()?;
    let feature_name = to_snake_case(name);
    let features_dir = project_root.join("src/features");
    let feature_dir = features_dir.join(&feature_name);

    if feature_dir.exists() {
        anyhow::bail!("Feature '{}' already exists", feature_name);
    }

    print_info(&format!("Generating feature: {}", feature_name));

    // Create feature directory
    fs::create_dir_all(&feature_dir)?;

    let tera = templates::create_engine()?;
    let mut ctx = templates::create_context(&feature_name);

    // Default empty fields (can be customized later)
    ctx.insert("fields", &Vec::<super::FieldInfo>::new());

    // Generate files
    let files = [
        ("feature/mod.rs", "mod.rs"),
        ("feature/model.rs", "model.rs"),
        ("feature/dto.rs", "dto.rs"),
        ("feature/repository.rs", "repository.rs"),
        ("feature/service.rs", "service.rs"),
        ("feature/handlers.rs", "handlers.rs"),
        ("feature/routes.rs", "routes.rs"),
        ("feature/tests.rs", "tests.rs"),
    ];

    for (i, (template, filename)) in files.iter().enumerate() {
        print_step(
            (i + 1) as u32,
            files.len() as u32,
            &format!("Creating {}", filename),
        );

        let content = tera.render(template, &ctx)?;
        let file_path = feature_dir.join(filename);
        fs::write(&file_path, content)?;
    }

    // Update features/mod.rs
    update_features_mod(&features_dir, &feature_name)?;

    print_success(&format!("Feature '{}' created successfully!", feature_name));
    println!();
    println!("Next steps:");
    println!(
        "  1. Create a migration: axum g migration create_{}",
        feature_name
    );
    println!("  2. Update src/schema.rs after migration");
    println!("  3. Add routes to main.rs");

    Ok(())
}

fn update_features_mod(features_dir: &std::path::Path, feature_name: &str) -> Result<()> {
    let mod_file = features_dir.join("mod.rs");

    if mod_file.exists() {
        let content = fs::read_to_string(&mod_file)?;

        // Check if already included
        if content.contains(&format!("pub mod {};", feature_name)) {
            return Ok(());
        }

        // Add new module
        let new_line = format!("pub mod {};", feature_name);
        let updated = format!("{}\n{}", content.trim(), new_line);
        fs::write(&mod_file, updated)?;
    } else {
        // Create new mod.rs
        let content = format!("pub mod {};", feature_name);
        fs::write(&mod_file, content)?;
    }

    Ok(())
}
