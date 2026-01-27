use anyhow::Result;
use std::fs;

use crate::templates;
use crate::utils::{ensure_in_project, print_info, print_success, to_snake_case};

pub async fn generate(name: &str) -> Result<()> {
    let project_root = ensure_in_project()?;
    let middleware_name = to_snake_case(name);
    let middleware_dir = project_root.join("src/core/middleware");

    if !middleware_dir.exists() {
        anyhow::bail!("Middleware directory not found");
    }

    let middleware_file = middleware_dir.join(format!("{}.rs", middleware_name));

    if middleware_file.exists() {
        anyhow::bail!("Middleware '{}' already exists", middleware_name);
    }

    print_info(&format!("Generating middleware: {}", middleware_name));

    let tera = templates::create_engine()?;
    let ctx = templates::create_context(&middleware_name);

    let content = tera.render("middleware/mod.rs", &ctx)?;
    fs::write(&middleware_file, content)?;

    // Update middleware/mod.rs
    update_middleware_mod(&middleware_dir, &middleware_name)?;

    print_success(&format!("Middleware '{}' created!", middleware_name));
    println!();
    println!("File: src/core/middleware/{}.rs", middleware_name);
    println!();
    println!("Usage in main.rs:");
    println!(
        "  .layer(middleware::from_fn({}_middleware))",
        middleware_name
    );

    Ok(())
}

fn update_middleware_mod(middleware_dir: &std::path::Path, name: &str) -> Result<()> {
    let mod_file = middleware_dir.join("mod.rs");

    if mod_file.exists() {
        let content = fs::read_to_string(&mod_file)?;

        if content.contains(&format!("pub mod {};", name)) {
            return Ok(());
        }

        let new_mod = format!("pub mod {};", name);
        let new_use = format!("pub use {}::{}_middleware;", name, name);
        let updated = format!("{}\n{}\n{}", content.trim(), new_mod, new_use);
        fs::write(&mod_file, updated)?;
    }

    Ok(())
}
