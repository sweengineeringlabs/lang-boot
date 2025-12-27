"""Tests for CLI module."""

import pytest
from dev.engineeringlabs.pyboot.cli import CLI, Command, Argument, Option, CLIResult, CLIError
from dev.engineeringlabs.pyboot.cli.core import CommandRegistry


class TestCommand:
    """Tests for Command dataclass."""
    
    def test_command_creation(self):
        """Test creating a command."""
        cmd = Command(name="test", description="A test command")
        assert cmd.name == "test"
        assert cmd.description == "A test command"
    
    def test_command_with_arguments(self):
        """Test command with arguments."""
        cmd = Command(
            name="deploy",
            arguments=[
                Argument(name="env", description="Environment"),
            ],
        )
        assert len(cmd.arguments) == 1
        assert cmd.arguments[0].name == "env"
    
    def test_command_with_options(self):
        """Test command with options."""
        cmd = Command(
            name="build",
            options=[
                Option(name="--verbose", short="-v"),
                Option(name="--output", short="-o", default="./dist"),
            ],
        )
        assert len(cmd.options) == 2


class TestArgument:
    """Tests for Argument dataclass."""
    
    def test_argument_required(self):
        """Test required argument."""
        arg = Argument(name="file", required=True)
        assert arg.required
    
    def test_argument_with_default(self):
        """Test argument with default."""
        arg = Argument(name="count", default=10, required=False)
        assert arg.default == 10


class TestOption:
    """Tests for Option dataclass."""
    
    def test_option_with_short(self):
        """Test option with short form."""
        opt = Option(name="--verbose", short="-v")
        assert opt.name == "--verbose"
        assert opt.short == "-v"
    
    def test_option_with_default(self):
        """Test option with default value."""
        opt = Option(name="--port", default=8080)
        assert opt.default == 8080


class TestCLIResult:
    """Tests for CLIResult."""
    
    def test_success_result(self):
        """Test successful result."""
        result = CLIResult(success=True, message="Done")
        assert result.success
        assert result.exit_code == 0
    
    def test_failure_result(self):
        """Test failure result."""
        result = CLIResult(success=False, message="Error", exit_code=1)
        assert not result.success
        assert result.exit_code == 1
    
    def test_result_with_data(self):
        """Test result with data."""
        result = CLIResult(success=True, data={"key": "value"})
        assert result.data == {"key": "value"}


class TestCommandRegistry:
    """Tests for CommandRegistry."""
    
    def test_register_command(self):
        """Test registering a command."""
        registry = CommandRegistry()
        cmd = Command(name="test")
        registry.register(cmd)
        
        assert registry.get("test") is cmd
    
    def test_get_nonexistent(self):
        """Test getting nonexistent command."""
        registry = CommandRegistry()
        assert registry.get("unknown") is None
    
    def test_list_all(self):
        """Test listing all commands."""
        registry = CommandRegistry()
        registry.register(Command(name="a"))
        registry.register(Command(name="b"))
        registry.register(Command(name="c"))
        
        commands = registry.list_all()
        assert len(commands) == 3


class TestCLI:
    """Tests for CLI."""
    
    def test_cli_creation(self):
        """Test creating CLI."""
        cli = CLI(name="myapp", description="My app")
        assert cli.name == "myapp"
    
    def test_command_decorator(self):
        """Test command decorator."""
        cli = CLI()
        
        @cli.command("hello")
        def hello() -> CLIResult:
            return CLIResult(success=True, message="Hello!")
        
        result = cli.run(["hello"])
        assert result.success
        assert result.message == "Hello!"
    
    def test_run_with_empty_args(self):
        """Test running with no arguments."""
        cli = CLI()
        result = cli.run([])
        
        assert not result.success
        assert "No command" in result.message
    
    def test_run_unknown_command(self):
        """Test running unknown command."""
        cli = CLI()
        result = cli.run(["unknown"])
        
        assert not result.success
        assert "Unknown command" in result.message
    
    def test_command_with_args(self):
        """Test command receiving arguments."""
        cli = CLI()
        
        @cli.command("add")
        def add(a: str = "0", b: str = "0") -> CLIResult:
            result = int(a) + int(b)
            return CLIResult(success=True, data=result)
        
        result = cli.run(["add", "5", "3"])
        assert result.success
        assert result.data == 8
