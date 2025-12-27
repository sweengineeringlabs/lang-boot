//! Swagger UI integration.
//!
//! This module provides utilities for serving Swagger UI with generated OpenAPI specs.

#[cfg(feature = "swagger-ui")]
pub use utoipa_swagger_ui::*;

use crate::spec::OpenApiSpec;

/// Swagger UI configuration.
#[derive(Debug, Clone)]
pub struct SwaggerUiConfig {
    /// Path to serve Swagger UI (e.g., "/swagger-ui")
    pub path: String,
    /// URL to the OpenAPI spec (e.g., "/api-docs/openapi.json")
    pub spec_url: String,
    /// Page title
    pub title: String,
}

impl Default for SwaggerUiConfig {
    fn default() -> Self {
        Self {
            path: "/swagger-ui".to_string(),
            spec_url: "/api-docs/openapi.json".to_string(),
            title: "API Documentation".to_string(),
        }
    }
}

impl SwaggerUiConfig {
    /// Create a new Swagger UI configuration.
    pub fn new(path: impl Into<String>, spec_url: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            spec_url: spec_url.into(),
            title: "API Documentation".to_string(),
        }
    }

    /// Set the page title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

/// Generate Swagger UI HTML.
pub fn generate_swagger_ui_html(config: &SwaggerUiConfig) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{}</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css">
    <style>
        html {{ box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }}
        *, *:before, *:after {{ box-sizing: inherit; }}
        body {{ margin:0; padding:0; }}
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {{
            const ui = SwaggerUIBundle({{
                url: "{}",
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            }});
            window.ui = ui;
        }};
    </script>
</body>
</html>"#,
        config.title, config.spec_url
    )
}

/// Serve OpenAPI spec and Swagger UI.
///
/// This struct provides methods to generate responses for serving
/// OpenAPI specifications and Swagger UI.
pub struct SwaggerUiServer {
    spec: OpenApiSpec,
    config: SwaggerUiConfig,
}

impl SwaggerUiServer {
    /// Create a new Swagger UI server.
    pub fn new(spec: OpenApiSpec, config: SwaggerUiConfig) -> Self {
        Self { spec, config }
    }

    /// Get the OpenAPI spec as JSON.
    pub fn spec_json(&self) -> crate::Result<String> {
        self.spec.to_json()
    }

    /// Get the OpenAPI spec as YAML.
    #[cfg(feature = "yaml")]
    pub fn spec_yaml(&self) -> crate::Result<String> {
        self.spec.to_yaml()
    }

    /// Get the Swagger UI HTML.
    pub fn ui_html(&self) -> String {
        generate_swagger_ui_html(&self.config)
    }

    /// Get the base path for Swagger UI.
    pub fn ui_path(&self) -> &str {
        &self.config.path
    }

    /// Get the spec URL.
    pub fn spec_url(&self) -> &str {
        &self.config.spec_url
    }
}
