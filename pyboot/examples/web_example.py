"""
PyBoot Examples - Web Framework Utilities

Demonstrates web routing and middleware.
"""

import asyncio
from dev.engineeringlabs.pyboot.web import (
    Router,
    CORSMiddleware,
    Request,
    Response,
    HTTPStatus,
    route,
    get,
    post,
)


async def main():
    # Example 1: Basic Router
    print("=" * 50)
    print("Example 1: Basic Router")
    print("=" * 50)
    
    router = Router()
    
    @router.get("/")
    async def home(request: Request) -> Response:
        return Response(
            status=HTTPStatus.OK,
            json_body={"message": "Welcome home!"},
        )
    
    @router.get("/users")
    async def list_users(request: Request) -> Response:
        return Response(
            status=HTTPStatus.OK,
            json_body={"users": ["alice", "bob", "charlie"]},
        )
    
    @router.post("/users")
    async def create_user(request: Request) -> Response:
        return Response(
            status=HTTPStatus.CREATED,
            json_body={"created": True},
        )
    
    # Simulate requests
    request = Request(method="GET", path="/")
    response = await router.handle(request)
    print(f"GET / -> {response.status.name}: {response.json_body}")
    
    request = Request(method="GET", path="/users")
    response = await router.handle(request)
    print(f"GET /users -> {response.status.name}: {response.json_body}")
    
    request = Request(method="POST", path="/users")
    response = await router.handle(request)
    print(f"POST /users -> {response.status.name}: {response.json_body}")
    
    request = Request(method="GET", path="/unknown")
    response = await router.handle(request)
    print(f"GET /unknown -> {response.status.name}")
    print()

    # Example 2: CORS Middleware
    print("=" * 50)
    print("Example 2: CORS Middleware")
    print("=" * 50)
    
    from dev.engineeringlabs.pyboot.web.core import CORSConfig
    
    cors_config = CORSConfig(
        allow_origins=["https://example.com"],
        allow_methods=["GET", "POST", "PUT"],
        allow_headers=["Content-Type", "Authorization"],
    )
    cors = CORSMiddleware(config=cors_config)
    
    # Handle preflight request
    preflight_request = Request(method="OPTIONS", path="/api/data")
    preflight_response = cors.process_request(preflight_request)
    if preflight_response:
        print("Preflight response headers:")
        for key, value in preflight_response.headers.items():
            print(f"  {key}: {value}")
    
    # Add CORS headers to normal response
    response = Response(status=HTTPStatus.OK, json_body={"data": "value"})
    response = cors.process_response(response)
    print(f"\nNormal response CORS header: {response.headers.get('Access-Control-Allow-Origin')}")
    print()

    # Example 3: Route decorators
    print("=" * 50)
    print("Example 3: Route Decorators")
    print("=" * 50)
    
    @get("/api/items")
    async def get_items(request: Request) -> Response:
        return Response(json_body={"items": []})
    
    @post("/api/items")
    async def create_item(request: Request) -> Response:
        return Response(status=HTTPStatus.CREATED)
    
    # Check route metadata
    print(f"get_items route: {get_items._route.method} {get_items._route.path}")
    print(f"create_item route: {create_item._route.method} {create_item._route.path}")
    print()

    # Example 4: Request with data
    print("=" * 50)
    print("Example 4: Request with Data")
    print("=" * 50)
    
    request = Request(
        method="POST",
        path="/api/users",
        headers={
            "Content-Type": "application/json",
            "Authorization": "Bearer token123",
        },
        query={"page": "1", "limit": "10"},
        json_body={"name": "Alice", "email": "alice@example.com"},
    )
    
    print(f"Method: {request.method}")
    print(f"Path: {request.path}")
    print(f"Headers: {request.headers}")
    print(f"Query params: {request.query}")
    print(f"Body: {request.json_body}")


if __name__ == "__main__":
    asyncio.run(main())
