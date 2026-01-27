use anyhow::Result;
use chrono::Utc;
use std::fs;

use super::FieldInfo;
use crate::templates;
use crate::utils::{
    ensure_in_project, pluralize, print_info, print_step, print_success, to_snake_case,
};

pub async fn generate(name: &str, fields: &[String]) -> Result<()> {
    let project_root = ensure_in_project()?;
    let feature_name = to_snake_case(name);
    let table_name = pluralize(&feature_name);
    let features_dir = project_root.join("src/features");
    let feature_dir = features_dir.join(&feature_name);

    if feature_dir.exists() {
        anyhow::bail!("Feature '{}' already exists", feature_name);
    }

    let parsed_fields = FieldInfo::parse_many(fields);

    if parsed_fields.is_empty() {
        anyhow::bail!(
            "At least one field is required. Use: axum g crud {} -f name:string",
            name
        );
    }

    print_info(&format!("Generating CRUD feature: {}", feature_name));

    // Create feature directory
    fs::create_dir_all(&feature_dir)?;

    let tera = templates::create_engine()?;
    let mut ctx = templates::create_context(&feature_name);
    ctx.insert("fields", &parsed_fields);

    // Generate feature files
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
            (files.len() + 2) as u32,
            &format!("Creating {}", filename),
        );

        let content = tera.render(template, &ctx)?;
        let file_path = feature_dir.join(filename);
        fs::write(&file_path, content)?;
    }

    // Update features/mod.rs
    print_step(
        (files.len() + 1) as u32,
        (files.len() + 2) as u32,
        "Updating features/mod.rs",
    );
    update_features_mod(&features_dir, &feature_name)?;

    // Generate migration
    print_step(
        (files.len() + 2) as u32,
        (files.len() + 2) as u32,
        "Creating migration",
    );
    generate_migration(&project_root, &table_name, &parsed_fields)?;

    print_success(&format!(
        "CRUD feature '{}' created successfully!",
        feature_name
    ));
    println!();
    println!("Generated:");
    println!("  - Feature: src/features/{}/", feature_name);
    println!("  - Migration: migrations/..._create_{}/", table_name);
    println!();
    println!("Next steps:");
    println!("  1. Run migration: axum db migrate");
    println!("  2. Add to schema.rs (or run diesel print-schema)");
    println!("  3. Add routes to main.rs:");
    println!(
        "     .nest(\"/{}\", features::{}::public_router())",
        table_name, feature_name
    );
    println!(
        "     .nest(\"/{}\", features::{}::protected_router())",
        table_name, feature_name
    );

    Ok(())
}

fn update_features_mod(features_dir: &std::path::Path, feature_name: &str) -> Result<()> {
    let mod_file = features_dir.join("mod.rs");

    if mod_file.exists() {
        let content = fs::read_to_string(&mod_file)?;

        if content.contains(&format!("pub mod {};", feature_name)) {
            return Ok(());
        }

        let new_line = format!("pub mod {};", feature_name);
        let updated = format!("{}\n{}", content.trim(), new_line);
        fs::write(&mod_file, updated)?;
    } else {
        let content = format!("pub mod {};", feature_name);
        fs::write(&mod_file, content)?;
    }

    Ok(())
}

fn generate_migration(
    project_root: &std::path::Path,
    table_name: &str,
    fields: &[FieldInfo],
) -> Result<()> {
    let timestamp = Utc::now().format("%Y-%m-%d-%H%M%S");
    let dir_name = format!("{}_create_{}", timestamp, table_name);

    let migrations_dir = project_root.join("migrations");
    let migration_dir = migrations_dir.join(&dir_name);

    if !migrations_dir.exists() {
        fs::create_dir_all(&migrations_dir)?;
    }

    fs::create_dir_all(&migration_dir)?;

    // Generate up.sql
    let mut columns = vec!["    id UUID PRIMARY KEY DEFAULT uuid_generate_v4()".to_string()];

    for field in fields {
        let nullable = if field.nullable { "" } else { " NOT NULL" };
        let default = field
            .default
            .as_ref()
            .map(|d| format!(" DEFAULT {}", d))
            .unwrap_or_default();

        columns.push(format!(
            "    {} {}{}{}",
            field.name, field.sql_type, nullable, default
        ));
    }

    columns.push("    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()".to_string());
    columns.push("    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()".to_string());

    let up_content = format!(
        r#"CREATE TABLE {} (
{}
);

CREATE INDEX idx_{}_created_at ON {}(created_at);
"#,
        table_name,
        columns.join(",\n"),
        table_name,
        table_name
    );

    fs::write(migration_dir.join("up.sql"), up_content)?;

    // Generate down.sql
    let down_content = format!("DROP TABLE IF EXISTS {};", table_name);
    fs::write(migration_dir.join("down.sql"), down_content)?;

    Ok(())
}
