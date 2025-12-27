use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_new_project_creation() {
    let test_dir = "/tmp/rustboot-cli-test";
    let project_name = "test-integration";
    let project_path = format!("{}/{}", test_dir, project_name);

    // Clean up any existing test directory
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).expect("Failed to create test directory");

    // Run the CLI to create a new project
    let output = Command::new("cargo")
        .args(&["run", "-p", "rustboot-cli", "--", "new", project_name, "--path", test_dir])
        .output()
        .expect("Failed to execute CLI");

    assert!(output.status.success(), "CLI command failed: {:?}", output);

    // Verify the project structure
    assert!(Path::new(&project_path).exists(), "Project directory was not created");
    assert!(Path::new(&format!("{}/Cargo.toml", project_path)).exists(), "Cargo.toml not found");
    assert!(Path::new(&format!("{}/src/main.rs", project_path)).exists(), "src/main.rs not found");
    assert!(Path::new(&format!("{}/config.toml", project_path)).exists(), "config.toml not found");
    assert!(Path::new(&format!("{}/Dockerfile", project_path)).exists(), "Dockerfile not found");
    assert!(Path::new(&format!("{}/deployment.yaml", project_path)).exists(), "deployment.yaml not found");
    assert!(Path::new(&format!("{}/README.md", project_path)).exists(), "README.md not found");
    assert!(Path::new(&format!("{}/.gitignore", project_path)).exists(), ".gitignore not found");

    // Verify Cargo.toml contains the project name
    let cargo_toml = fs::read_to_string(format!("{}/Cargo.toml", project_path))
        .expect("Failed to read Cargo.toml");
    assert!(cargo_toml.contains(&format!("name = \"{}\"", project_name)), "Project name not in Cargo.toml");

    // Verify main.rs contains the project name
    let main_rs = fs::read_to_string(format!("{}/src/main.rs", project_path))
        .expect("Failed to read main.rs");
    assert!(main_rs.contains(project_name), "Project name not in main.rs");

    // Clean up
    fs::remove_dir_all(test_dir).expect("Failed to clean up test directory");
}

#[test]
fn test_add_database_feature() {
    let test_dir = "/tmp/rustboot-cli-test-db";
    let project_name = "test-db";
    let project_path = format!("{}/{}", test_dir, project_name);

    // Clean up and create project
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).expect("Failed to create test directory");

    // Create new project
    let output = Command::new("cargo")
        .args(&["run", "-p", "rustboot-cli", "--", "new", project_name, "--path", test_dir])
        .output()
        .expect("Failed to execute CLI");
    assert!(output.status.success());

    // Add database feature
    let output = Command::new("cargo")
        .args(&["run", "-p", "rustboot-cli", "--", "add", "database"])
        .current_dir(&project_path)
        .output()
        .expect("Failed to execute add database");

    assert!(output.status.success(), "Add database command failed: {:?}", output);

    // Verify database files were created
    assert!(Path::new(&format!("{}/src/database.rs", project_path)).exists(), "database.rs not found");
    assert!(Path::new(&format!("{}/.env.example", project_path)).exists(), ".env.example not found");

    // Verify Cargo.toml was updated
    let cargo_toml = fs::read_to_string(format!("{}/Cargo.toml", project_path))
        .expect("Failed to read Cargo.toml");
    assert!(cargo_toml.contains("rustboot-database"), "Database dependency not added");
    assert!(cargo_toml.contains("sqlx"), "SQLx dependency not added");

    // Clean up
    fs::remove_dir_all(test_dir).expect("Failed to clean up test directory");
}

#[test]
fn test_invalid_project_name() {
    let output = Command::new("cargo")
        .args(&["run", "-p", "rustboot-cli", "--", "new", "Invalid-Name-With-Caps"])
        .output()
        .expect("Failed to execute CLI");

    assert!(!output.status.success(), "Should reject invalid project name");
}
