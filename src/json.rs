/*
    src/kson.rs
    This file is part of the JSON-rust crate.
    Written by, Q@khaa.pk
 */


 //use std::fs::File;
 //use std::io::{self, Read}; 

use std::{env, io, fs::File, io::Read};
use regex::Regex;

use crate::file_content::FileContent;
use crate::constants::{JSON_OPENIING_BRACE, JSON_CLOSING_BRACE, JSON_OPENING_BRACE_REG_EXPR_PATTERN, JSON_CLOSING_BRACE_REG_EXPR_PATTERN, JSON_KEY_REG_EXPR_PATTERN, JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE, JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE, JSON_VALUE_TYPE_STRING_REG_EXPR_PATTERN, JSON_QUOTED_CONTENT_PATTERN, JSON_VALUE_TYPE_NUMERIC_PATTERN, JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN, JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN_VALUE_STRING, JSON_VALUE_OPENING_BRACE_REG_EXPR_PATTERN, JSON_VALUE_CLOSING_BRACE_REG_EXPR_PATTERN, JSON_SINGLE_LINE_OBJECT_TYPE_KEY_NAME_WITH_OPENING_CLOSING_BRACE_PATTERN, JSON_VALUE_TYPE_NULL_PATTERN, JSON_VALUE_TYPE_FALSE_PATTERN, JSON_VALUE_TYPE_TRUE_PATTERN};
use crate::json_object::{ValueType, Key, JsonKeyPtr, JsonObject};


/// Removes the outermost opening and closing braces from a JSON string, preserving nested braces.
///
/// # Arguments
/// * `input` - A string slice containing a JSON object (e.g., `{"key": "value"}`).
///
/// # Returns
/// * `Ok(&str)` - The input string with the outermost braces removed (e.g., `"key": "value"`).
/// * `Err(&'static str)` - If the input is invalid (e.g., not starting with `{`, not ending with `}`,
///   or unbalanced braces).
///
/// # Example
/// ```
/// let json = r#"{"name": "test", "nested": {"a": 1}}"#;
/// let result = remove_outer_braces(json).unwrap();
/// assert_eq!(result, r#""name": "test", "nested": {"a": 1}"#);
/// ```
fn remove_outer_braces(input: &str) -> Result<&str, &'static str> {
    let input = input.trim();
    if input.is_empty() {
        return Err("Input is empty");
    }
    if !input.starts_with('{') || !input.ends_with('}') {
        return Err("Input must start with '{' and end with '}'");
    }

    let mut brace_count = 0;
    let mut chars = input.chars().enumerate();
    let mut start_idx = None;

    for (i, ch) in chars.by_ref() {
        if ch == '{' {
            if brace_count == 0 {
                start_idx = Some(i);
            }
            brace_count += 1;
        } else if ch == '}' {
            brace_count -= 1;
            if brace_count == 0 && i == input.len() - 1 {
                return Ok(&input[start_idx.unwrap() + 1..i]);
            }
        }
    }

    Err("Unbalanced braces in input")
}


pub fn process_singleline_json_array(line: &str, key: &mut Key) {

}

/// Processes a single-line JSON object, populating a `Key` with its key-value pairs.
///
/// This function parses a JSON object (e.g., `{"key": "value", "num": 42}`) from a single-line
/// string, creating `Key` nodes for each key-value pair and storing them in the provided `key`'s
/// `ptr` field. It handles all JSON value types: strings, numbers, booleans, nulls, objects, and
/// arrays. Nested objects and arrays are processed recursively by calling
/// `process_singleline_json_object` or `process_singleline_json_array`.
///
/// **Note**: Handling of escaped characters (e.g., `\"`, `\\`) and quotation marks within strings
/// is still being finalized to ensure robust parsing of strings like `"value \"with\" quotes"`.
/// The current implementation assumes well-formed JSON but will be updated to fully support
/// escape sequences.
///
/// The function uses a state machine approach to track parsing states, including:
/// - **Key collection**: Capturing key names within quotation marks.
/// - **Value collection**: Identifying value types (string, number, boolean, null, object, array)
///   and handling nested structures.
/// - **Delimiter tracking**: Managing colons (`:`), commas (`,`), braces (`{}`), and brackets (`[]`).
/// - **Nested structure counting**: Using `brace_count` and `bracket_count` to ensure proper nesting.
///
/// # Arguments
/// * `line` - A string slice containing the JSON object (e.g., `{"key": "value"}`).
/// * `key` - A mutable reference to a `Key` where key-value pairs are stored as nested keys in
///   the `ptr` field. The `n` field of the `key` is updated to reflect the number of pairs.
///
/// # Returns
/// * `Ok(())` if parsing succeeds.
/// * `Err(&'static str)` if the input is malformed (e.g., unmatched braces, invalid values).
///
/// # State Machine Overview
/// The function implements a finite state machine with the following states:
/// - **Idle**: Initial state, skipping whitespace and waiting for a key's opening quote (`"`).
/// - **Key Collection**: Collecting characters between quotes for a key name (`start_of_key`).
/// - **Colon Encountered**: After a colon (`:`), identifying the value type (`colon_encountered`).
/// - **String Value**: Collecting string characters, pending handling of escaped quotes
///   (`value_type_string_found`).
/// - **Object Value**: Tracking nested object with `brace_count` (`opening_brace_encountered`).
/// - **Array Value**: Tracking nested array with `bracket_count` (`opening_bracket_encountered`).
/// - **Non-Complex Value**: Collecting `null`, `true`, `false`, or numbers until a comma or end.
///
/// Transitions are driven by characters (e.g., `"`, `:`, `{`, `[`, `,`) and counters ensure proper
/// nesting. The state machine ensures robust parsing of complex JSON structures while maintaining
/// a linked list of `Key` nodes.
///
/// # Examples
/// ```
/// use json_parser::{Key, ValueType, process_singleline_json_object};
///
/// let mut root_key = Key::new(String::new(), ValueType::ObjectType, String::new());
/// let json = r#"{"name": "John", "age": 30, "active": true, "data": {"id": 1}}"#;
/// process_singleline_json_object(json, &mut root_key).unwrap();
/// assert_eq!(root_key.get_n(), 4); // Four key-value pairs
/// root_key.pretty_print();
/// ```
///
/// # Notes
/// - The function assumes the input is a single-line JSON object without newlines, aligning with
///   the project's strategy to concatenate multi-line JSON into a single line for simplicity.
/// - Escaped character handling (e.g., `\"`, `\n`) is under development. Future updates will
///   include an `escaped` flag to correctly parse strings with embedded quotes.
/// - For large JSON inputs, consider streaming to optimize memory usage, though the current
///   approach is suitable for typical use cases.
/// - The `pairs` vector is used for debugging and may be removed in production to reduce memory.
///
/// # See Also
/// * `process_singleline_json_array`: Companion function for parsing JSON arrays.
/// * `Key`: Structure for storing key-value pairs and nested structures.
/// * `JsonObject`: Container for managing a linked list of `Key` nodes.
pub fn process_singleline_json_object(line: &str, key: &mut Key) {

    /*println!("--> Found opening brace: {}", line);*/
    
    // You have line of key/value pairs
    // You need to process each key/value pair, iterate through the line
    // Iterate through the line in a way that you get each individual key/value pair
    // A line could be...        
    // "dimensions": {"width": 1300, "height": 1300, "aspect_ratio": 1.0, "has_transparency": false, "palette": null, "color_info": {"red": 255, "green": 0, "blue": 0, "alpha": null}, "image_data": [255, 0, 0]}

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut line_of_peakable_chars = line.chars().peekable();

    let mut key_of_pair = String::new();
    let mut value_of_pair = String::new();

    let mut start_of_key = false;
    let mut colon_encountered = false;
    let mut opening_brace_encountered = false;
    let mut opening_bracket_encountered = false;

    let mut brace_count: usize = 0;
    let mut bracket_count: usize = 0;

    // Value types 
    let mut value_type_string_found = false;

    while let Some(ch) = line_of_peakable_chars.next() {

        // Retrieve the key
        if !colon_encountered {
                    
            if ch == '"' && !start_of_key && !colon_encountered {
            
                start_of_key = true;
            } else if ch == '"' && start_of_key && !colon_encountered {

                start_of_key = false;            
            } else if ch == ':' && !start_of_key && !colon_encountered {
            
                colon_encountered = true;
            } else if start_of_key && !colon_encountered {

                // Get the each individual char here and store it in string variable            
                key_of_pair.push(ch);
            }
        // Retrieve the value    
        } else if colon_encountered { // We have the key, now found value type and gather it

            // String value type
            if !opening_brace_encountered && (ch == '"' && !value_type_string_found) {

                value_type_string_found = true;

            } else if !opening_brace_encountered && value_type_string_found {
                
                if ch == '"' {

                    value_type_string_found = false;
                    colon_encountered = false;   
                } else {
                    value_of_pair.push(ch);
                }
                            
                if !colon_encountered {

                    pairs.push((key_of_pair.clone(), value_of_pair.clone()));

                    /**********************************/
                    /* Place to add string value type */
                    /**********************************/
                    let l_key = Box::new(Key::new(key_of_pair.clone(), ValueType::StringType, value_of_pair.clone()));
                    key.add_key(l_key);
                   /******************************************************************************************************************/ 

                    key_of_pair.clear();
                    value_of_pair.clear();
                }
            // JSON object value type    
            } else if !value_type_string_found && ch == '{' {

                //value.push(ch);

                //println!("-------------->>>>>>>>> {}", key_of_pair.clone());

                opening_brace_encountered = true;

                brace_count += 1;

            } else if opening_brace_encountered && ch == '}' {

                //value.push(ch);

                brace_count -= 1;

                if brace_count == 0 {

                    pairs.push((key_of_pair.clone(), value_of_pair.clone()));

                    //println!("--------------<<<<<<<<<<<<<<<<<<<<<<<<<<< {}", key_of_pair.clone());
                    //println!("--------------<<<<<<<<<<<<<<<<<<<<<<<<<<< {}", value_of_pair.clone());

                    /****************************************/
                    /* Place to add  JSON object value type */
                    /****************************************/
                    let mut l_key = Key::new(key_of_pair.clone(), ValueType::ObjectType, value_of_pair.clone());
                    process_singleline_json_object(value_of_pair.clone().as_str(), &mut l_key);
                    key.add_key(Box::new(l_key));
                   /*****************************************************************************************************/

                    key_of_pair.clear();
                    value_of_pair.clear();

                    opening_brace_encountered = false;
                    colon_encountered = false;
                }
            } else if !value_type_string_found && opening_brace_encountered {

                value_of_pair.push(ch);
            // JSON array value type    
            } else if !opening_brace_encountered && !value_type_string_found && ch == '[' {

                //value.push(ch);

                opening_bracket_encountered = true;

                bracket_count += 1;
            } else if opening_bracket_encountered && ch == ']' {

                //value.push(ch);

                bracket_count -= 1;

                if bracket_count == 0 {

                    pairs.push((key_of_pair.clone(), value_of_pair.clone()));

                    /****************************************/
                    /* Place to add  JSON array value type */
                    /****************************************/
                    let mut l_key = Key::new(key_of_pair.clone(), ValueType::ArrayType, String::new());
                    process_singleline_json_array(value_of_pair.clone().as_str(), &mut l_key);
                    key.add_key(Box::new(l_key));
                   /*****************************************************************************************************/

                    key_of_pair.clear();
                    value_of_pair.clear();

                    opening_bracket_encountered = false;
                    colon_encountered = false;
                }
            // JSON null value type and JSON numeric value type and JSON boolean value type
            } else if opening_bracket_encountered && !opening_brace_encountered && !value_type_string_found {

                value_of_pair.push(ch); 
                
            // JSON null value type and JSON numeric value type and JSON boolean value type    
            } else if !opening_bracket_encountered && !opening_brace_encountered && !value_type_string_found {

                if ch != ',' {
                    
                    if ch != ' ' {

                        value_of_pair.push(ch);
                    }
                } else {


                    /* See if value type is null, only allowed value is "null" it self */
                    if value_of_pair.clone() == "null" {
                        
                        let l_key = Box::new(Key::new(key_of_pair.clone(), ValueType::NullType, value_of_pair.clone()));
                        key.add_key(l_key);

                    } else if value_of_pair.clone() == "true" || value_of_pair.clone() == "false" {

                        let l_key = Box::new(Key::new(key_of_pair.clone(), ValueType::BooleanType, value_of_pair.clone()));
                        key.add_key(l_key);

                    // Value type is number    
                    } else {
                                                
                        let l_key = Box::new(Key::new(key_of_pair.clone(), ValueType::NumberType, value_of_pair.clone()));
                        key.add_key(l_key);
                    }
                        
                    pairs.push((key_of_pair.clone(), value_of_pair.clone()));
                    
                    key_of_pair.clear();
                    value_of_pair.clear(); 
                    
                    colon_encountered = false;
                }
            }
        }
    }

    // Get the last key-value pair
    if key_of_pair.len() > 0 && value_of_pair.len() > 0 {

        if value_of_pair.clone() == "null" {
            
            let l_key = Box::new(Key::new(key_of_pair.clone(), ValueType::NullType, value_of_pair.clone()));
            key.add_key(l_key);

        } else if value_of_pair.clone() == "true" || value_of_pair.clone() == "false" {

            let l_key = Box::new(Key::new(key_of_pair.clone(), ValueType::BooleanType, value_of_pair.clone()));
            key.add_key(l_key);
        // Value type is Number    
        } else {

            let l_key = Box::new(Key::new(key_of_pair.clone(), ValueType::NumberType, value_of_pair.clone()));
            key.add_key(l_key);   
        }

        pairs.push((key_of_pair.clone(), value_of_pair.clone()));
                    
        key_of_pair.clear();
        value_of_pair.clear();         
    }
        
    // Process the pairs
    /*for (key, value) in pairs {
        println!("Key: {}, Value: {}", key, value);
    }*/
}

/// Recursively parses a JSON object that may span multiple lines.
///
/// This function processes a subset of lines from the file content, identified by start and end line numbers.
/// It parses key-value pairs within the object, handling nested objects and arrays by calling the
/// appropriate processing functions recursively.
///
/// # Arguments
/// * `l1` - The starting line number (index) of the multi-line object block.
/// * `l2` - The ending line number (index) of the multi-line object block.
/// * `file_content` - A reference to the `FileContent` struct containing the source text.
/// * `key` - A mutable reference to the parent `Key` to which the parsed nested keys of this object will be added.
pub fn process_multiline_json_object(l1: usize, l2: usize, file_content: &FileContent, key: &mut Key) {

    if l2 == 0 || ((l1 + 1) >= (l2 - 1)) {
        
    }

    /*println! ("In the process_multiline_json_object function, l1: {}, l2: {}", l1, l2);*/

    let mut starting_line_number: usize = 0; 
    let mut ending_line_number: usize = 0;

    let opening_brace_reg_expr = Regex::new(JSON_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let closing_brace_reg_expr = Regex::new(JSON_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();

    let key_regex = Regex::new(JSON_KEY_REG_EXPR_PATTERN).unwrap();
    let value_string_type_regex = Regex::new(JSON_VALUE_TYPE_STRING_REG_EXPR_PATTERN).unwrap();
    let quoted_content_regex = Regex::new(JSON_QUOTED_CONTENT_PATTERN).unwrap();
    let value_numeric_type_regex = Regex::new(JSON_VALUE_TYPE_NUMERIC_PATTERN).unwrap();
    let value_null_type_regex = Regex::new(JSON_VALUE_TYPE_NULL_PATTERN).unwrap();
    let value_true_type_regex = Regex::new(JSON_VALUE_TYPE_TRUE_PATTERN).unwrap();
    let value_false_type_regex = Regex::new(JSON_VALUE_TYPE_FALSE_PATTERN).unwrap();
    let value_json_array_type_regex = Regex::new(JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap(); // Matches `: ["` at the start of a JSON array value 
    let closing_square_bracket_of_json_array_type_regex = Regex::new(JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap();
    let single_line_json_array_type_regex = Regex::new(JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN).unwrap();
    let single_line_json_array_type_value_string_regex = Regex::new(JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN_VALUE_STRING).unwrap();
    let single_line_object_type_key_name_with_opening_closing_brace_regex = Regex::new(JSON_SINGLE_LINE_OBJECT_TYPE_KEY_NAME_WITH_OPENING_CLOSING_BRACE_PATTERN).unwrap();

    let value_opening_brace_reg_expr = Regex::new(JSON_VALUE_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let value_closing_brace_reg_expr = Regex::new(JSON_VALUE_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();

    let mut json_multi_line_array_opening_closing_bracket_count: usize = 0; 
    let mut json_multi_line_object_opening_closing_brace_count: usize = 0;

    // Track current key being processed
    let mut current_key_name: Option<String> = None;
    let mut key_value_pair_complete = false;

    let mut i: usize = l1 + 1;

    loop {
        
        file_content.get_line_by_index(i).map(|line| {

            // Process the line here
            // For example, you can print it or parse it
            /*println!("Processing line {}: {}", i, line);*/

            if opening_brace_reg_expr.is_match(line) {
                            
                /*println! ("--> Found opening brace: {} - {}", line, i);*/

                if json_multi_line_array_opening_closing_bracket_count == 0 {

                    if starting_line_number == 0 {

                        starting_line_number = /*file_content.get_current_line_index()*/ i;
                    }

                    json_multi_line_object_opening_closing_brace_count += 1;
                }

            } else if closing_brace_reg_expr.is_match(line) {
            
                /*println! ("--> Found closing brace: {} - {}", line, i);*/

                if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count > 0 {

                    json_multi_line_object_opening_closing_brace_count -= 1;

                    if json_multi_line_object_opening_closing_brace_count == 0 {

                        ending_line_number = /*file_content.get_current_line_index()*/ i;

                        let mut l_key = Key::new(String::new(), ValueType::ObjectType, String::new());
                                            
                        // Process Multilne JSON object type here...                                                
                        process_multiline_json_object(starting_line_number, ending_line_number, file_content, &mut l_key);

                        key.add_key(Box::new(l_key));

                        // Reset the starting and ending line numbers for the next multiline array
                        starting_line_number = 0;
                        ending_line_number = 0;
                    }            
                }
            // Check if line has a key...    `                  
            } if let Some(captures) = key_regex.captures(line) { 

                if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {
                    // Extract key name
                    captures.get(0).map(|key_match| {
                        if let Some(quoted_content_match) = quoted_content_regex.captures(key_match.as_str()) {
                            let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());
                            /*println!("Found quoted string key: ----> {}", un_quoted_string);*/
                            current_key_name = Some(un_quoted_string.to_string());
                        }                    
                    });
                }

                // Check for different value types...
                // json array, json ibject, string, numeric, boolean, none

                // Check if value type is string...
                if let Some(captures) = value_string_type_regex.captures(line) {

                    /*if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {
                        // Extract key name
                        captures.get(0).map(|key_match| {
                            if let Some(quoted_content_match) = quoted_content_regex.captures(key_match.as_str()) {
                                let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());
                                println!("Found quoted string key: ----> {}", un_quoted_string);
                                current_key_name = Some(un_quoted_string.to_string());
                            }                    
                        });
                    }*/

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        // We know that value type is string, now get the value text...
                        captures.get(0).map(|value_match| {
                            // Get value text...                    
                            if let Some (quoted_content_match) = quoted_content_regex.captures( value_match.as_str()) {
                                let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());

                                /*println!("Found quoted string value: {}", un_quoted_string); // Prints: test.png*/

                                // Create complete key-value pair for string type
                                if let Some(key_name) = current_key_name.take() {
                                    let l_key = Box::new(Key::new(key_name, ValueType::StringType, un_quoted_string.to_string()));
                                    key.add_key(l_key);
                                    key_value_pair_complete = true;
                                }
                            }                                                
                        });
                    }
                // Check for numeric value    
                } else if let Some(captures) = value_numeric_type_regex.captures(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {
                    
                        /*println! ("Found numeric value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for numeric type
                        if let Some(key_name) = current_key_name.take() {
                            let l_key = Box::new(Key::new(key_name, ValueType::NumberType, captures.get(1).map_or("", |m| m.as_str()).to_string()));
                            key.add_key(l_key);
                            key_value_pair_complete = true;
                        }
                    }
                // Check for value which is starting of json array    
                } else if value_json_array_type_regex.is_match(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 {

                        json_multi_line_array_opening_closing_bracket_count += 1; 

                        if starting_line_number == 0 {

                            starting_line_number = /*file_content.get_current_line_index()*/ i;
                        }                       
                    }
                /* ************************************************************************************************************************** */                    
                /*                         Check for single line object, match for the very first and very last brace                         */
                /* ************************************************************************************************************************** */    
                // Has not yet implemented
                /* ************************************************************************************************************************** */ 
                } else if value_opening_brace_reg_expr.is_match(line) && value_closing_brace_reg_expr.is_match(line) {                    
                    if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {                                                                        
                        if let Some(key_name) = current_key_name.take() {                            
                            if let Some(captures) = single_line_object_type_key_name_with_opening_closing_brace_regex.captures(line) {                                                                                                
                                let mut l_key = Box::new(Key::new(key_name, ValueType::ObjectType, String::new()));
                                process_singleline_json_object(captures.get(2).unwrap().as_str(), &mut l_key);                                
                                key.add_key(l_key);
                                key_value_pair_complete = true;                            
                            }
                            
                        }
                    }  
                /* ************************************************************************************************************************** */                    
                /*                         Check for single line Array, match for the very first and very last bracket                        */
                /* ************************************************************************************************************************** */ 
                // Check for single line json array, though it works but this implementation is just for one specific case. 
                /* ************************************************************************************************************************** */                       
                } else if let Some(captures) = single_line_json_array_type_regex.captures(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {
                        
                        //println! ("----------------------------------->############################### Found single line json array: {}", line);
                        
                        /*captures.get(0).map(|value_match| {
                           
                            // Create complete key-value pair for string type
                            if let Some(key_name) = current_key_name.take() {
                                let l_key = Box::new(Key::new(key_name, ValueType::StringType, value_match.as_str().to_string()));
                                key.add_key(l_key);
                                key_value_pair_complete = true;
                            }                           
                        });*/
                        
                        // We know that value type is string, now get the value text...
                        captures.get(0).map(|value_match| {
                            // Get value text...                    
                            if let Some (quoted_content_match) = single_line_json_array_type_value_string_regex.captures( value_match.as_str()) {
                                let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());
                        
                                /*println!("Found quoted string value: {}", un_quoted_string);*/ // Prints: test.png
                        
                                // Create complete key-value pair for string type
                                if let Some(key_name) = current_key_name.take() {
                                    /*
                                        // ValueType::ArrayType was ValueType::StringType
                                        Here we are dealing with single line json array type like "image_data": [1, 2, 3]
                                    */
                                    let l_key = Box::new(Key::new(key_name, ValueType::ArrayType, un_quoted_string.to_string()));
                                    key.add_key(l_key);
                                    key_value_pair_complete = true;
                                }
                            }                                                                                                    
                        });
                    }  
                } else if value_null_type_regex.is_match(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        /*println! ("Found null value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for null type
                        if let Some(key_name) = current_key_name.take() {
                            let l_key = Box::new(Key::new(key_name, ValueType::NullType, "null".to_string()));
                            key.add_key(l_key);
                            key_value_pair_complete = true;
                        }
                    }
                } else if value_true_type_regex.is_match(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        /*println! ("Found true value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for true type
                        if let Some(key_name) = current_key_name.take() {
                            let l_key = Box::new(Key::new(key_name, ValueType::BooleanType, "true".to_string()));
                            key.add_key(l_key);
                            key_value_pair_complete = true;
                        }
                    }
                } else if value_false_type_regex.is_match(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        /*println! ("Found false value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for false type
                        if let Some(key_name) = current_key_name.take() {
                            let l_key = Box::new(Key::new(key_name, ValueType::BooleanType, "false".to_string()));
                            key.add_key(l_key);
                            key_value_pair_complete = true;
                        }
                    }
                }
            // Chek if line consists of just the closing bracket of json array    
            } else if /*let Some(_captures) = closing_square_bracket_of_json_array_type_regex.captures(line)*/ closing_square_bracket_of_json_array_type_regex.is_match(line) {
                
                if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count > 0 {
                    
                    json_multi_line_array_opening_closing_bracket_count -= 1;

                    if json_multi_line_array_opening_closing_bracket_count == 0 {

                        ending_line_number = /*file_content.get_current_line_index()*/ i;

                        // Process Multilne JSON object type here...
                        if let Some(key_name) = current_key_name.take() {
                            let mut l_key = Key::new(key_name, ValueType::ArrayType, String::new());
                            process_multiline_json_array(starting_line_number, ending_line_number, &file_content, &mut l_key);
                            let l_key = Box::new(l_key);
                            key.add_key(l_key);
                            key_value_pair_complete = true;   
                        }
                                                
                        /*process_multiline_json_array(starting_line_number, ending_line_number, file_content);*/

                        // Reset the starting and ending line numbers for the next multiline array
                        starting_line_number = 0;
                        ending_line_number = 0;                        
                    }
                }
            } 
        });
        
        i = i + 1;

        if i > (l2 - 1) {

            break;
        }        
    }
}

/// Recursively parses a JSON array that may span multiple lines.
///
/// This function processes a subset of lines from the file content, identified by start and end line numbers.
/// It parses elements within the array, handling nested objects, strings, and numbers. Nested objects
/// are processed by recursively calling `process_multiline_json_object`.
///
/// # Arguments
/// * `l1` - The starting line number (index) of the multi-line array block.
/// * `l2` - The ending line number (index) of the multi-line array block.
/// * `file_content` - A reference to the `FileContent` struct containing the source text.
/// * `key` - A mutable reference to the parent `Key` (which represents the array) to which the parsed elements will be added as nested keys.
pub fn process_multiline_json_array(l1: usize, l2: usize, file_content: &FileContent, key: &mut Key) {

    if l2 == 0 || ((l1 + 1) >= (l2 - 1)) {
        
    }

    /*println! ("In the process_multiline_json_array function, l1: {}, l2: {}", l1, l2);*/

    let mut starting_line_number: usize = 0; 
    let mut ending_line_number: usize = 0;

    let opening_brace_reg_expr = Regex::new(JSON_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let closing_brace_reg_expr = Regex::new(JSON_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();

    let key_regex = Regex::new(JSON_KEY_REG_EXPR_PATTERN).unwrap();
    // Value type JSON array, value_json_array_type_regex
    let value_json_array_type_regex = Regex::new(JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap(); // Matches `: ["` at the start of a JSON array value    
    let closing_square_bracket_of_json_array_type_regex = Regex::new(JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap();
    let value_string_type_regex = Regex::new(JSON_VALUE_TYPE_STRING_REG_EXPR_PATTERN).unwrap();
    let quoted_content_regex = Regex::new(JSON_QUOTED_CONTENT_PATTERN).unwrap();
    let value_numeric_type_regex = Regex::new(JSON_VALUE_TYPE_NUMERIC_PATTERN).unwrap();
    let value_null_type_regex = Regex::new(JSON_VALUE_TYPE_NULL_PATTERN).unwrap();
    let value_true_type_regex = Regex::new(JSON_VALUE_TYPE_TRUE_PATTERN).unwrap();
    let value_false_type_regex = Regex::new(JSON_VALUE_TYPE_FALSE_PATTERN).unwrap();
    let single_line_json_array_type_regex = Regex::new(JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN).unwrap();
    let single_line_json_array_type_value_string_regex = Regex::new(JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN_VALUE_STRING).unwrap();
    let single_line_object_type_key_name_with_opening_closing_brace_regex = Regex::new(JSON_SINGLE_LINE_OBJECT_TYPE_KEY_NAME_WITH_OPENING_CLOSING_BRACE_PATTERN).unwrap();

    let value_opening_brace_reg_expr = Regex::new(JSON_VALUE_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let value_closing_brace_reg_expr = Regex::new(JSON_VALUE_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();

    let mut json_multi_line_array_opening_closing_bracket_count: usize = 0; 
    let mut json_multi_line_object_opening_closing_brace_count: usize = 0;

    // Track current key being processed
    let mut current_key_name: Option<String> = None;
    let mut key_value_pair_complete = false;

    /*let key =  Key {
        name: String::new(),
        value_type: ValueType::StringType,
        value: String::new(),
        next: None,
        prev: None,
    };

    let mut json_object = JsonObject {
        ptr: None,
        n: 0,
    };*/

    let mut i: usize = l1 + 1;

    loop {

        file_content.get_line_by_index(i).map(|line| {

            // Process the line here
            // For example, you can print it or parse it
            /*println!("Processing line {}: {}", i, line);*/

            if opening_brace_reg_expr.is_match(line) {
                            
                /*println! ("--> Found opening brace: {} - {}", line, i);*/

                if json_multi_line_array_opening_closing_bracket_count == 0 {

                    if starting_line_number == 0 {

                        starting_line_number = /*file_content.get_current_line_index()*/ i;
                    }

                    json_multi_line_object_opening_closing_brace_count += 1;
                }

            } else if closing_brace_reg_expr.is_match(line) {
            
                /*println! ("--> Found closing brace: {} - {}", line, i);*/

                if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count > 0 {

                    json_multi_line_object_opening_closing_brace_count -= 1;

                    if json_multi_line_object_opening_closing_brace_count == 0 {

                        let mut l_key = Key::new(String::new(), ValueType::ObjectType, String::new());

                        ending_line_number = /*file_content.get_current_line_index()*/ i;
                        // Process Multilne JSON object type here...                                                
                        process_multiline_json_object(starting_line_number, ending_line_number, file_content, &mut l_key);

                        key.add_key(Box::new(l_key));

                        //key.get_json_object().unwrap().as_ref().add_key( Box::new(l_key));
                                                                                                                                            
                        // Process Multilne JSON object type here...                                                
                        /*process_multiline_json_object(starting_line_number, ending_line_number, file_content);*/

                        // Reset the starting and ending line numbers for the next multiline array
                        starting_line_number = 0;
                        ending_line_number = 0;
                    }
                }

            // Check if line has a key...    
            } else if let Some(captures) = key_regex.captures(line) { 

                if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {
                    // Extract key name
                    captures.get(0).map(|key_match| {
                        if let Some(quoted_content_match) = quoted_content_regex.captures(key_match.as_str()) {
                            let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());
                            /*println!("Found quoted string key: ----> {}", un_quoted_string);*/
                            current_key_name = Some(un_quoted_string.to_string());
                        }                    
                    });
                }
                
                // Check for different value types...
                // json array, json ibject, string, numeric, boolean, none
                
                // Check for json multiline array
                if value_json_array_type_regex.is_match(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 {

                        json_multi_line_array_opening_closing_bracket_count += 1; 

                        if starting_line_number == 0 {

                            starting_line_number = /*file_content.get_current_line_index()*/ i;
                        }                       
                    }                 
                /* ************************************************************************************************************************** */                    
                /*                         Check for single line object, match for the very first and very last brace                         */
                /* ************************************************************************************************************************** */    
                // Has not yet implemented
                /* ************************************************************************************************************************** */ 
                } else if value_opening_brace_reg_expr.is_match(line) && value_closing_brace_reg_expr.is_match(line) {                    
                    if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {                                                                        
                        if let Some(key_name) = current_key_name.take() {                            
                            if let Some(captures) = single_line_object_type_key_name_with_opening_closing_brace_regex.captures(line) {                                                                                                
                                let mut l_key = Box::new(Key::new(key_name, ValueType::ObjectType, String::new()));
                                process_singleline_json_object(captures.get(2).unwrap().as_str(), &mut l_key);                                
                                key.add_key(l_key);
                                key_value_pair_complete = true;
                            }
                        }
                    }                                   
                /* ************************************************************************************************************************** */                    
                /*                         Check for single line Array, match for the very first and very last bracket                        */
                /* ************************************************************************************************************************** */ 
                // Check for single line json array, though it works but this implementation is just for one specific case. 
                /* ************************************************************************************************************************** */   
                } else if let Some(captures) = single_line_json_array_type_regex.captures(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        //println! ("----------------------------------->********************* Found single line json array: {}", line);

                        /*captures.get(0).map(|value_match| {
                           
                            // Create complete key-value pair for string type
                            if let Some(key_name) = current_key_name.take() {
                                let l_key = Box::new(Key::new(key_name, ValueType::StringType, value_match.as_str().to_string()));
                                key.add_key(l_key);
                                key_value_pair_complete = true;
                            }                           
                        });*/

                        // We know that value type is string, now get the value text...
                        captures.get(0).map(|value_match| {
                            // Get value text...                    
                            if let Some (quoted_content_match) = single_line_json_array_type_value_string_regex.captures( value_match.as_str()) {
                                let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());
                        
                                /*println!("Found quoted string value: {}", un_quoted_string);*/ // Prints: test.png
                        
                                // Create complete key-value pair for string type
                                if let Some(key_name) = current_key_name.take() {

                                    let l_key = Box::new(Key::new(key_name, ValueType::StringType, un_quoted_string.to_string()));
                                    key.add_key(l_key);
                                    key_value_pair_complete = true;
                                }
                            }                                                                                                    
                        });
                    }                
                // Check for string value    
                } else if let Some(captures) = value_string_type_regex.captures(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        // We know that value type is string, now get the value text...
                        captures.get(0).map(|value_match| {
                            // Get value text...                    
                            if let Some (quoted_content_match) = quoted_content_regex.captures( value_match.as_str()) {
                                let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());

                                /*println!("Found quoted string value: {}", un_quoted_string);*/ // Prints: test.png

                                // Create complete key-value pair for string type
                                if let Some(key_name) = current_key_name.take() {
                                    let l_key = Box::new(Key::new(key_name, ValueType::StringType, un_quoted_string.to_string()));
                                    key.add_key(l_key);
                                    key_value_pair_complete = true;
                                }
                            }                                                
                        });
                    }
                // Check for numeric value    
                } else if let Some(captures) = value_numeric_type_regex.captures(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {
                    
                        /*println! ("Found numeric value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for numeric type
                        if let Some(key_name) = current_key_name.take() {
                            let l_key = Box::new(Key::new(key_name, ValueType::NumberType, captures.get(1).map_or("", |m| m.as_str()).to_string()));
                            key.add_key(l_key);
                            key_value_pair_complete = true;
                        }
                    }
                } else if value_null_type_regex.is_match(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        /*println! ("Found null value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for null type
                        if let Some(key_name) = current_key_name.take() {
                            let l_key = Box::new(Key::new(key_name, ValueType::NullType, "null".to_string()));
                            key.add_key(l_key);
                            key_value_pair_complete = true;
                        }
                    }
                } else if value_true_type_regex.is_match(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        /*println! ("Found true value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for true type
                        if let Some(key_name) = current_key_name.take() {
                            let l_key = Box::new(Key::new(key_name, ValueType::BooleanType, "true".to_string()));
                            key.add_key(l_key);
                            key_value_pair_complete = true;
                        }
                    }
                } else if value_false_type_regex.is_match(line) {

                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {

                        /*println! ("Found false value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for false type
                        if let Some(key_name) = current_key_name.take() {
                            let l_key = Box::new(Key::new(key_name, ValueType::BooleanType, "false".to_string()));
                            key.add_key(l_key);
                            key_value_pair_complete = true;
                        }
                    }
                }
            // Chek if line consists of just the closing bracket of json array    
            } else if /*let Some(_captures) = closing_square_bracket_of_json_array_type_regex.captures(line)*/ closing_square_bracket_of_json_array_type_regex.is_match(line) {
                
                if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count > 0 {
                    
                    json_multi_line_array_opening_closing_bracket_count -= 1;

                    if json_multi_line_array_opening_closing_bracket_count == 0 {

                        ending_line_number = /*file_content.get_current_line_index()*/ i;

                        // Process Multilne JSON object type here...

                        // Process Multilne JSON object type here...
                        if let Some(key_name) = current_key_name.take() {
                            let mut l_key = Key::new(key_name, ValueType::ArrayType, String::new());
                            process_multiline_json_array(starting_line_number, ending_line_number, &file_content, &mut l_key);
                            let l_key = Box::new(l_key);
                            key.add_key(l_key);
                            key_value_pair_complete = true;   
                        }
                        
                        /*process_multiline_json_array(starting_line_number, ending_line_number, file_content);*/

                        // Reset the starting and ending line numbers for the next multiline array
                        starting_line_number = 0;
                        ending_line_number = 0;                        
                    }
                }
            }              
        });

        i = i + 1;

        if i > (l2 - 1) {

            break;
        }
    }    
}

pub fn json_main_single_line(file_name: &str) -> Result<Option<Box<JsonObject>>, io::Error> {
    let mut json_object = JsonObject::new();
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let single_line = contents.lines().collect::<Vec<&str>>().join("").trim().to_string();

    if single_line.is_empty() {
        return Ok(None);
    }

    let mut root_key = Key::new(String::new(), ValueType::ObjectType, String::new());
    let inner_content = remove_outer_braces(&single_line)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Move map_err to the Result returned by process_singleline_json_object
    process_singleline_json_object(inner_content, &mut root_key)
        /*.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?*/;
    json_object.add_key(Box::new(root_key));

    if json_object.get_n() > 0 {
        Ok(Some(Box::new(json_object)))
    } else {
        Ok(None)
    }
}

/*
pub fn json_main_single_line_old(file_name: &str) -> Result<Option<Box<JsonObject>>, io::Error> {
    let mut json_object = JsonObject::new();
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let single_line = contents.lines().collect::<Vec<&str>>().join("").trim().to_string();

    if single_line.is_empty() {
        return Ok(None);
    }

    let mut root_key = Key::new(String::new(), ValueType::ObjectType, String::new());
    let inner_content = remove_outer_braces(&single_line)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    process_singleline_json_object(inner_content, &mut root_key)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    json_object.add_key(Box::new(root_key));

    if json_object.get_n() > 0 {
        Ok(Some(Box::new(json_object)))
    } else {
        Ok(None)
    }
}
    */

pub fn json_main_single_line_older(file_name: &str) -> Result<Option<Box<JsonObject>>, io::Error> {

    let mut json_object = JsonObject::new();

    let mut file_content = FileContent::from_file(file_name)?;

    loop {
        if let Some(line) = file_content.go_to_next_line() {
            let mut l_key = Box::new(Key::new(String::new(), ValueType::ObjectType , String::new()));
            process_singleline_json_object(line, &mut l_key);
            json_object.add_key(l_key);
        } else {

            break; // Exit when we reach the end
        }
    }
    
    if json_object.get_n() > 0 {
        Ok(Some(Box::new(json_object)))
    } else {
        Ok(None)
    }
}
  

/// The main entry point for parsing a JSON file.
///
/// This function reads a file, iterates through its lines, and orchestrates the parsing process.
/// It identifies top-level key-value pairs and delegates the parsing of complex, multi-line
/// structures (objects and arrays) to the appropriate helper functions.
///
/// # Arguments
/// * `file_name` - A string slice that holds the path to the JSON file to be parsed.
///
/// # Returns
/// * `Ok(Some(Box<JsonObject>))` if parsing is successful, containing the root `JsonObject`.
/// * `Ok(None)` if the file is empty or does not form a valid object (though current implementation always returns a `JsonObject`).
/// * `Err(io::Error)` if the file cannot be read.
pub fn json_main(file_name: &str) -> Result<Option<Box<JsonObject>>, io::Error> {
    
    // Counter when it is zero means it is not set
    let mut json_multi_line_array_opening_closing_bracket_count: usize = 0;
    let mut json_multi_line_object_opening_closing_brace_count: usize = 0;

    let mut starting_line_number: usize = 0;
    let mut ending_line_number: usize = 0;

    let mut file_content = FileContent::from_file(file_name)?;
    
    let key_regex = Regex::new(JSON_KEY_REG_EXPR_PATTERN).unwrap();
    let value_string_type_regex = Regex::new(JSON_VALUE_TYPE_STRING_REG_EXPR_PATTERN).unwrap();
    let quoted_content_regex = Regex::new(JSON_QUOTED_CONTENT_PATTERN).unwrap();
    let value_numeric_type_regex = Regex::new(JSON_VALUE_TYPE_NUMERIC_PATTERN).unwrap();
    let value_null_type_regex = Regex::new(JSON_VALUE_TYPE_NULL_PATTERN).unwrap();
    let value_true_type_regex = Regex::new(JSON_VALUE_TYPE_TRUE_PATTERN).unwrap();
    let value_false_type_regex = Regex::new(JSON_VALUE_TYPE_FALSE_PATTERN).unwrap();
    let value_json_array_type_regex = Regex::new(JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap();
    let closing_square_bracket_of_json_array_type_regex = Regex::new(JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap();
    let opening_brace_reg_expr = Regex::new(JSON_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let closing_brace_reg_expr = Regex::new(JSON_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();
    let value_opening_brace_reg_expr = Regex::new(JSON_VALUE_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let value_closing_brace_reg_expr = Regex::new(JSON_VALUE_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();
    let single_line_object_type_key_name_with_opening_closing_brace_regex = Regex::new(JSON_SINGLE_LINE_OBJECT_TYPE_KEY_NAME_WITH_OPENING_CLOSING_BRACE_PATTERN).unwrap();

    let mut json_object = JsonObject::new();
    
    // Track current key being processed
    let mut current_key_name: Option<String> = None;
    let mut key_value_pair_complete = false;
    
    loop {
        if let Some(line) = file_content.go_to_next_line() {
            
            if line == JSON_OPENIING_BRACE {
                // Handle opening brace
            } else if line == JSON_CLOSING_BRACE {
                // Handle closing brace
            } else if let Some(captures) = key_regex.captures(line) {
                
                if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {
                    // Extract key name
                    captures.get(0).map(|key_match| {
                        if let Some(quoted_content_match) = quoted_content_regex.captures(key_match.as_str()) {
                            let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());
                            /*println!("Found quoted string key: ----> {}", un_quoted_string);*/
                            current_key_name = Some(un_quoted_string.to_string());
                        }                    
                    });
                }
                
                // Check for string value
                if let Some(captures) = value_string_type_regex.captures(line) {                    
                    if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {
                        captures.get(0).map(|value_match| {                        
                            if let Some(quoted_content_match) = quoted_content_regex.captures(value_match.as_str()) {
                                let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());
                                /*println!("Found quoted string value: {}", un_quoted_string);*/
                                
                                // Create complete key-value pair for string type
                                if let Some(key_name) = current_key_name.take() {
                                    let key = Box::new(Key::new(key_name, ValueType::StringType, un_quoted_string.to_string()));
                                    json_object.add_key(key);
                                    key_value_pair_complete = true;
                                }
                            }                                                
                        });                                        
                    }
                    
                // Check for numeric value            
                } else if let Some(captures) = value_numeric_type_regex.captures(line) {
                    if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {
                        let numeric_value = captures.get(1).map_or("", |m| m.as_str()).to_string();
                        
                        // Create complete key-value pair for numeric type
                        if let Some(key_name) = current_key_name.take() {
                            let key = Box::new(Key::new(key_name, ValueType::NumberType, numeric_value));
                            json_object.add_key(key);
                            key_value_pair_complete = true;
                        }
                    }
                    
                // Check for array start of multiline array
                } else if value_json_array_type_regex.is_match(line) {
                    if json_multi_line_object_opening_closing_brace_count == 0 {
                        if json_multi_line_array_opening_closing_bracket_count == 0 {
                            starting_line_number = file_content.get_current_line_index();
                        }
                        json_multi_line_array_opening_closing_bracket_count += 1;
                    }
                /* ************************************************************************************************************************** */                    
                /*                         Check for single line object, match for the very first and very last brace                         */
                /* ************************************************************************************************************************** */
                } else if value_opening_brace_reg_expr.is_match(line) && value_closing_brace_reg_expr.is_match(line) {                    
                    if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {                                                                        
                        if let Some(key_name) = current_key_name.take() {                            
                            if let Some(captures) = single_line_object_type_key_name_with_opening_closing_brace_regex.captures(line) {                                                                                                
                                let mut key = Box::new(Key::new(key_name, ValueType::ObjectType, String::new()));
                                process_singleline_json_object(captures.get(2).unwrap().as_str(), &mut key);                                
                                json_object.add_key(key);
                                key_value_pair_complete = true;
                            }
                        }
                    }
                /* ************************************************************************************************************************** */                    
                /*                         Check for single line Array, match for the very first and very last bracket                        */
                /* ************************************************************************************************************************** */ 
                // Has not yet implemented
                /* ************************************************************************************************************************** */  
                } else if value_null_type_regex.is_match(line) {
                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {
                        /*println! ("Found null value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for null type
                        if let Some(key_name) = current_key_name.take() {
                            let key = Box::new(Key::new(key_name, ValueType::NullType, "null".to_string()));
                            json_object.add_key(key);
                            key_value_pair_complete = true;
                        }
                    }
                } else if value_true_type_regex.is_match(line) {
                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {
                        /*println! ("Found true value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for true type
                        if let Some(key_name) = current_key_name.take() {
                            let key = Box::new(Key::new(key_name, ValueType::BooleanType, "true".to_string()));
                            json_object.add_key(key);
                            key_value_pair_complete = true;
                        }
                    }
                } else if value_false_type_regex.is_match(line) {
                    if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count == 0 {
                        /*println! ("Found false value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        // Create complete key-value pair for false type
                        if let Some(key_name) = current_key_name.take() {
                            let key = Box::new(Key::new(key_name, ValueType::BooleanType, "false".to_string()));
                            json_object.add_key(key);
                            key_value_pair_complete = true;
                        }
                    }
                }

            } else if closing_square_bracket_of_json_array_type_regex.is_match(line) {
                if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count > 0 {
                    json_multi_line_array_opening_closing_bracket_count -= 1;

                    if json_multi_line_array_opening_closing_bracket_count == 0 {
                        ending_line_number = file_content.get_current_line_index();

                        // Create complete key-value pair for array type
                        if let Some(key_name) = current_key_name.take() {
                            /*let key = Box::new(Key::new(key_name, ValueType::ArrayType, String::new()));
                            json_object.add_key(key);
                            key_value_pair_complete = true;*/
                        
                            let mut key = Key::new(key_name, ValueType::ArrayType, String::new());
                            process_multiline_json_array(starting_line_number, ending_line_number, &file_content, &mut key);
                            let key = Box::new(key);
                            json_object.add_key(key);
                            key_value_pair_complete = true;
                        }

                        starting_line_number = 0;
                        ending_line_number = 0;
                    }
                }
                
            } else if opening_brace_reg_expr.is_match(line) {
                /*println!("--> Found opening brace: {}", line);*/

                if json_multi_line_array_opening_closing_bracket_count == 0 {
                    if starting_line_number == 0 {
                        starting_line_number = file_content.get_current_line_index();
                    }
                    json_multi_line_object_opening_closing_brace_count += 1;
                }

            } else if closing_brace_reg_expr.is_match(line) {
                /*println!("--> Found closing brace: {}", line);*/

                if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count > 0 {
                    json_multi_line_object_opening_closing_brace_count -= 1;

                    if json_multi_line_object_opening_closing_brace_count == 0 {
                        ending_line_number = file_content.get_current_line_index();

                        // Create complete key-value pair for object type
                        if let Some(key_name) = current_key_name.take() {
                            let key = Box::new(Key::new(key_name, ValueType::ObjectType, String::new()));
                            json_object.add_key(key);
                            key_value_pair_complete = true;
                        
                            /*process_multiline_json_object(starting_line_number, ending_line_number, &file_content);*/
                        }

                        starting_line_number = 0;
                        ending_line_number = 0;
                    }
                }
            }
        } else {
            break; // Exit when we reach the end
        }
    }

    if json_object.get_n() > 0 {
        Ok(Some(Box::new(json_object)))
    } else {
        Ok(None)
    }
}

// It is here just for documentation purpose
pub fn json_main_old() -> /*Result<(), io::Error>*/ /*Option<Box<JsonObject>>*/ Result<Option<Box<JsonObject>>, io::Error> {
        
     // Counuterm when it is zero means it is not set
     let mut json_multi_line_array_opening_closing_bracket_count: usize = 0;
     let mut json_multi_line_object_opening_closing_brace_count: usize = 0;

     let mut starting_line_number: usize = 0;
     let mut ending_line_number: usize = 0;

     //let mut starting_line_number_of_multiline_array: usize = 0; 
     //let mut ending_line_number_of_multiline_array: usize = 0;

     /* ^ use `.ok()?` if you want to discard the `Result<Infallible, std::io::Error>` error information */
     let mut file_content = FileContent::from_file("src/png.json")?;
     
     let key_regex = Regex::new(JSON_KEY_REG_EXPR_PATTERN).unwrap();

     let value_string_type_regex = Regex::new(JSON_VALUE_TYPE_STRING_REG_EXPR_PATTERN).unwrap();
     let quoted_content_regex = Regex::new(JSON_QUOTED_CONTENT_PATTERN).unwrap();

     let value_numeric_type_regex = Regex::new(JSON_VALUE_TYPE_NUMERIC_PATTERN).unwrap();

     // Value type JSON array, value_json_array_type_regex
     let value_json_array_type_regex = Regex::new(JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap(); // Matches `: ["` at the start of a JSON array value
     let closing_square_bracket_of_json_array_type_regex = Regex::new(JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap();

    let opening_brace_reg_expr = Regex::new(JSON_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let closing_brace_reg_expr = Regex::new(JSON_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();

    let value_opening_brace_reg_expr = Regex::new(JSON_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let value_closing_brace_reg_expr = Regex::new(JSON_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();


     /*file_content.count_lines();*/

     let mut json_object = JsonObject::new();

     //let mut key: Option<Box<Key>> = None;

     // Will this key will get garbage collected when it is gone of of scope....
     let mut key: Box<Key> = Box::new(Key::new(String::new(), ValueType::StringType, String::new()));
     
     loop {

         if let Some(line) = file_content.go_to_next_line() {

             //println!("{}", line);

             if line == JSON_OPENIING_BRACE {
                 /*println!("Found opening brace: {}", line);*/
             } else if line == JSON_CLOSING_BRACE {
                 /*println!("Found closing brace: {}", line);*/

                // Check for key-value pairs in the JSON line  

             // Check if line has a key... 
             } else if let Some(captures) = key_regex.captures(line) { 

                if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {

                    // Yes line has key....
                    captures.get(0).map(|key_match| {

                        // One way to get the key text...
                        /*let key = key_match.as_str();                
                        println!("Found key: {}", key);*/

                        // Another way to get the key text...                        
                        // Get key text...
                        if let Some (quoted_content_match) = quoted_content_regex.captures( key_match.as_str()) {
                            let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());

                            /*println!("Found quoted string key: ----> {}", un_quoted_string);*/ // Prints: key name
                            //key = un_quoted_string.to_string();
                            key.set_name(un_quoted_string.to_string());
                            
                            //key.set_value_type(ValueType::StringType);
                        }                    
                    });
                }
                                
                // Each key has value...
                
                // See if value type is string...
                if let Some(captures) = value_string_type_regex.captures(line) {                    
                    // captures.get(0) = entire match: `: "test.png",`
                    // captures.get(1) = first capture group: `test.png` (without quotes)

                    if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {

                        // We know that value type is string, now get the value text...
                        captures.get(0).map(|value_match| {                        
                            //let string_value = value_match.as_str();
                            //println!("Found string value: {}", string_value); // Prints: test.png

                            // Get value text...                    
                            if let Some (quoted_content_match) = quoted_content_regex.captures( value_match.as_str()) {
                                let un_quoted_string = quoted_content_match.get(1).map_or("", |m| m.as_str());

                                /*println!("Found quoted string value: {}", un_quoted_string);*/ // Prints: test.png
                                key.set_value(un_quoted_string.to_string());

                                key.set_value_type(ValueType::StringType);
                            }                                                
                        });                                        
                          
                    
                       if json_object.get_ptr().is_none() {

                            //json_object.set_ptr(Some(Box::new(key.clone())));

                            //json_object.set_ptr(Some(key.clone()));

                            //json_object.set_ptr(Some(key));

                            //json_object.set_n(json_object.get_n() + 1);

                            json_object.add_key(key);
                        }

                        //json_object.set_n(json_object.get_n() + 1);
                    }
                                                            
                // See if value type is numeric...            
                } else if let Some(captures) = value_numeric_type_regex.captures(line) {

                    if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count == 0 {
                    
                        /*println! ("Found numeric value: {}", captures.get(1).map_or("", |m| m.as_str()));*/

                        key.set_value(captures.get(1).map_or("", |m| m.as_str()).to_string());
                        key.set_value_type(ValueType::NumberType);

                        if json_object.get_ptr().is_none() {

                            //json_object.set_ptr(Some(Box::new(key.clone())));

                            json_object.set_ptr(Some(key.clone()));

                            //json_object.set_ptr(Some(key));

                            json_object.set_n(json_object.get_n() + 1);
                        }

                        //json_object.set_n(json_object.get_n() + 1);
                    }
                // See if value is a JSON array, look for opening square bracket...
                } else if /*let Some(_captures) = *//*opening_square_bracket_of_json_array_type_regex*/ /*value_json_array_type_regex.captures(line)*/ /*opening_square_bracket_of_json_array_type_regex*/value_json_array_type_regex.is_match(line) {

                    /*println!("Found opening square bracket for JSON array type: {}", line);*/

                    if json_multi_line_object_opening_closing_brace_count == 0 {
                                            
                        if json_multi_line_array_opening_closing_bracket_count == 0 {

                            starting_line_number = file_content.get_current_line_index(); // Store the starting line number when we find the opening square bracket
                        }

                        json_multi_line_array_opening_closing_bracket_count += 1; // Increment the count for opening square bracket
                    }
                }

            } else if /*let Some(_captures) = closing_square_bracket_of_json_array_type_regex.captures(line)*/ closing_square_bracket_of_json_array_type_regex.is_match(line) {

                /*println!("Found closing square bracket for JSON array type: {}", line);*/

                if json_multi_line_object_opening_closing_brace_count == 0 && json_multi_line_array_opening_closing_bracket_count > 0 {

                    json_multi_line_array_opening_closing_bracket_count -= 1; // Decrement the count for closing square bracket 

                    if json_multi_line_array_opening_closing_bracket_count == 0 {

                        ending_line_number = file_content.get_current_line_index(); // Store the starting line number when we find the opening square bracket 

                        // Process Multilne JSON array type here...

                        if json_object.get_ptr().is_some() {

                            key.set_value_type(ValueType::ArrayType);
                            json_object.add_key(key);
                        }

                        //process_multiline_json_array(starting_line_number, ending_line_number, &file_content);

                        // Reset the starting and ending line numbers for the next multiline array
                        starting_line_number = 0;
                        ending_line_number = 0;
                    }
                }
            } else if opening_brace_reg_expr.is_match(line) {
                            
                /*println! ("--> Found opening brace: {}", line);*/

                if json_multi_line_array_opening_closing_bracket_count == 0 {

                    if starting_line_number == 0 {

                        starting_line_number = file_content.get_current_line_index();
                    }

                    json_multi_line_object_opening_closing_brace_count += 1;
                }

            } else if closing_brace_reg_expr.is_match(line) {
            
                /*println! ("--> Found closing brace: {}", line);*/

                if json_multi_line_array_opening_closing_bracket_count == 0 && json_multi_line_object_opening_closing_brace_count > 0 {

                    json_multi_line_object_opening_closing_brace_count -= 1;

                    if json_multi_line_object_opening_closing_brace_count == 0 {

                        ending_line_number = file_content.get_current_line_index();

                        if json_object.get_ptr().is_some() {

                            key.set_value_type(ValueType::ObjectType);
                            json_object.add_key(key);
                        }
                                            
                        // Process Multilne JSON object type here...                                                
                        /*process_multiline_json_object(starting_line_number, ending_line_number, &file_content);*/

                        // Reset the starting and ending line numbers for the next multiline array
                        starting_line_number = 0;
                        ending_line_number = 0;
                    }
                }

            // Check if line has a key...    
            }  

         } else {

             break; // Exit when we reach the end (method returns None)
         }

         key = Box::new(Key::new(String::new(), ValueType::NullType, String::new()));
     }

     /*println! ("starting line number and ending line number {}, {}", starting_line_number, ending_line_number);*/
  
    //Some(Box::new(json_object))

    //Ok(()) 

    Ok(Some(Box::new(json_object)))
}

