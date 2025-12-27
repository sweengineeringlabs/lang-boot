use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// Embedded templates
const CARGO_TOML_TEMPLATE: &str = include_str!("../../templates/cargo.toml.template");
const MAIN_RS_TEMPLATE: &str = include_str!("../../templates/main.rs.template");
const CONFIG_TOML_TEMPLATE: &str = include_str!("../../templates/config.toml.template");
const DOCKERFILE_TEMPLATE: &str = include_str!("../../templates/dockerfile.template");
const DEPLOYMENT_YAML_TEMPLATE: &str = include_str!("../../templates/deployment.yaml.template");
const GITIGNORE_TEMPLATE: &str = include_str!("../../templates/gitignore.template");
const README_TEMPLATE: &str = include_str!("../../templates/readme.md.template");

pub fn execute(name: &str, path: Option<&str>) -> Result<()> {
    println!("Creating new Rustboot project: {}", name);

    // Validate project name
    if !is_valid_project_name(name) {
        anyhow::bail!("Invalid project name. Must be a valid Rust package name (lowercase, alphanumeric, hyphens, underscores)");
    }

    // Determine project path
    let base_path = path.unwrap_or(".");
    let project_path = Path::new(base_path).join(name);

    // Check if directory already exists
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_path.display());
    }

    // Create project structure
    create_project_structure(&project_path, name)?;

    println!("\nProject '{}' created successfully!", name);
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  cargo build");
    println!("  cargo run");
    println!("\nTo add features, run:");
    println!("  rustboot add <feature>");
    println!("\nAvailable features: database, auth, api");

    Ok(())
}

fn is_valid_project_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        && !name.starts_with('-')
        && !name.starts_with('_')
}

fn create_project_structure(project_path: &Path, name: &str) -> Result<()> {
    // Create directories
    fs::create_dir_all(project_path).context("Failed to create project directory")?;

    let src_path = project_path.join("src");
    fs::create_dir_all(&src_path).context("Failed to create src directory")?;

    // Create files with template substitution
    let replacements = vec![("{{project_name}}", name)];

    // Cargo.toml
    create_file_from_template(
        &project_path.join("Cargo.toml"),
        CARGO_TOML_TEMPLATE,
        &replacements,
    )?;

    // src/main.rs
    create_file_from_template(
        &src_path.join("main.rs"),
        MAIN_RS_TEMPLATE,
        &replacements,
    )?;

    // config.toml
    create_file_from_template(
        &project_path.join("config.toml"),
        CONFIG_TOML_TEMPLATE,
        &replacements,
    )?;

    // Dockerfile
    create_file_from_template(
        &project_path.join("Dockerfile"),
        DOCKERFILE_TEMPLATE,
        &replacements,
    )?;

    // deployment.yaml
    create_file_from_template(
        &project_path.join("deployment.yaml"),
        DEPLOYMENT_YAML_TEMPLATE,
        &replacements,
    )?;

    // .gitignore
    create_file_from_template(
        &project_path.join(".gitignore"),
        GITIGNORE_TEMPLATE,
        &replacements,
    )?;

    // README.md
    create_file_from_template(
        &project_path.join("README.md"),
        README_TEMPLATE,
        &replacements,
    )?;

    println!("  Created project structure:");
    println!("    - Cargo.toml");
    println!("    - src/main.rs");
    println!("    - config.toml");
    println!("    - Dockerfile");
    println!("    - deployment.yaml");
    println!("    - .gitignore");
    println!("    - README.md");

    Ok(())
}

fn create_file_from_template(
    path: &Path,
    template: &str,
    replacements: &[(&str, &str)],
) -> Result<()> {
    let mut content = template.to_string();

    for (placeholder, value) in replacements {
        content = content.replace(placeholder, value);
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    Ok(())
}
