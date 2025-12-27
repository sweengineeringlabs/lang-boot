# Rustboot CLI Development Guide

## Architecture Overview

The Rustboot CLI is a code generation tool built with Clap for command-line parsing. It uses embedded templates to scaffold new projects and add features to existing ones.

### Project Structure

```
rustboot-cli/
├── Cargo.toml                  # Package manifest
├── README.md                   # User documentation
├── USAGE_EXAMPLES.md          # Usage examples
├── DEVELOPMENT.md             # This file
├── src/
│   ├── main.rs                # CLI entry point and command routing
│   └── commands/
│       ├── mod.rs             # Module exports
│       ├── new.rs             # 'new' command implementation
│       └── add.rs             # 'add' command implementation
├── templates/                  # Embedded template files
│   ├── cargo.toml.template
│   ├── main.rs.template
│   ├── config.toml.template
│   ├── dockerfile.template
│   ├── deployment.yaml.template
│   ├── gitignore.template
│   ├── readme.md.template
│   ├── database_setup.rs.template
│   ├── auth_middleware.rs.template
│   └── api_models.rs.template
└── tests/
    └── integration_test.rs    # Integration tests
```

## Key Components

### 1. Main Entry Point (`src/main.rs`)

Uses Clap's derive macros to define the CLI structure:

- Defines the `Cli` struct with command-line options
- Routes commands to appropriate handlers
- Handles error reporting

### 2. New Command (`src/commands/new.rs`)

Responsibilities:
- Validates project names (lowercase, alphanumeric, hyphens, underscores)
- Creates project directory structure
- Generates files from templates with placeholder substitution
- Provides user feedback and next steps

Template substitution:
- `{{project_name}}` → actual project name

### 3. Add Command (`src/commands/add.rs`)

Responsibilities:
- Verifies current directory is a Rustboot project
- Adds dependencies to Cargo.toml
- Creates feature-specific source files
- Provides integration instructions

Supported features:
- `database` - PostgreSQL with SQLx and connection pooling
- `auth` - JWT authentication middleware
- `api` - OpenAPI documentation support

### 4. Templates

All templates are embedded using `include_str!` macro:
- Zero runtime dependencies
- Fast template loading
- Single binary distribution

## Adding New Features

### 1. Create a New Template

Create a file in `templates/` directory:

```rust
// templates/my_feature.rs.template
pub struct MyFeature {
    name: String,
}

impl MyFeature {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
```

### 2. Update `add.rs`

Add the template constant:

```rust
const MY_FEATURE_TEMPLATE: &str = include_str!("../../templates/my_feature.rs.template");
```

Add a handler function:

```rust
fn add_my_feature() -> Result<()> {
    println!("  Adding my feature...");

    // Update Cargo.toml
    update_cargo_toml_dependencies(&[
        "my-dependency = \"1.0\"",
    ])?;

    // Create source file
    let src_path = Path::new("src");
    fs::write(
        src_path.join("my_feature.rs"),
        MY_FEATURE_TEMPLATE,
    )?;

    println!("  Created files:");
    println!("    - src/my_feature.rs");

    Ok(())
}
```

Add to the match statement:

```rust
match feature {
    "database" => add_database()?,
    "auth" => add_auth()?,
    "api" => add_api()?,
    "my-feature" => add_my_feature()?,  // Add this
    _ => { /* ... */ }
}
```

### 3. Update Documentation

Update help text in `src/main.rs`:

```rust
Add {
    /// Feature to add (database, auth, api, my-feature)
    feature: String,
},
```

## Template System

### Placeholder Substitution

The template system supports simple placeholder substitution:

```rust
fn create_file_from_template(
    path: &Path,
    template: &str,
    replacements: &[(&str, &str)],
) -> Result<()> {
    let mut content = template.to_string();
    for (placeholder, value) in replacements {
        content = content.replace(placeholder, value);
    }
    fs::write(path, content)?;
    Ok(())
}
```

Supported placeholders:
- `{{project_name}}` - The name of the project

### Adding New Placeholders

1. Add to the replacements vec in `new.rs`:

```rust
let replacements = vec![
    ("{{project_name}}", name),
    ("{{author}}", author),  // New placeholder
];
```

2. Use in templates:

```toml
[package]
name = "{{project_name}}"
authors = ["{{author}}"]
```

## Testing

### Unit Tests

Run unit tests:

```bash
cargo test -p rustboot-cli
```

### Integration Tests

Integration tests create real projects in `/tmp`:

```bash
cargo test -p rustboot-cli --test integration_test
```

Tests verify:
- Project structure creation
- File content
- Dependency injection
- Error handling

### Manual Testing

Create a test project:

```bash
# From rustboot workspace root
cargo run -p rustboot-cli -- new test-project
cd test-project
cargo build
```

Add features:

```bash
cargo run --manifest-path ../Cargo.toml -p rustboot-cli -- add database
cargo run --manifest-path ../Cargo.toml -p rustboot-cli -- add auth
cargo run --manifest-path ../Cargo.toml -p rustboot-cli -- add api
```

## Building and Distribution

### Development Build

```bash
cargo build -p rustboot-cli
# Binary at: target/debug/rustboot
```

### Release Build

```bash
cargo build -p rustboot-cli --release
# Binary at: target/release/rustboot
```

### Installation

```bash
# Install to cargo bin directory
cargo install --path crates/rustboot-cli

# Or copy binary manually
cp target/release/rustboot /usr/local/bin/
```

## Code Style

### Error Handling

Use `anyhow::Result` for functions that can fail:

```rust
pub fn execute(name: &str) -> Result<()> {
    // Use ? for error propagation
    create_file(path)?;
    Ok(())
}
```

Use `anyhow::bail!` for explicit errors:

```rust
if !is_valid {
    anyhow::bail!("Invalid input: {}", input);
}
```

### User Feedback

Provide clear, actionable feedback:

```rust
println!("Creating new Rustboot project: {}", name);
println!("  Created files:");
println!("    - Cargo.toml");
println!("\nNext steps:");
println!("  cd {}", name);
println!("  cargo build");
```

### File Operations

Always use `Path` and provide context:

```rust
fs::write(path, content)
    .with_context(|| format!("Failed to write file: {}", path.display()))?;
```

## Dependencies

Core dependencies:
- `clap` - Command-line parsing with derive macros
- `anyhow` - Error handling
- `thiserror` - Error type derivation
- `serde` / `serde_json` - Serialization (future use)
- `toml` - TOML parsing (future use)

## Future Improvements

### Potential Features

1. **Interactive Mode**: Prompt user for options
   ```bash
   rustboot new --interactive
   ```

2. **Custom Templates**: Support user-defined templates
   ```bash
   rustboot new myapp --template /path/to/template
   ```

3. **Configuration File**: Store user preferences
   ```toml
   # ~/.rustboot/config.toml
   [defaults]
   author = "Your Name"
   license = "MIT"
   ```

4. **More Features**: Add support for:
   - GraphQL
   - gRPC
   - WebSocket
   - Background jobs
   - Caching

5. **Project Update**: Update existing projects
   ```bash
   rustboot update
   ```

6. **Dependency Management**: Smart dependency resolution
   - Detect version conflicts
   - Suggest compatible versions

### Template Improvements

1. **Conditional Generation**: Generate code based on options
2. **Template Variables**: More sophisticated substitution
3. **Template Includes**: Compose templates from parts
4. **Template Validation**: Verify templates before use

## Contributing

To contribute to the CLI:

1. Follow Rust conventions and clippy suggestions
2. Add tests for new features
3. Update documentation
4. Keep templates simple and well-commented
5. Provide clear user feedback

## Troubleshooting

### Template Not Found

If you see "No such file or directory" for a template:
- Ensure the template file exists in `templates/`
- Check the `include_str!` path is correct
- Rebuild: `cargo clean && cargo build`

### Cargo.toml Parsing Issues

If dependency injection fails:
- Verify Cargo.toml has a `[dependencies]` section
- Check for syntax errors in generated Cargo.toml

### Binary Size

The release binary is ~1.2MB due to:
- Clap dependency
- Embedded templates (minimal)
- Rust standard library

To reduce size:
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

## Resources

- [Clap Documentation](https://docs.rs/clap)
- [Anyhow Documentation](https://docs.rs/anyhow)
- [Rustboot Framework](https://github.com/phdsystems/rustboot)
