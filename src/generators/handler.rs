use anyhow::Result;
use std::fs;

use crate::utils::{ensure_in_project, print_info, print_success, to_snake_case};

pub async fn generate(name: &str, feature: &str) -> Result<()> {
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

    print_info(&format!(
        "Adding handler '{}' to feature '{}'",
        name, feature_name
    ));

    let handlers_file = feature_dir.join("handlers.rs");

    if !handlers_file.exists() {
        anyhow::bail!("handlers.rs not found in feature");
    }

    let existing = fs::read_to_string(&handlers_file)?;

    let handler_name = to_snake_case(name);
    let handler_code = format!(
        r#"

pub async fn {}(
    State(state): State<AppState>,
) -> Result<ApiResponse<()>, ApiError> {{
    // TODO: Implement handler
    Ok(ApiResponse::empty())
}}
"#,
        handler_name
    );

    let updated = format!("{}{}", existing.trim(), handler_code);
    fs::write(&handlers_file, updated)?;

    print_success(&format!(
        "Handler '{}' added to {}/handlers.rs",
        handler_name, feature_name
    ));

    Ok(())
}
