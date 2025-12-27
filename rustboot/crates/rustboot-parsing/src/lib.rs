//! Rustboot Parsing - Document parsing and analysis utilities
//!
//! Utilities for parsing and extracting information from various document formats:
//! - Markdown analysis and link extraction
//! - Future: HTML parsing, XML parsing, etc.

pub mod markdown;

pub use markdown::{analyze_markdown, extract_headers, extract_links, MarkdownStats};
