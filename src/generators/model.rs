use anyhow::Result;
use std::fs;

use super::FieldInfo;
use crate::templates;
use crate::utils::{ensure_in_project, print_info, print_success, to_snake_case};

pub async fn generate(name: &str, feature: &str, fields: &[String]) -> Result<()> {
    let project_root = ensure_in_project()?;
    let feature_name = to_snake_case(feature);
    let feature_dir = project_root.join("src/features").join(&feature_name);

    if !feature_dir.exists() {
        anyhow::bail!(
            "Feature '{}' not found. Create it first: axum g feature {}",
            feature_name,
            feature_name
        );
    }

    let tera = templates::create_engine()?;
    let mut ctx = templates::create_context(name);

    let parsed_fields = FieldInfo::parse_many(fields);
    ctx.insert("fields", &parsed_fields);

    print_info(&format!("Generating model for feature: {}", feature_name));

    // Generate model.rs
    let model_content = tera.render("feature/model.rs", &ctx)?;
    fs::write(feature_dir.join("model.rs"), model_content)?;

    // Generate dto.rs
    let dto_content = tera.render("feature/dto.rs", &ctx)?;
    fs::write(feature_dir.join("dto.rs"), dto_content)?;

    print_success("Model and DTO generated!");
    println!();
    println!("Don't forget to:");
    println!("  1. Create migration for the table");
    println!("  2. Update src/schema.rs");

    Ok(())
}
