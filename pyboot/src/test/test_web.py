"""Tests for web module."""

import pytest
from dev.engineeringlabs.pyboot.web import (
    Router,
    Request,
    Response,
    Route,
    HTTPStatus,
    WebError,
    get,
    post,
    put,
    delete,
)
from dev.engineeringlabs.pyboot.web.core import CORSMiddleware, CORSConfig


class TestRequest:
    """Tests for Request dataclass."""
    
    def test_request_creation(self):
        """Test creating a request."""
        request = Request(method="GET", path="/api/users")
        assert request.method == "GET"
        assert request.path == "/api/users"
    
    def test_request_with_headers(self):
        """Test request with headers."""
        request = Request(
            method="POST",
            path="/api",
            headers={"Content-Type": "application/json"},
        )
        assert request.headers["Content-Type"] == "application/json"
    
    def test_request_with_query(self):
        """Test request with query params."""
        request = Request(
            method="GET",
            path="/search",
            query={"q": "test", "page": "1"},
        )
        assert request.query["q"] == "test"
    
    def test_request_with_body(self):
        """Test request with body."""
        request = Request(
            method="POST",
            path="/api",
            json_body={"name": "test"},
        )
        assert request.json_body["name"] == "test"


class TestResponse:
    """Tests for Response dataclass."""
    
    def test_response_defaults(self):
        """Test response default values."""
        response = Response()
        assert response.status == HTTPStatus.OK
        assert response.headers == {}
    
    def test_response_with_status(self):
        """Test response with status."""
        response = Response(status=HTTPStatus.NOT_FOUND)
        assert response.status == HTTPStatus.NOT_FOUND
    
    def test_response_with_body(self):
        """Test response with JSON body."""
        response = Response(json_body={"data": "value"})
        assert response.json_body["data"] == "value"


class TestHTTPStatus:
    """Tests for HTTPStatus enum."""
    
    def test_common_statuses(self):
        """Test common status codes."""
        assert HTTPStatus.OK == 200
        assert HTTPStatus.CREATED == 201
        assert HTTPStatus.BAD_REQUEST == 400
        assert HTTPStatus.NOT_FOUND == 404
        assert HTTPStatus.INTERNAL_ERROR == 500


class TestRouter:
    """Tests for Router."""
    
    @pytest.mark.asyncio
    async def test_add_route(self):
        """Test adding a route."""
        router = Router()
        
        async def handler(req):
            return Response(json_body={"ok": True})
        
        router.add_route("GET", "/test", handler)
        
        request = Request(method="GET", path="/test")
        response = await router.handle(request)
        
        assert response.json_body["ok"] is True
    
    @pytest.mark.asyncio
    async def test_get_decorator(self):
        """Test @router.get decorator."""
        router = Router()
        
        @router.get("/users")
        async def get_users(request):
            return Response(json_body={"users": []})
        
        request = Request(method="GET", path="/users")
        response = await router.handle(request)
        
        assert response.json_body["users"] == []
    
    @pytest.mark.asyncio
    async def test_post_decorator(self):
        """Test @router.post decorator."""
        router = Router()
        
        @router.post("/users")
        async def create_user(request):
            return Response(status=HTTPStatus.CREATED)
        
        request = Request(method="POST", path="/users")
        response = await router.handle(request)
        
        assert response.status == HTTPStatus.CREATED
    
    @pytest.mark.asyncio
    async def test_not_found(self):
        """Test unknown route returns 404."""
        router = Router()
        
        request = Request(method="GET", path="/unknown")
        response = await router.handle(request)
        
        assert response.status == HTTPStatus.NOT_FOUND
    
    def test_match_route(self):
        """Test matching a route."""
        router = Router()
        
        async def handler(req):
            return Response()
        
        router.add_route("GET", "/test", handler)
        
        route = router.match("GET", "/test")
        assert route is not None
        assert route.path == "/test"
    
    def test_match_nonexistent(self):
        """Test matching nonexistent route."""
        router = Router()
        route = router.match("GET", "/unknown")
        assert route is None


class TestRouteDecorators:
    """Tests for route decorators."""
    
    def test_get_decorator(self):
        """Test @get decorator adds route metadata."""
        @get("/api/items")
        async def handler(request):
            return Response()
        
        assert handler._route.method == "GET"
        assert handler._route.path == "/api/items"
    
    def test_post_decorator(self):
        """Test @post decorator."""
        @post("/api/items")
        async def handler(request):
            return Response()
        
        assert handler._route.method == "POST"
    
    def test_put_decorator(self):
        """Test @put decorator."""
        @put("/api/items")
        async def handler(request):
            return Response()
        
        assert handler._route.method == "PUT"
    
    def test_delete_decorator(self):
        """Test @delete decorator."""
        @delete("/api/items")
        async def handler(request):
            return Response()
        
        assert handler._route.method == "DELETE"


class TestCORSMiddleware:
    """Tests for CORSMiddleware."""
    
    def test_preflight_response(self):
        """Test OPTIONS request returns CORS headers."""
        cors = CORSMiddleware()
        request = Request(method="OPTIONS", path="/api")
        
        response = cors.process_request(request)
        
        assert response is not None
        assert response.status == HTTPStatus.NO_CONTENT
        assert "Access-Control-Allow-Origin" in response.headers
    
    def test_normal_request_passthrough(self):
        """Test non-OPTIONS requests pass through."""
        cors = CORSMiddleware()
        request = Request(method="GET", path="/api")
        
        response = cors.process_request(request)
        
        assert response is None
    
    def test_adds_cors_to_response(self):
        """Test adds CORS headers to response."""
        cors = CORSMiddleware()
        response = Response(json_body={"data": "value"})
        
        processed = cors.process_response(response)
        
        assert "Access-Control-Allow-Origin" in processed.headers
    
    def test_custom_cors_config(self):
        """Test custom CORS configuration."""
        config = CORSConfig(
            allow_origins=["https://example.com"],
            allow_methods=["GET", "POST"],
        )
        cors = CORSMiddleware(config=config)
        request = Request(method="OPTIONS", path="/api")
        
        response = cors.process_request(request)
        
        assert "https://example.com" in response.headers["Access-Control-Allow-Origin"]


class TestWebError:
    """Tests for WebError."""
    
    def test_error_with_status(self):
        """Test error includes status."""
        error = WebError("Not found", status=HTTPStatus.NOT_FOUND)
        assert error.status == HTTPStatus.NOT_FOUND
    
    def test_error_message(self):
        """Test error message."""
        error = WebError("Something went wrong")
        assert "Something went wrong" in error.message
    
    def test_error_with_cause(self):
        """Test error with cause."""
        cause = ValueError("Original")
        error = WebError("Wrapped", cause=cause)
        assert error.cause is cause
