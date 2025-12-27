//! Router for registering and matching routes.

use crate::{Handler, HandlerContext, Response, WebError, WebResult};
use crate::handler::SharedHandler;
use std::collections::HashMap;
use std::sync::Arc;

/// HTTP methods supported by the router.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl RouteMethod {
    /// Convert string to RouteMethod.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(RouteMethod::Get),
            "POST" => Some(RouteMethod::Post),
            "PUT" => Some(RouteMethod::Put),
            "DELETE" => Some(RouteMethod::Delete),
            "PATCH" => Some(RouteMethod::Patch),
            "HEAD" => Some(RouteMethod::Head),
            "OPTIONS" => Some(RouteMethod::Options),
            _ => None,
        }
    }

    /// Convert to string.
    pub fn as_str(&self) -> &'static str {
        match self {
            RouteMethod::Get => "GET",
            RouteMethod::Post => "POST",
            RouteMethod::Put => "PUT",
            RouteMethod::Delete => "DELETE",
            RouteMethod::Patch => "PATCH",
            RouteMethod::Head => "HEAD",
            RouteMethod::Options => "OPTIONS",
        }
    }
}

/// A registered route with its handler.
#[derive(Clone)]
struct Route {
    method: RouteMethod,
    path: String,
    handler: SharedHandler,
    path_params: Vec<String>,
}

impl Route {
    /// Check if this route matches the given method and path.
    /// Returns path parameters if matched.
    fn matches(&self, method: RouteMethod, path: &str) -> Option<HashMap<String, String>> {
        if self.method != method {
            return None;
        }

        // Split both paths into segments
        let route_segments: Vec<&str> = self.path.split('/').filter(|s| !s.is_empty()).collect();
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        // Must have same number of segments
        if route_segments.len() != path_segments.len() {
            return None;
        }

        let mut params = HashMap::new();

        // Check each segment
        for (route_seg, path_seg) in route_segments.iter().zip(path_segments.iter()) {
            if route_seg.starts_with(':') {
                // This is a parameter
                let param_name = &route_seg[1..];
                params.insert(param_name.to_string(), path_seg.to_string());
            } else if route_seg != path_seg {
                // Static segment doesn't match
                return None;
            }
        }

        Some(params)
    }
}

/// Router for managing routes and handlers.
pub struct Router {
    routes: Vec<Route>,
    middleware: Vec<Arc<dyn rustboot_middleware::Middleware<HandlerContext>>>,
}

impl Router {
    /// Create a new router.
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    /// Create a router builder.
    pub fn builder() -> RouterBuilder {
        RouterBuilder::new()
    }

    /// Add a route to the router.
    pub fn route(&mut self, method: RouteMethod, path: &str, handler: SharedHandler) {
        // Extract path parameters
        let path_params = path
            .split('/')
            .filter(|s| s.starts_with(':'))
            .map(|s| s[1..].to_string())
            .collect();

        let route = Route {
            method,
            path: path.to_string(),
            handler,
            path_params,
        };

        self.routes.push(route);
    }

    /// Add middleware to the router.
    pub fn add_middleware<M>(&mut self, middleware: M)
    where
        M: rustboot_middleware::Middleware<HandlerContext> + 'static,
    {
        self.middleware.push(Arc::new(middleware));
    }

    /// Find and execute a handler for the given request.
    pub async fn handle(&self, mut ctx: HandlerContext) -> WebResult<Response> {
        let method = RouteMethod::from_str(&ctx.method)
            .ok_or_else(|| WebError::MethodNotAllowed(ctx.method.clone()))?;

        // Find matching route
        let route = self
            .routes
            .iter()
            .find_map(|r| r.matches(method, &ctx.path).map(|params| (r, params)))
            .ok_or_else(|| WebError::NotFound(ctx.path.clone()))?;

        // Add path parameters to context
        ctx.params = route.1;

        // Execute handler
        route.0.handler.handle(ctx).await
    }

    /// Get all registered routes.
    pub fn routes(&self) -> Vec<(RouteMethod, String)> {
        self.routes
            .iter()
            .map(|r| (r.method, r.path.clone()))
            .collect()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing routers.
pub struct RouterBuilder {
    routes: Vec<(RouteMethod, String, SharedHandler)>,
    middleware: Vec<Arc<dyn rustboot_middleware::Middleware<HandlerContext>>>,
}

impl RouterBuilder {
    /// Create a new router builder.
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    /// Add a GET route.
    pub fn get<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        self.routes.push((RouteMethod::Get, path.to_string(), Arc::new(handler)));
        self
    }

    /// Add a POST route.
    pub fn post<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        self.routes.push((RouteMethod::Post, path.to_string(), Arc::new(handler)));
        self
    }

    /// Add a PUT route.
    pub fn put<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        self.routes.push((RouteMethod::Put, path.to_string(), Arc::new(handler)));
        self
    }

    /// Add a DELETE route.
    pub fn delete<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        self.routes.push((RouteMethod::Delete, path.to_string(), Arc::new(handler)));
        self
    }

    /// Add a PATCH route.
    pub fn patch<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        self.routes.push((RouteMethod::Patch, path.to_string(), Arc::new(handler)));
        self
    }

    /// Add a route with any method.
    pub fn route<H>(mut self, method: RouteMethod, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        self.routes.push((method, path.to_string(), Arc::new(handler)));
        self
    }

    /// Add middleware.
    pub fn middleware<M>(mut self, middleware: M) -> Self
    where
        M: rustboot_middleware::Middleware<HandlerContext> + 'static,
    {
        self.middleware.push(Arc::new(middleware));
        self
    }

    /// Build the router.
    pub fn build(self) -> Router {
        let mut router = Router::new();

        for (method, path, handler) in self.routes {
            router.route(method, &path, handler);
        }

        for middleware in self.middleware {
            router.middleware.push(middleware);
        }

        router
    }
}

impl Default for RouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Response;

    #[tokio::test]
    async fn test_router_basic() {
        let router = Router::builder()
            .get("/", |_ctx| async { Ok(Response::ok().with_text("Home")) })
            .get("/users", |_ctx| async { Ok(Response::ok().with_text("Users")) })
            .build();

        let ctx = HandlerContext::new("GET".to_string(), "/".to_string());
        let response = router.handle(ctx).await.unwrap();
        assert_eq!(response.body, b"Home");

        let ctx = HandlerContext::new("GET".to_string(), "/users".to_string());
        let response = router.handle(ctx).await.unwrap();
        assert_eq!(response.body, b"Users");
    }

    #[tokio::test]
    async fn test_router_path_params() {
        let router = Router::builder()
            .get("/users/:id", |ctx: HandlerContext| async move {
                let id = ctx.param("id").unwrap_or("unknown");
                Ok(Response::ok().with_text(format!("User {}", id)))
            })
            .build();

        let ctx = HandlerContext::new("GET".to_string(), "/users/123".to_string());
        let response = router.handle(ctx).await.unwrap();
        assert_eq!(response.body, b"User 123");
    }

    #[tokio::test]
    async fn test_router_multiple_params() {
        let router = Router::builder()
            .get("/users/:user_id/posts/:post_id", |ctx: HandlerContext| async move {
                let user_id = ctx.param("user_id").unwrap_or("unknown");
                let post_id = ctx.param("post_id").unwrap_or("unknown");
                Ok(Response::ok().with_text(format!("User {} Post {}", user_id, post_id)))
            })
            .build();

        let ctx = HandlerContext::new("GET".to_string(), "/users/42/posts/99".to_string());
        let response = router.handle(ctx).await.unwrap();
        assert_eq!(response.body, b"User 42 Post 99");
    }

    #[tokio::test]
    async fn test_router_not_found() {
        let router = Router::builder()
            .get("/users", |_ctx| async { Ok(Response::ok().with_text("Users")) })
            .build();

        let ctx = HandlerContext::new("GET".to_string(), "/posts".to_string());
        let result = router.handle(ctx).await;
        assert!(result.is_err());
        match result.err().unwrap() {
            WebError::NotFound(_) => {},
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_router_method_not_allowed() {
        let router = Router::builder()
            .get("/users", |_ctx| async { Ok(Response::ok().with_text("Users")) })
            .build();

        let ctx = HandlerContext::new("POST".to_string(), "/users".to_string());
        let result = router.handle(ctx).await;
        assert!(result.is_err());
        // POST to GET-only route should return NotFound since no matching route exists
        match result.err().unwrap() {
            WebError::NotFound(_) => {},
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_route_matching() {
        let route = Route {
            method: RouteMethod::Get,
            path: "/users/:id/posts/:post_id".to_string(),
            handler: Arc::new(|_ctx| async { Ok(Response::ok()) }),
            path_params: vec!["id".to_string(), "post_id".to_string()],
        };

        let params = route.matches(RouteMethod::Get, "/users/123/posts/456").unwrap();
        assert_eq!(params.get("id"), Some(&"123".to_string()));
        assert_eq!(params.get("post_id"), Some(&"456".to_string()));

        assert!(route.matches(RouteMethod::Get, "/users/123").is_none());
        assert!(route.matches(RouteMethod::Post, "/users/123/posts/456").is_none());
    }
}
