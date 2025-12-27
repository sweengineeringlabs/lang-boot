"""
PyBoot Examples - CLI Framework

Demonstrates command-line interface utilities.
"""

from dev.engineeringlabs.pyboot.cli import (
    CLI,
    Command,
    Argument,
    Option,
    CLIResult,
)


# Example 1: Basic CLI
print("=" * 50)
print("Example 1: Basic CLI")
print("=" * 50)

cli = CLI(name="myapp", description="My awesome application")


@cli.command("hello", description="Say hello")
def hello_cmd(name: str = "World") -> CLIResult:
    return CLIResult(success=True, message=f"Hello, {name}!")


@cli.command("add", description="Add two numbers")
def add_cmd(a: str = "0", b: str = "0") -> CLIResult:
    result = int(a) + int(b)
    return CLIResult(success=True, message=f"{a} + {b} = {result}", data=result)


# Simulate running commands
result = cli.run(["hello"])
print(f"hello: {result.message}")

result = cli.run(["add", "5", "3"])
print(f"add 5 3: {result.message}")
print()


# Example 2: Command Registry
print("=" * 50)
print("Example 2: Command Registry")
print("=" * 50)

from dev.engineeringlabs.pyboot.cli.core import CommandRegistry

registry = CommandRegistry()

cmd1 = Command(name="init", description="Initialize project")
cmd2 = Command(name="build", description="Build project")
cmd3 = Command(name="test", description="Run tests")

registry.register(cmd1)
registry.register(cmd2)
registry.register(cmd3)

print("Available commands:")
for cmd in registry.list_all():
    print(f"  {cmd.name}: {cmd.description}")
print()


# Example 3: Command with Arguments and Options
print("=" * 50)
print("Example 3: Command Definition")
print("=" * 50)

deploy_cmd = Command(
    name="deploy",
    description="Deploy application to environment",
    arguments=[
        Argument(name="environment", description="Target environment", required=True),
    ],
    options=[
        Option(name="--force", short="-f", description="Force deployment"),
        Option(name="--version", short="-v", description="Version to deploy", default="latest"),
    ],
)

print(f"Command: {deploy_cmd.name}")
print(f"Description: {deploy_cmd.description}")
print("Arguments:")
for arg in deploy_cmd.arguments:
    print(f"  {arg.name}: {arg.description} (required={arg.required})")
print("Options:")
for opt in deploy_cmd.options:
    print(f"  {opt.name} ({opt.short}): {opt.description}")
print()


# Example 4: Error Handling
print("=" * 50)
print("Example 4: Error Handling")
print("=" * 50)

result = cli.run([])  # No command
print(f"Empty args: success={result.success}, message='{result.message}'")

result = cli.run(["unknown"])  # Unknown command
print(f"Unknown cmd: success={result.success}, message='{result.message}'")
