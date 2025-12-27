"""
OpenAPI Module - API documentation generation.

This module provides:
- OpenAPI 3.0 spec generation
- Route documentation
- Schema generation from types
- Swagger UI integration
- ReDoc support

Example:
    from dev.engineeringlabs.pyboot.openapi import OpenAPI, route
    
    # Document routes
    @route(
        path="/users/{id}",
        method="GET",
        summary="Get user by ID",
        responses={200: User, 404: Error}
    )
    async def get_user(id: str) -> User:
        ...
    
    # Generate spec
    spec = OpenAPI(
        title="My API",
        version="1.0.0",
    )
    spec.add_route(get_user)
    
    # Export
    yaml_spec = spec.to_yaml()
    json_spec = spec.to_json()
"""

from dev.engineeringlabs.pyboot.openapi.api import (
    # Types
    OpenAPISpec,
    PathItem,
    Operation,
    Parameter,
    RequestBody,
    APIResponse,
    Schema,
    Tag,
    Info,
    Server,
    # Decorators
    route,
    schema_doc,
    parameter,
    response,
)

from dev.engineeringlabs.pyboot.openapi.core import (
    OpenAPI,
    SchemaGenerator,
    generate_spec,
)

__all__ = [
    # API
    "OpenAPISpec",
    "PathItem",
    "Operation",
    "Parameter",
    "RequestBody",
    "APIResponse",
    "Schema",
    "Tag",
    "Info",
    "Server",
    "route",
    "schema_doc",
    "parameter",
    "response",
    # Core
    "OpenAPI",
    "SchemaGenerator",
    "generate_spec",
]

