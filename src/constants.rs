/*
    src/constants.rs
    This file is part of the JSON-rust crate.
    Written by, Q@khaa.pk
 */

pub const JSON_OPENIING_BRACE: &str = "{";
pub const JSON_CLOSING_BRACE: &str = "}";

// Regular expression patterns for JSON structures
// --------------------------------------------------

pub const JSON_OPENING_BRACE_REG_EXPR_PATTERN: &str = r"^\s*\{\s*$";
pub const JSON_CLOSING_BRACE_REG_EXPR_PATTERN: &str = r"^\s*\}\s*,?\s*$";

pub const JSON_OPENING_BRACKET_REG_EXPR_PATTERN: &str = r"^\s*\[\s*$";
pub const JSON_CLOSING_BRACKET_REG_EXPR_PATTERN: &str = r"^\s*\]\s*,?\s*$";

// The # symbols in r#"..."# are raw string delimiters in Rust
pub const JSON_KEY_REG_EXPR_PATTERN: &str = r#"^\s*"([^"\\]|\\.)*"\s*:\s*"#;
// The # symbols in r#"..."# are raw string delimiters in Rust
pub const JSON_VALUE_TYPE_STRING_REG_EXPR_PATTERN: &str = r#":\s*"([^"\\]|\\.)*"\s*,?\s*$"#;
// Simple pattern to capture anything inside quotes
pub const JSON_QUOTED_CONTENT_PATTERN: &str = r#""([^"]*)""#;

// JSON number as value in key-value pair (with colon and optional comma)
pub const JSON_VALUE_TYPE_NUMERIC_PATTERN: &str = r":\s*(-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)\s*,?\s*$";
// Just capture any number from anywhere in the line
pub const CAPTURE_NUMBER_PATTERN: &str = r"(-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)";

// JSON array openinig square bracket
// r"[\s.]*\[\s*$";
//  r#"\s*"[^"]*":\s*\["#;
pub const JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE: &str = r#":\s*\[\s*$"#;
pub const JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE: &str = r"^\s*\]\s*,?\s*$";
pub const JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN: &str =  r":\s*\[[^\]]*\]"; // r"^:\s*\[(\s*.*\s*)+\]\s*$"; // r"^\s*(\d+)\s*,?\s*$"
pub const JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN_VALUE_STRING: &str = r":\s*\[([^\]]*)\]";