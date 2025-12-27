"""CLI Core - CLI and command registry implementations."""

from typing import Callable, Any
from dev.engineeringlabs.pyboot.cli.api import Command, Argument, Option, CLIResult, CLIError


class CommandRegistry:
    """Registry for CLI commands."""
    
    def __init__(self) -> None:
        self._commands: dict[str, Command] = {}
    
    def register(self, command: Command) -> None:
        """Register a command."""
        self._commands[command.name] = command
    
    def get(self, name: str) -> Command | None:
        """Get a command by name."""
        return self._commands.get(name)
    
    def list_all(self) -> list[Command]:
        """List all commands."""
        return list(self._commands.values())


class CLI:
    """Command-line interface."""
    
    def __init__(self, name: str = "cli", description: str = "") -> None:
        self.name = name
        self.description = description
        self._registry = CommandRegistry()
    
    def command(
        self,
        name: str,
        description: str = "",
    ) -> Callable[[Callable[..., CLIResult]], Command]:
        """Decorator to register a command."""
        def decorator(func: Callable[..., CLIResult]) -> Command:
            cmd = Command(
                name=name,
                description=description,
                handler=func,
            )
            self._registry.register(cmd)
            return cmd
        return decorator
    
    def run(self, args: list[str]) -> CLIResult:
        """Run the CLI with given arguments."""
        if not args:
            return CLIResult(
                success=False,
                message="No command specified",
                exit_code=1,
            )
        
        cmd_name = args[0]
        cmd = self._registry.get(cmd_name)
        
        if not cmd:
            return CLIResult(
                success=False,
                message=f"Unknown command: {cmd_name}",
                exit_code=1,
            )
        
        if cmd.handler:
            return cmd.handler(*args[1:])
        
        return CLIResult(success=True)


__all__ = [
    "CLI",
    "CommandRegistry",
]
