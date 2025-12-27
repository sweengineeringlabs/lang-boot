//! Path utilities

use crate::error::{FileIoError, FileIoResult};
use std::path::{Path, PathBuf};

/// Ensure directory exists, creating it if necessary
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> FileIoResult<()> {
    std::fs::create_dir_all(path.as_ref())?;
    Ok(())
}

/// Safely join paths, preventing directory traversal
pub fn safe_join<P: AsRef<Path>, Q: AsRef<Path>>(base: P, child: Q) -> FileIoResult<PathBuf> {
    let base = base.as_ref();
    let child = child.as_ref();

    // Normalize and check for directory traversal
    let joined = base.join(child);
    let canonical_base = base.canonicalize().unwrap_or_else(|_| base.to_path_buf());
    
    // Check if result starts with base
    if !joined.starts_with(&canonical_base) {
        return Err(FileIoError::PathError(
            "Path traversal detected".to_string(),
        ));
    }

    Ok(joined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_dir() {
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("a/b/c");

        ensure_dir(&nested).unwrap();
        assert!(nested.exists());
    }

    #[test]
    fn test_safe_join() {
        let base = Path::new("/base");
        let result = safe_join(base, "file.txt").unwrap();
        assert_eq!(result, Path::new("/base/file.txt"));
    }

    #[test]
    fn test_safe_join_nested() {
        let base = Path::new("/base");
        let result = safe_join(base, "dir/file.txt").unwrap();
        assert_eq!(result, Path::new("/base/dir/file.txt"));
    }
}
