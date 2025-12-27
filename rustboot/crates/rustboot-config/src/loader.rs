//! Configuration loader with hierarchical merging

use crate::error::ConfigResult;
use crate::source::Source;
use crate::traits::Mergeable;
use serde::de::DeserializeOwned;

/// Configuration loader that can merge from multiple sources
pub struct ConfigLoader<T> {
    config: Option<T>,
}

impl<T> ConfigLoader<T>
where
    T: DeserializeOwned + Mergeable + Default,
{
    /// Create a new config loader
    pub fn new() -> Self {
        Self { config: None }
    }

    /// Load from a source
    pub fn load(mut self, source: impl Source) -> ConfigResult<Self> {
        let loaded: T = source.load()?;

        if let Some(existing) = &mut self.config {
            existing.merge(loaded);
        } else {
            self.config = Some(loaded);
        }

        Ok(self)
    }

    /// Build the final configuration
    pub fn build(self) -> T {
        self.config.unwrap_or_default()
    }

    /// Try to build, returning error if no config was loaded
    pub fn try_build(self) -> ConfigResult<T> {
        match self.config {
            Some(config) => Ok(config),
            None => Ok(T::default()),
        }
    }
}

impl<T> Default for ConfigLoader<T>
where
    T: DeserializeOwned + Mergeable + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{FileFormat, FileSource};
    use serde::Deserialize;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[derive(Debug, Deserialize, Default, PartialEq)]
    struct TestConfig {
        name: String,
        value: i32,
        optional: Option<String>,
    }

    impl Mergeable for TestConfig {
        fn merge(&mut self, other: Self) {
            if !other.name.is_empty() {
                self.name = other.name;
            }
            if other.value != 0 {
                self.value = other.value;
            }
            if other.optional.is_some() {
                self.optional = other.optional;
            }
        }
    }

    #[test]
    fn test_single_source() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("config.yaml");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "name: test\nvalue: 42").unwrap();

        let config: TestConfig = ConfigLoader::new()
            .load(FileSource::auto(file_path.to_str().unwrap()).unwrap())
            .unwrap()
            .build();

        assert_eq!(config.name, "test");
        assert_eq!(config.value, 42);
    }

    #[test]
    fn test_multiple_sources_merge() {
        let dir = TempDir::new().unwrap();

        // Base config
        let base_path = dir.path().join("base.yaml");
        let mut base_file = File::create(&base_path).unwrap();
        write!(base_file, "name: base\nvalue: 10").unwrap();

        // Override config
        let override_path = dir.path().join("override.yaml");
        let mut override_file = File::create(&override_path).unwrap();
        write!(override_file, "value: 99\noptional: override").unwrap();

        let config: TestConfig = ConfigLoader::new()
            .load(FileSource::auto(base_path.to_str().unwrap()).unwrap())
            .unwrap()
            .load(FileSource::auto(override_path.to_str().unwrap()).unwrap())
            .unwrap()
            .build();

        assert_eq!(config.name, "base"); // From base
        assert_eq!(config.value, 99); // Overridden
        assert_eq!(config.optional, Some("override".to_string())); // From override
    }
}
