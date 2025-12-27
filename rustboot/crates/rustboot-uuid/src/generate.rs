//! UUID generation

use crate::error::UuidResult;
use uuid::Uuid;

/// Generate a new UUID v4 (random)
pub fn new_v4() -> Uuid {
    Uuid::new_v4()
}

/// Generate a new UUID v7 (time-based, sortable)
pub fn new_v7() -> Uuid {
    Uuid::now_v7()
}

/// Parse a UUID from string
pub fn parse_uuid(s: &str) -> UuidResult<Uuid> {
    Uuid::parse_str(s).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_v4() {
        let uuid1 = new_v4();
        let uuid2 = new_v4();
        assert_ne!(uuid1, uuid2);
        assert_eq!(uuid1.get_version_num(), 4);
    }

    #[test]
    fn test_new_v7() {
        let uuid = new_v7();
        assert_eq!(uuid.get_version_num(), 7);
    }

    #[test]
    fn test_parse_uuid() {
        let uuid = new_v4();
        let s = uuid.to_string();
        let parsed = parse_uuid(&s).unwrap();
        assert_eq!(uuid, parsed);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_uuid("invalid").is_err());
    }
}
