# Rustboot CLI

A command-line tool for scaffolding Rustboot projects and adding features.

## Installation

From the rustboot workspace:

```bash
cargo build -p rustboot-cli --release
```

The binary will be available at `target/release/rustboot`.

## Usage

### Create a New Project

Create a new Rustboot project with a complete starter template:

```bash
rustboot new my-project
```

This creates:
- `Cargo.toml` - Project manifest with rustboot dependencies
- `src/main.rs` - Basic web server example
- `config.toml` - Application configuration
- `Dockerfile` - Container build configuration
- `deployment.yaml` - Kubernetes deployment manifests
- `.gitignore` - Git ignore file
- `README.md` - Project documentation

You can also specify a custom path:

```bash
rustboot new my-project --path /path/to/parent
```

### Add Features

Add features to an existing Rustboot project:

#### Database Support

```bash
rustboot add database
```

Adds:
- Database dependency with SQLx and connection pooling
- `src/database.rs` - Database setup module
- `.env.example` - Environment variable template

#### Authentication

```bash
rustboot add auth
```

Adds:
- JWT and bcrypt dependencies
- `src/auth.rs` - Authentication middleware

#### API/OpenAPI

```bash
rustboot add api
```

Adds:
- OpenAPI dependencies
- `src/models.rs` - API model definitions

## Features

### Supported Commands

- `rustboot new <name>` - Create a new project
- `rustboot add <feature>` - Add a feature to existing project

### Available Features

- `database` - PostgreSQL database with SQLx and connection pooling
- `auth` - JWT-based authentication middleware
- `api` - OpenAPI documentation support

## Template Files

All templates are embedded in the binary using `include_str!` for zero-dependency distribution. Templates support the following placeholders:

- `{{project_name}}` - Replaced with the actual project name

## Project Structure

```
rustboot-cli/
├── Cargo.toml          # Package manifest
├── src/
│   ├── main.rs         # CLI entry point
│   └── commands/
│       ├── mod.rs      # Commands module
│       ├── new.rs      # 'new' command implementation
│       └── add.rs      # 'add' command implementation
└── templates/          # Embedded templates
    ├── cargo.toml.template
    ├── main.rs.template
    ├── config.toml.template
    ├── dockerfile.template
    ├── deployment.yaml.template
    ├── gitignore.template
    ├── readme.md.template
    ├── database_setup.rs.template
    ├── auth_middleware.rs.template
    └── api_models.rs.template
```

## Examples

Create a new project with database and auth:

```bash
rustboot new my-api
cd my-api
rustboot add database
rustboot add auth
cargo build
```

## Development

To work on the CLI tool:

```bash
# Build
cargo build -p rustboot-cli

# Run tests
cargo test -p rustboot-cli

# Run CLI
cargo run -p rustboot-cli -- new test-project
```

## License

MIT
