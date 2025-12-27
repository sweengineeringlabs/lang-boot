"""OpenAPI types."""

from dataclasses import dataclass, field
from typing import Any
from enum import Enum


class ParameterIn(str, Enum):
    """Parameter location."""
    QUERY = "query"
    PATH = "path"
    HEADER = "header"
    COOKIE = "cookie"


@dataclass
class Schema:
    """JSON Schema definition."""
    type: str = "object"
    properties: dict[str, "Schema"] = field(default_factory=dict)
    required: list[str] = field(default_factory=list)
    items: "Schema | None" = None
    format: str | None = None
    description: str | None = None
    enum: list[Any] | None = None
    default: Any = None
    example: Any = None
    
    def to_dict(self) -> dict[str, Any]:
        result: dict[str, Any] = {"type": self.type}
        if self.properties:
            result["properties"] = {k: v.to_dict() for k, v in self.properties.items()}
        if self.required:
            result["required"] = self.required
        if self.items:
            result["items"] = self.items.to_dict()
        if self.format:
            result["format"] = self.format
        if self.description:
            result["description"] = self.description
        if self.enum:
            result["enum"] = self.enum
        if self.default is not None:
            result["default"] = self.default
        if self.example is not None:
            result["example"] = self.example
        return result


@dataclass
class Parameter:
    """API parameter definition."""
    name: str
    location: ParameterIn = ParameterIn.QUERY
    required: bool = False
    schema: Schema | None = None
    description: str | None = None
    example: Any = None
    
    def to_dict(self) -> dict[str, Any]:
        result: dict[str, Any] = {
            "name": self.name,
            "in": self.location.value,
            "required": self.required,
        }
        if self.schema:
            result["schema"] = self.schema.to_dict()
        if self.description:
            result["description"] = self.description
        if self.example is not None:
            result["example"] = self.example
        return result


@dataclass
class RequestBody:
    """Request body definition."""
    content_type: str = "application/json"
    schema: Schema | None = None
    required: bool = True
    description: str | None = None
    
    def to_dict(self) -> dict[str, Any]:
        content = {}
        if self.schema:
            content[self.content_type] = {"schema": self.schema.to_dict()}
        return {
            "required": self.required,
            "content": content,
            "description": self.description or "",
        }


@dataclass
class Response:
    """API response definition."""
    status: int
    description: str = ""
    schema: Schema | None = None
    content_type: str = "application/json"
    
    def to_dict(self) -> dict[str, Any]:
        result: dict[str, Any] = {"description": self.description}
        if self.schema:
            result["content"] = {
                self.content_type: {"schema": self.schema.to_dict()}
            }
        return result


@dataclass
class Operation:
    """API operation (endpoint method)."""
    method: str
    path: str
    summary: str = ""
    description: str = ""
    operation_id: str | None = None
    tags: list[str] = field(default_factory=list)
    parameters: list[Parameter] = field(default_factory=list)
    request_body: RequestBody | None = None
    responses: dict[int, Response] = field(default_factory=dict)
    deprecated: bool = False
    
    def to_dict(self) -> dict[str, Any]:
        result: dict[str, Any] = {}
        if self.summary:
            result["summary"] = self.summary
        if self.description:
            result["description"] = self.description
        if self.operation_id:
            result["operationId"] = self.operation_id
        if self.tags:
            result["tags"] = self.tags
        if self.parameters:
            result["parameters"] = [p.to_dict() for p in self.parameters]
        if self.request_body:
            result["requestBody"] = self.request_body.to_dict()
        if self.responses:
            result["responses"] = {str(k): v.to_dict() for k, v in self.responses.items()}
        if self.deprecated:
            result["deprecated"] = True
        return result


@dataclass
class PathItem:
    """API path with operations."""
    path: str
    operations: dict[str, Operation] = field(default_factory=dict)
    
    def to_dict(self) -> dict[str, Any]:
        return {op.method.lower(): op.to_dict() for op in self.operations.values()}


@dataclass
class Tag:
    """API tag for grouping."""
    name: str
    description: str = ""


@dataclass
class Info:
    """API info."""
    title: str
    version: str
    description: str = ""
    terms_of_service: str = ""
    contact: dict[str, str] = field(default_factory=dict)
    license: dict[str, str] = field(default_factory=dict)


@dataclass
class Server:
    """API server."""
    url: str
    description: str = ""


@dataclass
class OpenAPISpec:
    """Complete OpenAPI specification."""
    info: Info
    paths: dict[str, PathItem] = field(default_factory=dict)
    servers: list[Server] = field(default_factory=list)
    tags: list[Tag] = field(default_factory=list)
    openapi: str = "3.0.3"
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "openapi": self.openapi,
            "info": {
                "title": self.info.title,
                "version": self.info.version,
                "description": self.info.description,
            },
            "servers": [{"url": s.url, "description": s.description} for s in self.servers],
            "paths": {path: item.to_dict() for path, item in self.paths.items()},
            "tags": [{"name": t.name, "description": t.description} for t in self.tags],
        }
