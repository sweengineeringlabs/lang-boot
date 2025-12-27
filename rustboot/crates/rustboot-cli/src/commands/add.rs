use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// Embedded templates
const DATABASE_SETUP_TEMPLATE: &str = include_str!("../../templates/database_setup.rs.template");
const AUTH_MIDDLEWARE_TEMPLATE: &str = include_str!("../../templates/auth_middleware.rs.template");
const API_MODELS_TEMPLATE: &str = include_str!("../../templates/api_models.rs.template");

pub fn execute(feature: &str) -> Result<()> {
    println!("Adding feature: {}", feature);

    // Check if we're in a Rustboot project
    if !is_rustboot_project() {
        anyhow::bail!("Not in a Rustboot project. Run this command from the project root.");
    }

    match feature {
        "database" => add_database()?,
        "auth" => add_auth()?,
        "api" => add_api()?,
        _ => {
            anyhow::bail!(
                "Unknown feature: {}. Available features: database, auth, api",
                feature
            );
        }
    }

    println!("\nFeature '{}' added successfully!", feature);

    Ok(())
}

fn is_rustboot_project() -> bool {
    // Check if Cargo.toml exists
    if !Path::new("Cargo.toml").exists() {
        return false;
    }

    // Check if it contains rustboot dependencies
    if let Ok(cargo_toml) = fs::read_to_string("Cargo.toml") {
        cargo_toml.contains("rustboot")
    } else {
        false
    }
}

fn add_database() -> Result<()> {
    println!("  Adding database support...");

    // Update Cargo.toml
    update_cargo_toml_dependencies(&[
        "dev-engineeringlabs-rustboot-database = { version = \"0.1\", features = [\"sqlx-postgres\", \"pool-deadpool\"] }",
        "sqlx = { version = \"0.8\", features = [\"runtime-tokio\", \"postgres\"] }",
    ])?;

    // Create database module
    let src_path = Path::new("src");
    fs::create_dir_all(src_path).context("Failed to access src directory")?;

    fs::write(
        src_path.join("database.rs"),
        DATABASE_SETUP_TEMPLATE,
    )
    .context("Failed to create database.rs")?;

    // Create .env.example
    let env_example = "# Database Configuration\nDATABASE_URL=postgres://user:password@localhost/mydb\n";
    fs::write(".env.example", env_example).context("Failed to create .env.example")?;

    println!("  Created files:");
    println!("    - src/database.rs");
    println!("    - .env.example");
    println!("\n  Next steps:");
    println!("    1. Copy .env.example to .env and configure your database URL");
    println!("    2. Add 'mod database;' to your src/main.rs");
    println!("    3. Initialize database in main: let pool = database::init_database().await?;");

    Ok(())
}

fn add_auth() -> Result<()> {
    println!("  Adding authentication support...");

    // Update Cargo.toml
    update_cargo_toml_dependencies(&[
        "dev-engineeringlabs-rustboot-middleware = { version = \"0.1\" }",
        "jsonwebtoken = \"9.2\"",
        "bcrypt = \"0.15\"",
    ])?;

    // Create auth module
    let src_path = Path::new("src");
    fs::create_dir_all(src_path).context("Failed to access src directory")?;

    fs::write(
        src_path.join("auth.rs"),
        AUTH_MIDDLEWARE_TEMPLATE,
    )
    .context("Failed to create auth.rs")?;

    println!("  Created files:");
    println!("    - src/auth.rs");
    println!("\n  Next steps:");
    println!("    1. Add 'mod auth;' to your src/main.rs");
    println!("    2. Configure JWT secret in your config or environment");
    println!("    3. Use AuthMiddleware in your middleware chain");
    println!("    4. Implement proper JWT token generation and verification");

    Ok(())
}

fn add_api() -> Result<()> {
    println!("  Adding API support with OpenAPI documentation...");

    // Update Cargo.toml
    update_cargo_toml_dependencies(&[
        "dev-engineeringlabs-rustboot-openapi = \"0.1\"",
        "dev-engineeringlabs-rustboot-macros = \"0.1\"",
        "utoipa = \"4.2\"",
    ])?;

    // Create models module
    let src_path = Path::new("src");
    fs::create_dir_all(src_path).context("Failed to access src directory")?;

    fs::write(
        src_path.join("models.rs"),
        API_MODELS_TEMPLATE,
    )
    .context("Failed to create models.rs")?;

    println!("  Created files:");
    println!("    - src/models.rs");
    println!("\n  Next steps:");
    println!("    1. Add 'mod models;' to your src/main.rs");
    println!("    2. Use the models for your API endpoints");
    println!("    3. Add OpenAPI annotations to your handlers");
    println!("    4. Generate OpenAPI spec with rustboot-openapi");

    Ok(())
}

fn update_cargo_toml_dependencies(deps: &[&str]) -> Result<()> {
    let cargo_toml_path = Path::new("Cargo.toml");
    let mut cargo_toml = fs::read_to_string(cargo_toml_path)
        .context("Failed to read Cargo.toml")?;

    // Find the [dependencies] section
    if let Some(deps_pos) = cargo_toml.find("[dependencies]") {
        // Find the next section or end of file
        let after_deps = &cargo_toml[deps_pos + "[dependencies]".len()..];
        let next_section_pos = after_deps.find("\n[").unwrap_or(after_deps.len());

        let insertion_point = deps_pos + "[dependencies]".len() + next_section_pos;

        // Check if dependencies already exist
        let deps_section = &cargo_toml[deps_pos..insertion_point];
        let mut new_deps = Vec::new();

        for dep in deps {
            let dep_name = dep.split('=').next().unwrap().trim();
            if !deps_section.contains(dep_name) {
                new_deps.push(*dep);
            }
        }

        if !new_deps.is_empty() {
            let deps_text = new_deps.join("\n");
            cargo_toml.insert_str(insertion_point, &format!("\n{}\n", deps_text));

            fs::write(cargo_toml_path, cargo_toml)
                .context("Failed to write Cargo.toml")?;

            println!("  Updated Cargo.toml with new dependencies");
        } else {
            println!("  Dependencies already present in Cargo.toml");
        }
    } else {
        anyhow::bail!("Could not find [dependencies] section in Cargo.toml");
    }

    Ok(())
}
