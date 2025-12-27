# CLI Module Overview

> **📝 Important**: This overview links to working examples and tests.

## WHAT: Command-Line Interface Builder

The `jboot-cli` module provides utilities for building type-safe CLI applications with argument parsing, subcommands, help generation, and interactive prompts.

Key capabilities:
- **Argument Parsing** - Type-safe flag and argument handling
- **Subcommands** - Nested command hierarchies
- **Help Generation** - Automatic help text from configuration
- **Interactive Prompts** - User input, password prompts, confirmations

## WHY: Consistent CLI Development

**Problems Solved**:
1. **Boilerplate Reduction** - Declarative command definition
2. **Type Safety** - Strongly typed flags and arguments
3. **User Experience** - Consistent help and error messages

**When to Use**: 
- Building CLI tools
- DevOps automation scripts
- Interactive utilities

**When NOT to Use**: 
- GUI applications
- Web services

## HOW: Usage Guide

### Basic Application

```java
import com.jboot.cli.Cli;

var app = Cli.app("myapp")
    .version("1.0.0")
    .description("My CLI application");

app.command("greet")
    .description("Greet a user")
    .option("--name", String.class, "Name to greet")
    .flag("--loud", "Use uppercase")
    .action(ctx -> {
        String name = ctx.get("name", String.class);
        boolean loud = ctx.flag("loud");
        String greeting = "Hello, " + name + "!";
        System.out.println(loud ? greeting.toUpperCase() : greeting);
    });

app.run(args);
```

**Available**:
- `app()` - Create CLI application
- `command()` - Add subcommand
- `option()` - Add typed option
- `flag()` - Add boolean flag
- `action()` - Set command handler

**Planned**:
- Tab completion generation
- Shell script generation
- Configuration file support

### Interactive Prompts

```java
// Text prompt
String name = Cli.prompt("Enter your name: ");

// Password (hidden)
String password = Cli.promptPassword("Password: ");

// Confirmation
boolean confirmed = Cli.confirm("Are you sure?");
```

### Progress Indicators

```java
// Progress bar
var progress = Cli.progressBar("Downloading", 100);
for (int i = 0; i <= 100; i++) {
    progress.update(i);
}
progress.complete();

// Spinner for indeterminate operations
var spinner = Cli.spinner("Loading...");
spinner.start();
// ... work ...
spinner.stop();
```

## Relationship to Other Modules

| Module | Purpose | Relationship |
|--------|---------|--------------|
| jboot-config | Configuration | Load config files |
| jboot-validation | Validation | Validate inputs |
| jboot-observability | Logging | Log commands |

## Examples and Tests

### Examples

**Location**: [`examples/`](../../examples/) directory

**Current examples**:
- See `CliExample.java` in examples README

### Tests

**Location**: [`src/test/java/com/jboot/cli/`](../src/test/java/com/jboot/cli/)

**Current tests**:
- `CliTest.java` - App creation, configuration tests

### Testing Guidance

```bash
mvn test -pl modules/jboot-cli
```

---

**Status**: Beta  
**Java Version**: 17+
