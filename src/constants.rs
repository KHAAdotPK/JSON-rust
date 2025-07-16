/*
    src/constants.rs
    This file is part of the JSON-rust crate.
    Written by, Q@khaa.pk
 */

/*
    The r at the beginning creates a raw string literal in Rust. Why Use Raw Strings for Regex?
    Without Raw String (painful):
    // You have to escape every backslash
    let pattern = "\"([^\"]+)\"\\s*:\\s*\\{(.*)\\}\\s*,?\\s*$";
    //             ^^     ^^    ^^      ^^    ^^        ^^
    //             All these backslashes need to be doubled!
    
    With Raw String (clean):
    // No escaping needed - what you see is what you get
    let pattern = r#""([^"]+)"\s*:\s*\{(.*)\}\s*,?\s*$"#;
    //            ^^                                   ^^
    //            Raw string delimiters

    -----------------------------------------------------------------------------------------------
    The # Symbol:    
    The # symbols in r#"..."# are raw string delimiters in Rust
    The # symbols are delimiters that let you include quotes inside the raw string:
    let pattern = r#""([^"]+)"\s*:\s*\{(.*)\}\s*,?\s*$"#;
    //            ^^                                   ^^
    //            Raw string delimiters

    let pattern = r##"This can contain # and "quotes""##  // Multiple #'s if needed
 */ 

pub const JSON_OPENIING_BRACE: &str = "{";
pub const JSON_CLOSING_BRACE: &str = "}";

// Regular expression patterns for JSON structures
// --------------------------------------------------

pub const JSON_VALUE_OPENING_BRACE_REG_EXPR_PATTERN: &str = r#""[^"]*"\s*:\s*[^{]*\{\s*(?:[^}]|$)"#; //r#":\s*\{\s*"#; //r"(?m)(?:^\s*\{\s*$|:\s*\{)"; // r#":\s*\{\s*\.*\s*"#;  // r"^\s*\{\s*$"; 
pub const JSON_VALUE_CLOSING_BRACE_REG_EXPR_PATTERN: &str = r#"[^{}\s][^{}]*\}\s*(?:,\s*)?$"#; //r"(?m)(?:^\s*\}\s*$|\S.*\})"; // r#"\s*\.*\s*\}\s*"#;  // r"^\s*\}\s*,?\s*$";

pub const JSON_OPENING_BRACE_REG_EXPR_PATTERN: &str = r"^\s*\{\s*$"; // Standalone opening brace
pub const JSON_CLOSING_BRACE_REG_EXPR_PATTERN: &str = r"^\s*\}\s*,?\s*$"; // Standalone closing brace followed by optional comma

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

pub const JSON_VALUE_TYPE_NULL_PATTERN: &str = r#":\s*null\s*,?\s*$"#;
pub const JSON_VALUE_TYPE_TRUE_PATTERN: &str = r#":\s*true\s*,?\s*$"#;
pub const JSON_VALUE_TYPE_FALSE_PATTERN: &str = r#":\s*false\s*,?\s*$"#;

// JSON array openinig square bracket
// r"[\s.]*\[\s*$";
//  r#"\s*"[^"]*":\s*\["#;
pub const JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE: &str = r#":\s*\[\s*$"#;
pub const JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE: &str = r"^\s*\]\s*,?\s*$";
pub const JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN: &str =  r":\s*\[[^\]]*\]"; // r"^:\s*\[(\s*.*\s*)+\]\s*$"; // r"^\s*(\d+)\s*,?\s*$"
pub const JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN_VALUE_STRING: &str = r":\s*\[([^\]]*)\]";

/* Process Single Line JSON Ibject, divide line in two groups; first group contains key and the other group contains value */  
pub const JSON_SINGLE_LINE_OBJECT_TYPE_KEY_NAME_WITH_OPENING_CLOSING_BRACE_PATTERN: &str = r#""([^"]+)"\s*:\s*\{(.*)\}\s*,?\s*$"#;

