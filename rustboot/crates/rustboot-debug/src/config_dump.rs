//! Configuration dumping utilities for debugging.

use serde::Serialize;
use std::collections::HashMap;
use tracing::info;

/// Configuration dumper for debugging.
pub struct ConfigDumper;

impl ConfigDumper {
    /// Dump configuration as JSON (pretty-printed).
    pub fn dump_json<T: Serialize>(config: &T) -> Result<String, String> {
        serde_json::to_string_pretty(config).map_err(|e| format!("JSON serialization error: {}", e))
    }

    /// Dump configuration as compact JSON.
    pub fn dump_json_compact<T: Serialize>(config: &T) -> Result<String, String> {
        serde_json::to_string(config).map_err(|e| format!("JSON serialization error: {}", e))
    }

    /// Log configuration to tracing.
    pub fn log_config<T: Serialize>(name: &str, config: &T) {
        match Self::dump_json(config) {
            Ok(json) => {
                info!(
                    target: "rustboot::debug::config",
                    name = name,
                    "Configuration dump:\n{}", json
                );
            }
            Err(e) => {
                tracing::error!(
                    target: "rustboot::debug::config",
                    name = name,
                    error = %e,
                    "Failed to dump configuration"
                );
            }
        }
    }

    /// Redact sensitive fields from configuration.
    pub fn redact_sensitive(json: &str, sensitive_keys: &[&str]) -> String {
        let mut result = json.to_string();

        for key in sensitive_keys {
            // Simple regex-like replacement for sensitive fields
            let pattern = format!("\"{}\":", key);
            if let Some(start) = result.find(&pattern) {
                if let Some(value_start) = result[start..].find(':') {
                    let value_pos = start + value_start + 1;
                    if let Some(value_end) = result[value_pos..].find(',').or_else(|| result[value_pos..].find('}')) {
                        let end_pos = value_pos + value_end;
                        let original = &result[value_pos..end_pos];
                        let redacted = if original.trim().starts_with('"') {
                            " \"***REDACTED***\"".to_string()
                        } else {
                            " \"***REDACTED***\"".to_string()
                        };
                        result.replace_range(value_pos..end_pos, &redacted);
                    }
                }
            }
        }

        result
    }

    /// Compare two configurations and show differences.
    pub fn diff_configs<T: Serialize>(name1: &str, config1: &T, name2: &str, config2: &T) -> Result<String, String> {
        let json1 = Self::dump_json(config1)?;
        let json2 = Self::dump_json(config2)?;

        let mut output = String::new();
        output.push_str(&format!("Configuration Diff: {} vs {}\n", name1, name2));
        output.push_str("=====================================\n\n");

        if json1 == json2 {
            output.push_str("Configurations are identical.\n");
        } else {
            output.push_str(&format!("{}:\n{}\n\n", name1, json1));
            output.push_str(&format!("{}:\n{}\n", name2, json2));
        }

        Ok(output)
    }

    /// Generate a configuration summary (for large configs).
    pub fn summarize<T: Serialize>(config: &T) -> Result<ConfigSummary, String> {
        let json = Self::dump_json(config)?;
        let value: serde_json::Value = serde_json::from_str(&json)
            .map_err(|e| format!("JSON parsing error: {}", e))?;

        Ok(ConfigSummary::from_value(&value))
    }
}

/// Configuration summary for large configs.
#[derive(Debug, Clone)]
pub struct ConfigSummary {
    /// Total number of fields.
    pub total_fields: usize,
    /// Number of nested objects.
    pub nested_objects: usize,
    /// Number of arrays.
    pub arrays: usize,
    /// Top-level field names.
    pub top_level_fields: Vec<String>,
}

impl ConfigSummary {
    /// Create summary from JSON value.
    fn from_value(value: &serde_json::Value) -> Self {
        let mut summary = Self {
            total_fields: 0,
            nested_objects: 0,
            arrays: 0,
            top_level_fields: Vec::new(),
        };

        summary.count_fields(value, true);
        summary
    }

    /// Count fields recursively.
    fn count_fields(&mut self, value: &serde_json::Value, is_top_level: bool) {
        match value {
            serde_json::Value::Object(map) => {
                if !is_top_level {
                    self.nested_objects += 1;
                }

                for (key, val) in map {
                    self.total_fields += 1;

                    if is_top_level {
                        self.top_level_fields.push(key.clone());
                    }

                    self.count_fields(val, false);
                }
            }
            serde_json::Value::Array(arr) => {
                self.arrays += 1;
                for item in arr {
                    self.count_fields(item, false);
                }
            }
            _ => {}
        }
    }

    /// Format summary as string.
    pub fn format(&self) -> String {
        let mut output = String::new();
        output.push_str("Configuration Summary\n");
        output.push_str("====================\n\n");
        output.push_str(&format!("Total Fields: {}\n", self.total_fields));
        output.push_str(&format!("Nested Objects: {}\n", self.nested_objects));
        output.push_str(&format!("Arrays: {}\n\n", self.arrays));

        if !self.top_level_fields.is_empty() {
            output.push_str("Top-Level Fields:\n");
            for field in &self.top_level_fields {
                output.push_str(&format!("  - {}\n", field));
            }
        }

        output
    }
}

/// Environment variable configuration helper.
pub struct EnvConfigHelper;

impl EnvConfigHelper {
    /// Dump all environment variables matching a prefix.
    pub fn dump_with_prefix(prefix: &str) -> HashMap<String, String> {
        std::env::vars()
            .filter(|(key, _)| key.starts_with(prefix))
            .collect()
    }

    /// Log environment variables with prefix.
    pub fn log_with_prefix(prefix: &str) {
        let vars = Self::dump_with_prefix(prefix);

        info!(
            target: "rustboot::debug::config",
            prefix = prefix,
            count = vars.len(),
            "Environment variables"
        );

        for (key, value) in vars {
            info!(
                target: "rustboot::debug::config",
                key = %key,
                value = %value,
                "Environment variable"
            );
        }
    }

    /// Dump all environment variables as JSON.
    pub fn dump_all_as_json() -> Result<String, String> {
        let vars: HashMap<String, String> = std::env::vars().collect();
        serde_json::to_string_pretty(&vars)
            .map_err(|e| format!("JSON serialization error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestConfig {
        host: String,
        port: u16,
        password: String,
        nested: NestedConfig,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct NestedConfig {
        enabled: bool,
        timeout: u32,
    }

    #[test]
    fn test_dump_json() {
        let config = TestConfig {
            host: "localhost".to_string(),
            port: 8080,
            password: "secret".to_string(),
            nested: NestedConfig {
                enabled: true,
                timeout: 30,
            },
        };

        let json = ConfigDumper::dump_json(&config).unwrap();
        assert!(json.contains("localhost"));
        assert!(json.contains("8080"));
        assert!(json.contains("secret"));
    }

    #[test]
    fn test_dump_json_compact() {
        let config = TestConfig {
            host: "localhost".to_string(),
            port: 8080,
            password: "secret".to_string(),
            nested: NestedConfig {
                enabled: true,
                timeout: 30,
            },
        };

        let json = ConfigDumper::dump_json_compact(&config).unwrap();
        assert!(!json.contains('\n'));
    }

    #[test]
    fn test_redact_sensitive() {
        let json = r#"{"host":"localhost","password":"secret123","port":8080}"#;
        let redacted = ConfigDumper::redact_sensitive(json, &["password"]);
        assert!(!redacted.contains("secret123"));
        assert!(redacted.contains("REDACTED"));
    }

    #[test]
    fn test_summarize() {
        let config = TestConfig {
            host: "localhost".to_string(),
            port: 8080,
            password: "secret".to_string(),
            nested: NestedConfig {
                enabled: true,
                timeout: 30,
            },
        };

        let summary = ConfigDumper::summarize(&config).unwrap();
        assert!(summary.total_fields > 0);
        assert!(summary.top_level_fields.contains(&"host".to_string()));
    }

    #[test]
    fn test_diff_configs() {
        let config1 = TestConfig {
            host: "localhost".to_string(),
            port: 8080,
            password: "secret".to_string(),
            nested: NestedConfig {
                enabled: true,
                timeout: 30,
            },
        };

        let config2 = TestConfig {
            host: "localhost".to_string(),
            port: 9090,
            password: "secret".to_string(),
            nested: NestedConfig {
                enabled: true,
                timeout: 30,
            },
        };

        let diff = ConfigDumper::diff_configs("config1", &config1, "config2", &config2).unwrap();
        assert!(diff.contains("config1"));
        assert!(diff.contains("config2"));
    }

    #[test]
    fn test_env_config_helper() {
        std::env::set_var("TEST_VAR_1", "value1");
        std::env::set_var("TEST_VAR_2", "value2");

        let vars = EnvConfigHelper::dump_with_prefix("TEST_VAR");
        assert_eq!(vars.len(), 2);
        assert_eq!(vars.get("TEST_VAR_1"), Some(&"value1".to_string()));

        std::env::remove_var("TEST_VAR_1");
        std::env::remove_var("TEST_VAR_2");
    }
}
