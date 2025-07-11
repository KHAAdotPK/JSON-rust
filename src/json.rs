/*
    src/kson.rs
    This file is part of the JSON-rust crate.
    Written by, Q@khaa.pk
 */

use std::io;
use regex::Regex;

use crate::file_content::FileContent;
use crate::constants::{JSON_OPENIING_BRACE, JSON_CLOSING_BRACE, JSON_OPENING_BRACE_REG_EXPR_PATTERN, JSON_CLOSING_BRACE_REG_EXPR_PATTERN, JSON_KEY_REG_EXPR_PATTERN, JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE, JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE, JSON_VALUE_TYPE_STRING_REG_EXPR_PATTERN, JSON_QUOTED_CONTENT_PATTERN, JSON_VALUE_TYPE_NUMERIC_PATTERN, JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN, JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN_VALUE_STRING};
use crate::json_object::{ValueType, Key, JsonKeyPtr, JsonObject};

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
    let value_json_array_type_regex = Regex::new(JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap(); // Matches `: ["` at the start of a JSON array value 
    let closing_square_bracket_of_json_array_type_regex = Regex::new(JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap();
    let single_line_json_array_type_regex = Regex::new(JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN).unwrap();
    let single_line_json_array_type_value_string_regex = Regex::new(JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN_VALUE_STRING).unwrap();

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
                // Check for single line json array                        
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

                                    let l_key = Box::new(Key::new(key_name, ValueType::StringType, un_quoted_string.to_string()));
                                    key.add_key(l_key);
                                    key_value_pair_complete = true;
                                }
                            }                                                                                                    
                        });

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
    let single_line_json_array_type_regex = Regex::new(JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN).unwrap();
    let single_line_json_array_type_value_string_regex = Regex::new(JSON_SINGLE_LINE_ARRAY_TYPE_PATTERN_VALUE_STRING).unwrap();

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
                // Check for single line json array    
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
    let value_json_array_type_regex = Regex::new(JSON_OPENING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap();
    let closing_square_bracket_of_json_array_type_regex = Regex::new(JSON_CLOSING_SQUARE_BRACKET_PATTERN_FOR_ARRAY_TYPE).unwrap();
    let opening_brace_reg_expr = Regex::new(JSON_OPENING_BRACE_REG_EXPR_PATTERN).unwrap();
    let closing_brace_reg_expr = Regex::new(JSON_CLOSING_BRACE_REG_EXPR_PATTERN).unwrap();

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
                // Check for single line array    
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

                            println!("Found quoted string key: ----> {}", un_quoted_string); // Prints: key name
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

                                println!("Found quoted string value: {}", un_quoted_string); // Prints: test.png
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
                            
                println! ("--> Found opening brace: {}", line);

                if json_multi_line_array_opening_closing_bracket_count == 0 {

                    if starting_line_number == 0 {

                        starting_line_number = file_content.get_current_line_index();
                    }

                    json_multi_line_object_opening_closing_brace_count += 1;
                }

            } else if closing_brace_reg_expr.is_match(line) {
            
                println! ("--> Found closing brace: {}", line);

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

