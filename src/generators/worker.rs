use anyhow::Result;
use std::fs;

use crate::templates;
use crate::utils::{ensure_in_project, print_info, print_success, to_snake_case};

pub async fn generate(name: &str) -> Result<()> {
    let project_root = ensure_in_project()?;
    let worker_name = to_snake_case(name);
    let workers_dir = project_root.join("src/workers");

    if !workers_dir.exists() {
        fs::create_dir_all(&workers_dir)?;
    }

    let worker_file = workers_dir.join(format!("{}_worker.rs", worker_name));

    if worker_file.exists() {
        anyhow::bail!("Worker '{}' already exists", worker_name);
    }

    print_info(&format!("Generating worker: {}", worker_name));

    let tera = templates::create_engine()?;
    let ctx = templates::create_context(&worker_name);

    let content = tera.render("worker/mod.rs", &ctx)?;
    fs::write(&worker_file, content)?;

    // Update workers/mod.rs
    update_workers_mod(&workers_dir, &worker_name)?;

    print_success(&format!("Worker '{}' created!", worker_name));
    println!();
    println!("File: src/workers/{}_worker.rs", worker_name);

    Ok(())
}

fn update_workers_mod(workers_dir: &std::path::Path, worker_name: &str) -> Result<()> {
    let mod_file = workers_dir.join("mod.rs");
    let module_name = format!("{}_worker", worker_name);

    if mod_file.exists() {
        let content = fs::read_to_string(&mod_file)?;

        if content.contains(&format!("mod {};", module_name)) {
            return Ok(());
        }

        let new_mod = format!("mod {};", module_name);
        let new_use = format!("pub use {}::*;", module_name);
        let updated = format!("{}\n{}\n{}", content.trim(), new_mod, new_use);
        fs::write(&mod_file, updated)?;
    } else {
        let content = format!("mod {};\n\npub use {}::*;", module_name, module_name);
        fs::write(&mod_file, content)?;
    }

    Ok(())
}
