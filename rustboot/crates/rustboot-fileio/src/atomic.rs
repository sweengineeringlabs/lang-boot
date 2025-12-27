//! Atomic file operations

use crate::error::FileIoResult;
use std::io::Write;
use std::path::Path;

/// Write data to file atomically (write-then-rename)
pub fn write_atomic<P: AsRef<Path>>(path: P, data: &[u8]) -> FileIoResult<()> {
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        crate::error::FileIoError::PathError("No parent directory".to_string())
    })?;

    // Create temp file in same directory
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(data)?;
    temp.flush()?;

    // Atomic rename
    temp.persist(path)?;
    Ok(())
}

/// Write with specific permissions (Unix only)
#[cfg(unix)]
pub fn write_atomic_with_perms<P: AsRef<Path>>(
    path: P,
    data: &[u8],
    mode: u32,
) -> FileIoResult<()> {
    use std::os::unix::fs::PermissionsExt;
    
    let path = path.as_ref();
    write_atomic(path, data)?;
    
    let metadata = std::fs::metadata(path)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(mode);
    std::fs::set_permissions(path, permissions)?;
    
    Ok(())
}

/// Write with permissions (Windows - no-op)
#[cfg(not(unix))]
pub fn write_atomic_with_perms<P: AsRef<Path>>(
    path: P,
    data: &[u8],
    _mode: u32,
) -> FileIoResult<()> {
    write_atomic(path, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_atomic_write() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        write_atomic(&file_path, b"Hello, World!").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_atomic_write_overwrites() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        write_atomic(&file_path, b"First").unwrap();
        write_atomic(&file_path, b"Second").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Second");
    }
}
