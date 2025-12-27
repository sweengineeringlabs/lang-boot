//! Authentication module
//!
//! JWT token generation/validation, session management, API keys

use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// JWT claims structure
#[derive(Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

/// Simple JWT token structure (base64-encoded JSON, not cryptographically signed)
/// Note: This is a basic implementation for demonstration. For production use,
/// use a proper JWT library like `jsonwebtoken` with proper signing.
#[derive(Debug)]
pub struct Token {
    pub header: String,
    pub payload: Claims,
    pub signature: String,
}

/// Generate JWT token
/// 
/// This is a simplified implementation that creates a token-like structure.
/// For production use, integrate with a proper JWT library with signing.
pub fn generate_jwt(user_id: &str, duration: Duration) -> crate::SecurityResult<String> {
    if duration.as_millis() == 0 {
        return Err(crate::SecurityError::InvalidToken("Duration must be greater than zero".to_string()));
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| crate::SecurityError::InvalidToken(format!("System time error: {}", e)))?
        .as_millis() as i64;

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + duration.as_millis() as i64,
    };

    // Simple token format: base64(header).base64(claims).signature
    // Note: This is NOT cryptographically secure - use jsonwebtoken crate for production
    let header = base64_encode("{\"alg\":\"HS256\",\"typ\":\"JWT\"}");
    let payload_json = format!(
        r#"{{"sub":"{}","iat":{},"exp":{}}}"#,
        claims.sub, claims.iat, claims.exp
    );
    let payload = base64_encode(&payload_json);
    let signature = simple_sign(&format!("{}.{}", header, payload));

    Ok(format!("{}.{}.{}", header, payload, signature))
}

/// Validate JWT token
///
/// This is a simplified implementation for demonstration.
/// For production use, integrate with a proper JWT library.
pub fn validate_jwt(token: &str) -> crate::SecurityResult<Claims> {
    let parts: Vec<&str> = token.split('.').collect();
    
    if parts.len() != 3 {
        return Err(crate::SecurityError::InvalidToken("Invalid token format".to_string()));
    }

    let header = parts[0];
    let payload = parts[1];
    let signature = parts[2];

    // Verify signature
    let expected_signature = simple_sign(&format!("{}.{}", header, payload));
    if signature != expected_signature {
        return Err(crate::SecurityError::InvalidToken("Invalid signature".to_string()));
    }

    // Decode payload
    let payload_json = base64_decode(payload)
        .map_err(|e| crate::SecurityError::InvalidToken(format!("Invalid payload encoding: {}", e)))?;

    let claims = parse_claims(&payload_json)?;

    // Check expiration
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| crate::SecurityError::InvalidToken(format!("System time error: {}", e)))?
        .as_millis() as i64;

    if claims.exp < now {
        return Err(crate::SecurityError::InvalidToken("Token expired".to_string()));
    }

    Ok(claims)
}

// Helper functions

fn base64_encode(input: &str) -> String {
    use std::fmt::Write;
    let bytes = input.as_bytes();
    let mut result = String::new();
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        let b1 = (buf[0] >> 2) & 0x3F;
        let b2 = ((buf[0] & 0x03) << 4) | ((buf[1] >> 4) & 0x0F);
        let b3 = ((buf[1] & 0x0F) << 2) | ((buf[2] >> 6) & 0x03);
        let b4 = buf[2] & 0x3F;
        
        write!(&mut result, "{}", to_base64_char(b1)).unwrap();
        write!(&mut result, "{}", to_base64_char(b2)).unwrap();
        if chunk.len() > 1 {
            write!(&mut result, "{}", to_base64_char(b3)).unwrap();
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            write!(&mut result, "{}", to_base64_char(b4)).unwrap();
        } else {
            result.push('=');
        }
    }
    result
}

fn to_base64_char(val: u8) -> char {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    CHARS[val as usize] as char
}

fn base64_decode(input: &str) -> Result<String, String> {
    let input = input.trim_end_matches('=');
    let mut bytes = Vec::new();
    let chars: Vec<u8> = input.chars().map(from_base64_char).collect::<Result<_, _>>()?;
    
    for chunk in chars.chunks(4) {
        let b1 = chunk.first().copied().unwrap_or(0);
        let b2 = chunk.get(1).copied().unwrap_or(0);
        let b3 = chunk.get(2).copied().unwrap_or(0);
        let b4 = chunk.get(3).copied().unwrap_or(0);
        
        bytes.push((b1 << 2) | (b2 >> 4));
        if chunk.len() > 2 {
            bytes.push(((b2 & 0x0F) << 4) | (b3 >> 2));
        }
        if chunk.len() > 3 {
            bytes.push(((b3 & 0x03) << 6) | b4);
        }
    }
    
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 error: {}", e))
}

fn from_base64_char(c: char) -> Result<u8, String> {
    match c {
        'A'..='Z' => Ok((c as u8) - b'A'),
        'a'..='z' => Ok((c as u8) - b'a' + 26),
        '0'..='9' => Ok((c as u8) - b'0' + 52),
        '+' => Ok(62),
        '/' => Ok(63),
        _ => Err(format!("Invalid base64 character: {}", c)),
    }
}

fn simple_sign(data: &str) -> String {
    // Simple hash function - NOT cryptographically secure
    // For production, use HMAC-SHA256 or similar
    let mut hash: u64 = 5381;
    for byte in data.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    format!("{:x}", hash)
}

fn parse_claims(json: &str) -> crate::SecurityResult<Claims> {
    // Simple JSON parsing for claims
    let sub = extract_json_field(json, "sub")
        .ok_or_else(|| crate::SecurityError::InvalidToken("Missing 'sub' field".to_string()))?;
    let iat = extract_json_number(json, "iat")
        .ok_or_else(|| crate::SecurityError::InvalidToken("Missing 'iat' field".to_string()))?;
    let exp = extract_json_number(json, "exp")
        .ok_or_else(|| crate::SecurityError::InvalidToken("Missing 'exp' field".to_string()))?;

    Ok(Claims { sub, iat, exp })
}

fn extract_json_field(json: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", field);
    let start = json.find(&pattern)? + pattern.len();
    let end = json[start..].find('"')?;
    Some(json[start..start + end].to_string())
}

fn extract_json_number(json: &str, field: &str) -> Option<i64> {
    let pattern = format!("\"{}\":", field);
    let start = json.find(&pattern)? + pattern.len();
    let end = json[start..].find(|c: char| !c.is_ascii_digit())?;
    json[start..start + end].parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_valid_token() {
        let token = generate_jwt("user123", Duration::from_secs(3600));
        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn token_can_be_validated() {
        let token = generate_jwt("user123", Duration::from_secs(3600)).unwrap();
        let claims = validate_jwt(&token);
        assert!(claims.is_ok());
        let claims = claims.unwrap();
        assert_eq!(claims.sub, "user123");
    }

    #[test]
    fn rejects_invalid_token() {
        let result = validate_jwt("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_tampered_token() {
        let token = generate_jwt("user123", Duration::from_secs(3600)).unwrap();
        // Tamper with the signature (last part of the token)
        let parts: Vec<&str> = token.split('.').collect();
        let tampered = format!("{}.{}.invalid_signature", parts[0], parts[1]);
        let result = validate_jwt(&tampered);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_expired_token() {
        // Create a token that expires almost immediately (1 millisecond)
        let token = generate_jwt("user123", Duration::from_millis(1)).unwrap();
        std::thread::sleep(Duration::from_millis(10));
        let result = validate_jwt(&token);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_duration() {
        let result = generate_jwt("user", Duration::from_secs(0));
        assert!(result.is_err());
    }
}
