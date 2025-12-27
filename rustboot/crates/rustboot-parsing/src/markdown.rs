//! Markdown analysis and statistics helpers

use serde::{Deserialize, Serialize};

/// Statistics about markdown content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarkdownStats {
    /// Total number of lines
    pub lines: usize,
    /// Total number of words
    pub words: usize,
    /// Total number of characters
    pub characters: usize,
    /// Number of headers (lines starting with #)
    pub headers: usize,
    /// Number of code blocks (delimited by ```)
    pub code_blocks: usize,
    /// Number of links ([text](url))
    pub links: usize,
}

/// Analyze markdown content and return statistics
///
/// # Example
/// ```
/// use dev_engineeringlabs_rustboot_parsing::markdown::analyze_markdown;
///
/// let content = "# Title\n\nSome text with a [link](https://example.com).\n\n```rust\nlet code = \"example\";\n```";
///
/// let stats = analyze_markdown(content);
/// assert!(stats.headers > 0);
/// assert!(stats.code_blocks > 0);
/// assert!(stats.links > 0);
/// ```
pub fn analyze_markdown(content: &str) -> MarkdownStats {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let characters = content.chars().count();

    // Count headers (lines starting with #)
    let headers = content.lines().filter(|l| l.trim().starts_with('#')).count();

    // Count code blocks (delimited by ```)
    let code_blocks = content.matches("```").count() / 2;

    // Count links ([text](url))
    let links = content.matches("](").count();

    MarkdownStats {
        lines,
        words,
        characters,
        headers,
        code_blocks,
        links,
    }
}

/// Extract all links from markdown content
///
/// Returns a vector of (link_text, url) tuples
pub fn extract_links(content: &str) -> Vec<(String, String)> {
    let mut links = Vec::new();
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '[' {
            // Extract link text
            let mut text = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == ']' {
                    chars.next(); // consume ]
                    break;
                }
                text.push(chars.next().unwrap());
            }

            // Check for (url)
            if chars.peek() == Some(&'(') {
                chars.next(); // consume (
                let mut url = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == ')' {
                        chars.next(); // consume )
                        break;
                    }
                    url.push(chars.next().unwrap());
                }

                if !text.is_empty() && !url.is_empty() {
                    links.push((text, url));
                }
            }
        }
    }

    links
}

/// Extract all headers from markdown content
///
/// Returns a vector of (level, text) tuples where level is 1-6
pub fn extract_headers(content: &str) -> Vec<(usize, String)> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.starts_with('#') {
                return None;
            }

            let mut level = 0;
            let mut chars = trimmed.chars();

            while let Some('#') = chars.next() {
                level += 1;
                if level > 6 {
                    return None;
                }
            }

            let text = chars.collect::<String>().trim().to_string();
            if text.is_empty() {
                return None;
            }

            Some((level, text))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_markdown() {
        let content = r#"
# Title

Some text with a [link](https://example.com).

## Subtitle

```rust
let code = "example";
```

Another [link](https://example.org).
"#;

        let stats = analyze_markdown(content);

        assert!(stats.lines > 0);
        assert!(stats.words > 0);
        assert_eq!(stats.headers, 2);
        assert_eq!(stats.code_blocks, 1);
        assert_eq!(stats.links, 2);
    }

    #[test]
    fn test_extract_links() {
        let content = "Check [this](https://example.com) and [that](https://example.org).";
        let links = extract_links(content);

        assert_eq!(links.len(), 2);
        assert_eq!(links[0], ("this".to_string(), "https://example.com".to_string()));
        assert_eq!(links[1], ("that".to_string(), "https://example.org".to_string()));
    }

    #[test]
    fn test_extract_headers() {
        let content = r#"
# H1 Title
## H2 Subtitle
### H3 Section
"#;
        let headers = extract_headers(content);

        assert_eq!(headers.len(), 3);
        assert_eq!(headers[0], (1, "H1 Title".to_string()));
        assert_eq!(headers[1], (2, "H2 Subtitle".to_string()));
        assert_eq!(headers[2], (3, "H3 Section".to_string()));
    }
}
