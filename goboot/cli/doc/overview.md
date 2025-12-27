# CLI Module Overview

> **📝 Important**: This overview links to working examples and tests.

## WHAT: Command-Line Interface Builder

The `cli` package provides utilities for building type-safe CLI applications with argument parsing, subcommands, and help generation.

Key capabilities:
- **Argument Parsing** - Type-safe flag and argument handling
- **Subcommands** - Nested command hierarchies
- **Help Generation** - Automatic help from configuration
- **Environment Binding** - Read flags from environment

## WHY: Consistent CLI Development

**Problems Solved**:
1. **Boilerplate Reduction** - Declarative command definition
2. **Type Safety** - Strongly typed flags
3. **User Experience** - Consistent help messages

**When to Use**: 
- Building CLI tools
- DevOps automation
- Interactive utilities

**When NOT to Use**: 
- GUI applications
- Web services

## HOW: Usage Guide

### Basic Application

```go
import "dev.engineeringlabs/goboot/cli"

app := cli.NewApp("myapp", "My CLI application")
cli.Version("1.0.0")(app)
cli.Author("John Doe", "john@example.com")(app)

// Define commands (conceptual API)
app.Command("greet").
    Description("Greet a user").
    Flag("name", cli.String, "Name to greet", cli.Required).
    Flag("loud", cli.Bool, "Use uppercase").
    Action(func(ctx *cli.Context) error {
        name := ctx.String("name")
        greeting := fmt.Sprintf("Hello, %s!", name)
        if ctx.Bool("loud") {
            greeting = strings.ToUpper(greeting)
        }
        fmt.Println(greeting)
        return nil
    })

app.Run(os.Args)
```

**Available**:
- `NewApp()` - Create CLI application
- `Version()` - Set version
- `Author()` - Set author info
- `Command()` - Add subcommand

**Planned**:
- Tab completion
- Shell completion scripts
- Config file binding

### Flag Types

```go
const (
    String      FlagType = iota
    Int
    Bool
    Float
    Duration
    StringSlice
)
```

### Flag Options

```go
// Required flag
cli.Required

// Hidden from help
cli.Hidden

// Bind to environment variable
cli.EnvVar("MY_APP_NAME")

// Default value
cli.Default("world")
```

## Relationship to Other Modules

| Module | Purpose | Relationship |
|--------|---------|--------------|
| config | Configuration | Load config files |
| validation | Validation | Validate inputs |
| observability | Logging | Log commands |

## Examples and Tests

### Examples

**Location**: [`examples/`](../examples/)

**Current examples**:
- [`cli_example.go`](../examples/cli_example.go) - Basic app structure

### Tests

**Location**: [`cli_test.go`](cli_test.go)

**Current tests**:
- `TestNewApp` - App creation
- `TestVersion` - Version setting
- `TestAuthor` - Author info

### Testing Guidance

```bash
go test ./cli/...
```

---

**Status**: Beta  
**Go Version**: 1.21+
