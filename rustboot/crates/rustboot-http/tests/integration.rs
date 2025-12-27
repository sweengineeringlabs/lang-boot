//! Integration tests for rustboot-http

use dev_engineeringlabs_rustboot_http::{HttpClient, Method, Request, Response};

#[cfg(feature = "reqwest")]
use dev_engineeringlabs_rustboot_http::ReqwestClient;

#[cfg(feature = "reqwest")]
use wiremock::{
    matchers::{body_string, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_get_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/test"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Hello, World!"))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let response = client.get(&format!("{}/api/test", mock_server.uri())).await.unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(response.text().unwrap(), "Hello, World!");
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_post_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/data"))
        .and(body_string("test body"))
        .respond_with(ResponseTemplate::new(201).set_body_string("Created"))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let response = client
        .post(&format!("{}/api/data", mock_server.uri()), b"test body".to_vec())
        .await
        .unwrap();

    assert_eq!(response.status, 201);
    assert_eq!(response.text().unwrap(), "Created");
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_put_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/api/update"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Updated"))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let response = client
        .put(&format!("{}/api/update", mock_server.uri()), b"update data".to_vec())
        .await
        .unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(response.text().unwrap(), "Updated");
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_delete_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api/resource/123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let response = client
        .delete(&format!("{}/api/resource/123", mock_server.uri()))
        .await
        .unwrap();

    assert_eq!(response.status, 204);
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_custom_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/auth"))
        .and(header("Authorization", "Bearer token123"))
        .and(header("X-Custom-Header", "custom-value"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Authenticated"))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let request = Request::new(Method::Get, format!("{}/api/auth", mock_server.uri()))
        .header("Authorization".to_string(), "Bearer token123".to_string())
        .header("X-Custom-Header".to_string(), "custom-value".to_string());

    let response = client.send(request).await.unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(response.text().unwrap(), "Authenticated");
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_json_request_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/json"))
        .and(header("Content-Type", "application/json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"{"id": 1, "name": "test"}"#)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();

    #[derive(serde::Serialize)]
    struct RequestBody {
        name: String,
    }

    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct ResponseBody {
        id: i32,
        name: String,
    }

    let request = Request::new(Method::Post, format!("{}/api/json", mock_server.uri()))
        .json(&RequestBody { name: "test".to_string() })
        .unwrap();

    let response = client.send(request).await.unwrap();

    assert_eq!(response.status, 200);
    let body: ResponseBody = response.json().unwrap();
    assert_eq!(body, ResponseBody { id: 1, name: "test".to_string() });
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_response_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/headers"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("OK")
                .insert_header("X-Request-Id", "abc123")
                .insert_header("X-Rate-Limit", "100"),
        )
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let response = client.get(&format!("{}/api/headers", mock_server.uri())).await.unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(response.headers.get("x-request-id").unwrap(), "abc123");
    assert_eq!(response.headers.get("x-rate-limit").unwrap(), "100");
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_error_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/notfound"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let response = client.get(&format!("{}/api/notfound", mock_server.uri())).await.unwrap();

    assert_eq!(response.status, 404);
    assert!(!response.is_success());
    assert_eq!(response.text().unwrap(), "Not Found");
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_server_error_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/error"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let response = client.get(&format!("{}/api/error", mock_server.uri())).await.unwrap();

    assert_eq!(response.status, 500);
    assert!(!response.is_success());
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_connection_error() {
    let client = ReqwestClient::new();
    // Try to connect to a non-existent server
    let result = client.get("http://127.0.0.1:59999/nonexistent").await;

    assert!(result.is_err());
}

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_all_http_methods() {
    let mock_server = MockServer::start().await;

    // HEAD
    Mock::given(method("HEAD"))
        .and(path("/api/head"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    // OPTIONS
    Mock::given(method("OPTIONS"))
        .and(path("/api/options"))
        .respond_with(ResponseTemplate::new(200).insert_header("Allow", "GET, POST, OPTIONS"))
        .mount(&mock_server)
        .await;

    // PATCH
    Mock::given(method("PATCH"))
        .and(path("/api/patch"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Patched"))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();

    // Test HEAD
    let head_request = Request::new(Method::Head, format!("{}/api/head", mock_server.uri()));
    let head_response = client.send(head_request).await.unwrap();
    assert_eq!(head_response.status, 200);

    // Test OPTIONS
    let options_request = Request::new(Method::Options, format!("{}/api/options", mock_server.uri()));
    let options_response = client.send(options_request).await.unwrap();
    assert_eq!(options_response.status, 200);

    // Test PATCH
    let patch_request = Request::new(Method::Patch, format!("{}/api/patch", mock_server.uri()))
        .body(b"patch data".to_vec());
    let patch_response = client.send(patch_request).await.unwrap();
    assert_eq!(patch_response.status, 200);
    assert_eq!(patch_response.text().unwrap(), "Patched");
}

// Tests that work without the reqwest feature
#[test]
fn test_request_builder() {
    let request = Request::new(Method::Get, "https://example.com".to_string())
        .header("Accept".to_string(), "application/json".to_string())
        .body(b"test".to_vec());

    assert_eq!(request.method, Method::Get);
    assert_eq!(request.url, "https://example.com");
    assert_eq!(request.headers.get("Accept").unwrap(), "application/json");
    assert_eq!(request.body.unwrap(), b"test");
}

#[test]
fn test_request_json() {
    #[derive(serde::Serialize)]
    struct Data {
        value: i32,
    }

    let request = Request::new(Method::Post, "https://example.com".to_string())
        .json(&Data { value: 42 })
        .unwrap();

    assert_eq!(request.headers.get("Content-Type").unwrap(), "application/json");
    assert_eq!(request.body.unwrap(), br#"{"value":42}"#);
}

#[test]
fn test_response_is_success() {
    let success_response = Response {
        status: 200,
        headers: std::collections::HashMap::new(),
        body: vec![],
    };
    assert!(success_response.is_success());

    let created_response = Response {
        status: 201,
        headers: std::collections::HashMap::new(),
        body: vec![],
    };
    assert!(created_response.is_success());

    let redirect_response = Response {
        status: 301,
        headers: std::collections::HashMap::new(),
        body: vec![],
    };
    assert!(!redirect_response.is_success());

    let error_response = Response {
        status: 404,
        headers: std::collections::HashMap::new(),
        body: vec![],
    };
    assert!(!error_response.is_success());
}

#[test]
fn test_response_text() {
    let response = Response {
        status: 200,
        headers: std::collections::HashMap::new(),
        body: b"Hello, World!".to_vec(),
    };
    assert_eq!(response.text().unwrap(), "Hello, World!");
}

#[test]
fn test_response_json() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct Data {
        value: i32,
    }

    let response = Response {
        status: 200,
        headers: std::collections::HashMap::new(),
        body: br#"{"value": 42}"#.to_vec(),
    };
    let data: Data = response.json().unwrap();
    assert_eq!(data, Data { value: 42 });
}

#[test]
fn test_method_variants() {
    assert_eq!(Method::Get, Method::Get);
    assert_ne!(Method::Get, Method::Post);
    assert_ne!(Method::Put, Method::Patch);
}
