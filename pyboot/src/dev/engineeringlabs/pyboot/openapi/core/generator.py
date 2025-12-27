"""OpenAPI generator."""

import json
from typing import Any, Callable, get_type_hints
from dataclasses import dataclass, fields, is_dataclass

from dev.engineeringlabs.pyboot.openapi.api.types import (
    OpenAPISpec,
    Info,
    PathItem,
    Operation,
    Parameter,
    ParameterIn,
    Response,
    Schema,
    Tag,
    Server,
)


class SchemaGenerator:
    """Generates JSON Schema from Python types."""
    
    TYPE_MAPPING = {
        str: "string",
        int: "integer",
        float: "number",
        bool: "boolean",
        list: "array",
        dict: "object",
        type(None): "null",
    }
    
    def __init__(self) -> None:
        self._schemas: dict[str, Schema] = {}
    
    def generate(self, type_hint: type) -> Schema:
        """Generate schema from type."""
        if type_hint in self.TYPE_MAPPING:
            return Schema(type=self.TYPE_MAPPING[type_hint])
        
        if is_dataclass(type_hint):
            return self._from_dataclass(type_hint)
        
        # Handle Optional, List, Dict
        origin = getattr(type_hint, "__origin__", None)
        if origin is list:
            args = getattr(type_hint, "__args__", (Any,))
            return Schema(type="array", items=self.generate(args[0]))
        
        if origin is dict:
            return Schema(type="object")
        
        return Schema(type="object")
    
    def _from_dataclass(self, cls: type) -> Schema:
        """Generate schema from dataclass."""
        properties = {}
        required = []
        
        for f in fields(cls):
            prop_schema = self.generate(f.type)
            properties[f.name] = prop_schema
            
            if f.default is f.default_factory is None:
                required.append(f.name)
        
        return Schema(
            type="object",
            properties=properties,
            required=required,
        )


class OpenAPI:
    """OpenAPI specification builder.
    
    Example:
        api = OpenAPI(title="My API", version="1.0.0")
        api.add_server("https://api.example.com")
        api.add_tag("users", "User operations")
        
        # Add routes
        api.add_route(get_user)
        api.add_route(create_user)
        
        # Export
        spec = api.to_json()
    """
    
    def __init__(
        self,
        title: str,
        version: str,
        description: str = "",
    ) -> None:
        self._info = Info(title=title, version=version, description=description)
        self._paths: dict[str, PathItem] = {}
        self._servers: list[Server] = []
        self._tags: list[Tag] = []
        self._schema_gen = SchemaGenerator()
    
    def add_server(self, url: str, description: str = "") -> "OpenAPI":
        """Add server URL."""
        self._servers.append(Server(url=url, description=description))
        return self
    
    def add_tag(self, name: str, description: str = "") -> "OpenAPI":
        """Add tag for grouping."""
        self._tags.append(Tag(name=name, description=description))
        return self
    
    def add_route(self, func: Callable) -> "OpenAPI":
        """Add route from decorated function."""
        route_info = getattr(func, "_openapi_route", None)
        if not route_info:
            return self
        
        path = route_info["path"]
        method = route_info["method"].lower()
        
        operation = Operation(
            method=method,
            path=path,
            summary=route_info.get("summary", ""),
            description=route_info.get("description", ""),
            operation_id=route_info.get("operation_id"),
            tags=route_info.get("tags", []),
            deprecated=route_info.get("deprecated", False),
        )
        
        # Add parameters from decorator
        params = getattr(func, "_openapi_parameters", [])
        for p in params:
            operation.parameters.append(Parameter(
                name=p["name"],
                location=ParameterIn(p["in"]),
                required=p.get("required", False),
                description=p.get("description", ""),
            ))
        
        # Add responses from decorator
        responses = getattr(func, "_openapi_responses", {})
        for status, resp_info in responses.items():
            schema = None
            if resp_info.get("schema_type"):
                schema = self._schema_gen.generate(resp_info["schema_type"])
            operation.responses[status] = Response(
                status=status,
                description=resp_info.get("description", ""),
                schema=schema,
            )
        
        if path not in self._paths:
            self._paths[path] = PathItem(path=path)
        self._paths[path].operations[method] = operation
        
        return self
    
    def add_operation(
        self,
        path: str,
        method: str,
        summary: str = "",
        description: str = "",
        tags: list[str] | None = None,
    ) -> "OpenAPI":
        """Add operation manually."""
        operation = Operation(
            method=method.lower(),
            path=path,
            summary=summary,
            description=description,
            tags=tags or [],
        )
        
        if path not in self._paths:
            self._paths[path] = PathItem(path=path)
        self._paths[path].operations[method.lower()] = operation
        
        return self
    
    def build(self) -> OpenAPISpec:
        """Build the OpenAPI spec."""
        return OpenAPISpec(
            info=self._info,
            paths=self._paths,
            servers=self._servers,
            tags=self._tags,
        )
    
    def to_dict(self) -> dict[str, Any]:
        """Export as dictionary."""
        return self.build().to_dict()
    
    def to_json(self, indent: int = 2) -> str:
        """Export as JSON string."""
        return json.dumps(self.to_dict(), indent=indent)
    
    def to_yaml(self) -> str:
        """Export as YAML string."""
        try:
            import yaml
            return yaml.dump(self.to_dict(), default_flow_style=False)
        except ImportError:
            raise ImportError("PyYAML required for YAML export")


def generate_spec(
    title: str,
    version: str,
    routes: list[Callable],
    description: str = "",
) -> OpenAPISpec:
    """Generate OpenAPI spec from routes."""
    api = OpenAPI(title, version, description)
    for route in routes:
        api.add_route(route)
    return api.build()
