"""
Web Module - Web framework utilities.

Provides web framework utilities:
- Request/Response types
- Routing utilities
- CORS handling
"""

from dev.engineeringlabs.pyboot.web.api import (
    Request,
    Response,
    Route,
    WebError,
    HTTPStatus,
)

from dev.engineeringlabs.pyboot.web.core import (
    Router,
    CORSMiddleware,
    route,
    get,
    post,
    put,
    delete,
)

__all__ = [
    # API
    "Request",
    "Response",
    "Route",
    "WebError",
    "HTTPStatus",
    # Core
    "Router",
    "CORSMiddleware",
    "route",
    "get",
    "post",
    "put",
    "delete",
]
