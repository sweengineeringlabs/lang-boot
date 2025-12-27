"""
Template engine - Simple template rendering for notifications.
"""

import re
from typing import Any


class TemplateEngine:
    """Simple template engine using {{ variable }} syntax.
    
    Example:
        engine = TemplateEngine()
        result = engine.render(
            "Hello {{ name }}, your code is {{ code }}.",
            {"name": "Alice", "code": "123456"}
        )
        # "Hello Alice, your code is 123456."
    """
    
    VARIABLE_PATTERN = re.compile(r"\{\{\s*(\w+(?:\.\w+)*)\s*\}\}")
    
    def render(self, template: str, context: dict[str, Any]) -> str:
        """Render a template with context.
        
        Args:
            template: Template string with {{ variable }} placeholders.
            context: Dictionary of values.
            
        Returns:
            Rendered string.
        """
        def replace(match: re.Match) -> str:
            key = match.group(1)
            return str(self._get_nested(context, key))
        
        return self.VARIABLE_PATTERN.sub(replace, template)
    
    def _get_nested(self, data: dict[str, Any], key: str) -> Any:
        """Get nested value using dot notation."""
        parts = key.split(".")
        value = data
        
        for part in parts:
            if isinstance(value, dict):
                value = value.get(part, f"{{{{ {key} }}}}")
            else:
                return f"{{{{ {key} }}}}"
        
        return value
    
    def validate(self, template: str) -> list[str]:
        """Extract variable names from template."""
        return self.VARIABLE_PATTERN.findall(template)


# Global engine instance
_engine = TemplateEngine()


def render_template(template: str | None, context: dict[str, Any]) -> str:
    """Render a template string with context.
    
    Args:
        template: Template string or None.
        context: Context dictionary.
        
    Returns:
        Rendered string or empty string if template is None.
    """
    if template is None:
        return ""
    return _engine.render(template, context)


def get_template_variables(template: str) -> list[str]:
    """Extract variable names from a template."""
    return _engine.validate(template)
