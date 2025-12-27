"""OpenAPI decorators for documenting routes."""

from typing import Any, Callable, TypeVar
from functools import wraps

T = TypeVar("T")


def route(
    path: str,
    method: str = "GET",
    *,
    summary: str = "",
    description: str = "",
    tags: list[str] | None = None,
    operation_id: str | None = None,
    deprecated: bool = False,
    responses: dict[int, Any] | None = None,
) -> Callable[[Callable[..., T]], Callable[..., T]]:
    """Document an API route.
    
    Example:
        @route(
            path="/users/{id}",
            method="GET",
            summary="Get user by ID",
            tags=["users"],
            responses={200: User, 404: Error}
        )
        async def get_user(id: str) -> User:
            ...
    """
    def decorator(func: Callable[..., T]) -> Callable[..., T]:
        func._openapi_route = {
            "path": path,
            "method": method,
            "summary": summary,
            "description": description,
            "tags": tags or [],
            "operation_id": operation_id or func.__name__,
            "deprecated": deprecated,
            "responses": responses or {},
        }
        return func
    return decorator


def schema_doc(
    type_class: type,
    *,
    description: str = "",
    examples: list[Any] | None = None,
) -> Callable[[type[T]], type[T]]:
    """Document a schema/model class.
    
    Example:
        @schema_doc(description="User account", examples=[{"name": "Alice"}])
        class User:
            name: str
            email: str
    """
    def decorator(cls: type[T]) -> type[T]:
        cls._openapi_schema = {
            "type_class": type_class,
            "description": description,
            "examples": examples or [],
        }
        return cls
    return decorator


def parameter(
    name: str,
    location: str = "query",
    *,
    required: bool = False,
    description: str = "",
    schema_type: str = "string",
    example: Any = None,
) -> Callable[[Callable[..., T]], Callable[..., T]]:
    """Document a parameter.
    
    Example:
        @parameter("id", "path", required=True, description="User ID")
        @parameter("include", "query", description="Fields to include")
        async def get_user(id: str, include: str = "") -> User:
            ...
    """
    def decorator(func: Callable[..., T]) -> Callable[..., T]:
        if not hasattr(func, "_openapi_parameters"):
            func._openapi_parameters = []
        func._openapi_parameters.append({
            "name": name,
            "in": location,
            "required": required,
            "description": description,
            "schema": {"type": schema_type},
            "example": example,
        })
        return func
    return decorator


def response(
    status: int,
    description: str = "",
    *,
    schema_type: type | None = None,
    content_type: str = "application/json",
) -> Callable[[Callable[..., T]], Callable[..., T]]:
    """Document a response.
    
    Example:
        @response(200, "Successful response", schema_type=User)
        @response(404, "User not found")
        async def get_user(id: str) -> User:
            ...
    """
    def decorator(func: Callable[..., T]) -> Callable[..., T]:
        if not hasattr(func, "_openapi_responses"):
            func._openapi_responses = {}
        func._openapi_responses[status] = {
            "description": description,
            "schema_type": schema_type,
            "content_type": content_type,
        }
        return func
    return decorator
