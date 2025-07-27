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

pub fn helper_for_object_and_array_types (line: &str, key: &mut Key) {
   // State Machine
    let mut array_type_encountered = false;
    let mut array_type_encountered_count: usize = 0;
    let mut object_type_encountered = false;
    let mut object_type_encountered_count: usize = 0;

    let mut start_of_string_encountered = false;
    let mut start_of_key_name_encountered = false;
    let mut end_of_string_encountered = false;
    let mut end_of_key_name_encountered = false;
    let mut start_of_value_string_encountered = false;
    let mut end_of_value_string_encountered = false;

    let mut i: usize = 0;

    let mut key_of_pair = String::new();
    let mut value_of_pair = String::new();
    let mut neutral_string = String::new();

    let mut json_object = JsonObject::new();

    // Main parsing loop - processes each line of the file
    
        
        // Create a peekable iterator for the current line's characters
        let mut line_of_peekable_chars = line.chars().peekable();

        //println! ("-->>>>> {}", line);

        // Process each character in the line
        while let Some(ch) = line_of_peekable_chars.next() {

            //println! ("-------->>>>>>>>>>> {}, {}, {}", line.chars().count(), i, line.len());

            /*if i == (line.len() - 1) && ch == '}' {

                println! ("---->>>>>>>   Found and {}", ch);
            }*/
                      
            // Handle JSON root object markers (first '{' and last '}')
            if ((ch == '{' && i == 0) || (ch == '}' && i == (line.len() - 1))) || ((ch == '[' && i == 0) || (ch == ']' && i == (line.len() - 1))) || ch == '\n' { // Ignore JSON root object

                //println! ("Hola...... ");

                //println! ("k = {} / n = {} / v = {}", key_of_pair, neutral_string, value_of_pair);

                /*println! ("-> k = {} / n = {} / v = {} and {}", key_of_pair, neutral_string, value_of_pair, end_of_string_encountered);*/
                
                // Edge case: handle any remaining string type key-value pair before root object ends
                if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && end_of_string_encountered) /*&& key_of_pair.len() > 0*/ && neutral_string.len() > 0 {

                    //println! ("{} / {}", neutral_string, value_of_pair);

                    value_of_pair = neutral_string.clone();
                                        
                    let lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::StringType, value_of_pair.clone()));                   
                    key.add_key(lkey); 

                    // Cleanup
                    key_of_pair.clear();
                    value_of_pair.clear();
                    neutral_string.clear();
                    
                // Edge case: handle non-string values (null, boolean, number) before root object ends                        
                } else if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && !end_of_string_encountered) /*&& key_of_pair.len() > 0*/ && neutral_string.len() > 0 {

                    //println! ("{} / {}", key_of_pair, value_of_pair);

                    /*println! ("-> Are we there yet.... {}", neutral_string.clone());*/

                    value_of_pair = neutral_string.clone();

                    if value_of_pair.clone().trim() == "null" { // Null

                        let lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::NullType, value_of_pair.clone()));
                        key.add_key(lkey);
  
                    } else if value_of_pair.clone().trim() == "true" || value_of_pair.clone().trim() == "false" { // Boolean

                        /*println! ("Yes we are....");*/

                        let lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::BooleanType, value_of_pair.clone()));
                        key.add_key(lkey);

                    } else { // Number
                    
                        //if value_of_pair.len() > 0 {

                            let lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::NumberType, value_of_pair.clone()));
                            key.add_key(lkey);
                        //}
                    }

                    // Cleanup
                    neutral_string.clear();
                    key_of_pair.clear();
                    value_of_pair.clear();                    
                }

                i = i + 1;
                
                continue;  // Skip processing further below              
            }
            if !object_type_encountered && ch == '[' {

                value_of_pair.push(ch);

                array_type_encountered = true;
                array_type_encountered_count += 1;

                i = i + 1;

                continue;
            }
            if array_type_encountered && ch == ']' {

                value_of_pair.push(ch);

                array_type_encountered_count -= 1;

                if array_type_encountered_count == 0 {

                    array_type_encountered = false;

                    // Here we have a complete key/value pair of Object type, add it to the json tree
                    //println! ("{} / {}", key_of_pair, value_of_pair);

                    let mut lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::ArrayType, value_of_pair.clone()));
                    helper_for_object_and_array_types(&value_of_pair.clone(), &mut lkey);
                    key.add_key(lkey);

                    // Cleanup
                    key_of_pair.clear();
                    value_of_pair.clear();
                }

                i = i + 1;

                continue;
            }
            if array_type_encountered {

                value_of_pair.push(ch);

                i = i + 1;

                continue;
            }             
            if !array_type_encountered && ch == '{' {

                value_of_pair.push(ch);

                object_type_encountered = true;
                object_type_encountered_count += 1;

                i = i + 1;
                
                continue;
            } 
            if object_type_encountered && ch == '}' {

                value_of_pair.push(ch);

                object_type_encountered_count -= 1;

                if object_type_encountered_count == 0 {

                    object_type_encountered = false;

                    // Here we have a complete key/value pair of Object type, add it to the json tree
                    //println! ("{} / {}", key_of_pair, value_of_pair);

                    let mut lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::ObjectType, value_of_pair.clone()));
                    helper_for_object_and_array_types(&value_of_pair.clone(), &mut lkey);
                    key.add_key(lkey);
                                    
                    // Cleanup
                    key_of_pair.clear();
                    value_of_pair.clear();
                }

                i = i + 1;

                continue;   
            }
            if object_type_encountered {

                value_of_pair.push(ch);

                i = i + 1;

                continue;
            } 
            /* 
                String type, it could be the name of key and value.
                In the case of key it is always delimited by closing quotation mark followed by colon like in "name": 
                In the case of value no following colon after closing quotation mark and option coma ',' as in "key": "value"[,]
            */
            // String parsing logic
            // Start of string (opening quote)
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered) && ch == '"' {
                                                                
                start_of_string_encountered = true;

                i = i + 1; 

                continue;                                
            } 
            // End of string (closing quote), where collected string could be key or value
            if start_of_string_encountered && ch == '"' {
                
                // Set
                end_of_string_encountered = true;

                // Reset
                start_of_string_encountered = false;

                i = i + 1;

                continue;
            }
            // Collect string content between quotes, where the string could be key or value 
            if start_of_string_encountered {
                
                neutral_string.push(ch);

                i = i + 1;
                                
                continue;
            }           
            // Determine if it was a key (string followed by colon)
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && end_of_string_encountered) && ch == ':' {
                                
                key_of_pair = neutral_string.clone();
                
                // Cleanup
                neutral_string.clear();

                // Reset
                end_of_string_encountered = false;

                //println! ("A = ----> {}", key_of_pair);

                i = i + 1;
                                
                continue;
            }
             // Value handling for string type (comma indicates end of key-value pair)
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && end_of_string_encountered) && ch == ',' {
                                
                value_of_pair = neutral_string.clone();

                //println! ("{} / {}", key_of_pair, value_of_pair);
                
                let lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::StringType, value_of_pair.clone()));                   
                key.add_key(lkey);

                // Cleanup
                neutral_string.clear();
                key_of_pair.clear();
                value_of_pair.clear();

                // Reset
                end_of_string_encountered = false;

                i = i + 1;
                                
                continue;
            }
            // Value handling for non-string types (null, boolean, number)
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && !end_of_string_encountered) /*&& key_of_pair.len() > 0*/ && ch != ' ' && ch != ',' && ch != '\n' {

                //value_of_pair.push(ch);
                neutral_string.push(ch);

                i = i + 1;

                continue;
            }
            // Finalize non-string value when comma encountered
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && !end_of_string_encountered) /*&& key_of_pair.len() > 0*/ && neutral_string.len() > 0  && (ch == ',' || ch == '\n') {

                //println! ("{} / {}", key_of_pair, value_of_pair);

                value_of_pair = neutral_string.clone();

                if value_of_pair.clone().trim() == "null" { // Null

                    let lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::NullType, value_of_pair.clone()));
                    key.add_key(lkey);
  
                } else if value_of_pair.clone().trim() == "true" || value_of_pair.clone().trim() == "false" { // Boolean

                    let lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::BooleanType, value_of_pair.clone()));
                    key.add_key(lkey);

                } else { // Number
                    //if value_of_pair.len() > 0 {
                        
                        let lkey = Box::new(Key::new(key_of_pair.clone(), ValueType::NumberType, value_of_pair.clone()));
                        key.add_key(lkey);
                    //}
                }
                
                // Cleanup
                neutral_string.clear();
                key_of_pair.clear();
                value_of_pair.clear();

                i = i + 1;

                continue;
            }
            
            i = i + 1;                            
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
pub fn parser (file_name: &str) -> Result<Option<Box<JsonObject>>, io::Error> {

    // State Machine
    let mut array_type_encountered = false;
    let mut array_type_encountered_count: usize = 0;
    let mut object_type_encountered = false;
    let mut object_type_encountered_count: usize = 0;

    let mut start_of_string_encountered = false;
    let mut start_of_key_name_encountered = false;
    let mut end_of_string_encountered = false;
    let mut end_of_key_name_encountered = false;
    let mut start_of_value_string_encountered = false;
    let mut end_of_value_string_encountered = false;

    let mut i: usize = 0;

    let mut key_of_pair = String::new();
    let mut value_of_pair = String::new();
    let mut neutral_string = String::new();

    let mut json_object = JsonObject::new();

    /*
        Rust does not have exceptions. It has panics, but their use for error-handling is discouraged (they are meant for unrecoverable errors).
        In Rust, error handling uses Result. 
        Following method is very verbose. This is where the question mark operator ? comes in ... 
        // let file_content = FileContent::from_file(file_name)?;
        It is called the try operator or the question mark operator. It is a postfix operator that unwraps Result<T, E> and Option<T> values ...
        (the ? operator can only be used in a function that returns Result or Option)

        What "?" does when applied to Result<T, E>, here is equivalent to the match statement below. In short ... 
        1. It unpacks the Result if OK (it unwraps the result and gives you the inner value).
        2. It returns the error and potentially convert it to another type (propagating the error of the called function to the calling function).

        What "?" does when applied to Option<T>, it propagates None to the caller, leaving you the content of the Some branch to deal with ...
        /*
            let val = Some(42)?;
            println!("{:?}", val); // 42
         */ 
        So "?" cuts down the boilerplate code. 
    */
    let file_content = match FileContent::from_file(file_name) {

        Ok(content) => { content },
        Err(e) => { return Err(e) }
    };

    // Main parsing loop - processes each line of the file
    loop {
        
        let line = file_content.get_line_by_index(i);

        if line == None {

            break;
        }

        // Create a peekable iterator for the current line's characters
        let mut line_of_peekable_chars = line.unwrap().chars().peekable();

        // Process each character in the line
        while let Some(ch) = line_of_peekable_chars.next() {
          
            // Handle JSON root object markers (first '{' and last '}')
            if ( (ch == '{' && i == 0) || (ch == '}' && i == (file_content.count_lines() - 1)) ) || ( (ch == '[' && i == 0) || (ch == ']' && i == (file_content.count_lines() - 1)) ) || ch == '\n' { // Ignore JSON root object

                /*println! ("--> k = {} / n = {} / v = {} and {}", key_of_pair, neutral_string, value_of_pair, end_of_string_encountered);*/

                // Edge case: handle any remaining string type key-value pair before root object ends
                if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && end_of_string_encountered) /*&& key_of_pair.len() > 0*/ && neutral_string.len() > 0 {

                    //println! ("{} / {}", neutral_string, value_of_pair);

                    value_of_pair = neutral_string.clone();
                                        
                    let key = Box::new(Key::new(key_of_pair.clone(), ValueType::StringType, value_of_pair.clone()));                   
                    json_object.add_key(key); 

                    // Cleanup
                    neutral_string.clear();
                    key_of_pair.clear();
                    value_of_pair.clear();                    
                    
                // Edge case: handle non-string values (null, boolean, number) before root object ends                        
                } else if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && !end_of_string_encountered) /*&& key_of_pair.len() > 0*/ && neutral_string.len() > 0 {

                    //println! ("{} / {}", key_of_pair, value_of_pair);
                    value_of_pair = neutral_string.clone();

                    /*println! ("--> Are we there yet....");*/

                    if value_of_pair.clone().trim() == "null" { // Null

                        let key = Box::new(Key::new(key_of_pair.clone(), ValueType::NullType, value_of_pair.clone()));
                        json_object.add_key(key);
  
                    } else if value_of_pair.clone().trim() == "true" || value_of_pair.clone().trim() == "false" { // Boolean

                        let key = Box::new(Key::new(key_of_pair.clone(), ValueType::BooleanType, value_of_pair.clone()));
                        json_object.add_key(key);

                    } else { // Number
                    
                        let key = Box::new(Key::new(key_of_pair.clone(), ValueType::NumberType, value_of_pair.clone()));
                        json_object.add_key(key);
                    }

                    // Cleanup
                    neutral_string.clear();
                    key_of_pair.clear();
                    value_of_pair.clear();                    
                }
                
                continue;  // Skip processing further below              
            }
            if !object_type_encountered && ch == '[' {

                value_of_pair.push(ch);

                array_type_encountered = true;
                array_type_encountered_count += 1;

                continue;
            }
            if array_type_encountered && ch == ']' {

                value_of_pair.push(ch);

                array_type_encountered_count -= 1;

                if array_type_encountered_count == 0 {

                    array_type_encountered = false;

                    // Here we have a complete key/value pair of Object type, add it to the json tree
                    //println! ("{} / {}", key_of_pair, value_of_pair);

                    let mut key = Box::new(Key::new(key_of_pair.clone(), ValueType::ArrayType, value_of_pair.clone()));
                    helper_for_object_and_array_types(&value_of_pair.clone(), &mut key);
                    json_object.add_key(key);

                    // Cleanup
                    key_of_pair.clear();
                    value_of_pair.clear();
                }

                continue;
            }
            if array_type_encountered {

                value_of_pair.push(ch);

                continue;
            }
            if !array_type_encountered && ch == '{' {

                value_of_pair.push(ch);

                object_type_encountered = true;
                object_type_encountered_count += 1;
                
                continue;
            }
            if object_type_encountered && ch == '}' {

                value_of_pair.push(ch);

                object_type_encountered_count -= 1;

                if object_type_encountered_count == 0 {

                    object_type_encountered = false;

                    // Here we have a complete key/value pair of Object type, add it to the json tree
                    //println! ("{} / {}", key_of_pair, value_of_pair);

                    let mut key = Box::new(Key::new(key_of_pair.clone(), ValueType::ObjectType, value_of_pair.clone()));
                    helper_for_object_and_array_types(&value_of_pair.clone(), &mut key);
                    json_object.add_key(key);
                                    
                    // Cleanup
                    key_of_pair.clear();
                    value_of_pair.clear();
                }

                continue;            
            }
            if object_type_encountered {

                value_of_pair.push(ch);

                continue;
            }
            /* 
                String type, it could be the name of key and value.
                In the case of key it is always delimited by closing quotation mark followed by colon like in "name": 
                In the case of value no following colon after closing quotation mark and option coma ',' as in "key": "value"[,]
            */
            // String parsing logic
            // Start of string (opening quote)
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered) && ch == '"' {
                                                                
                start_of_string_encountered = true; 

                continue;                                
            }
            // End of string (closing quote), where collected string could be key or value
            if start_of_string_encountered && ch == '"' {
                
                // Set
                end_of_string_encountered = true;

                // Reset
                start_of_string_encountered = false;

                continue;
            }
            // Collect string content between quotes, where the string could be key or value 
            if start_of_string_encountered {
                
                neutral_string.push(ch);
                                
                continue;
            }            
            // Determine if it was a key (string followed by colon)
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && end_of_string_encountered) && ch == ':' {
                                
                key_of_pair = neutral_string.clone();
                
                // Cleanup
                neutral_string.clear();

                // Reset
                end_of_string_encountered = false;

                //println! ("----> {}", key_of_pair);
                                
                continue;
            }
             // Value handling for string type (comma indicates end of key-value pair)
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && end_of_string_encountered) && ch == ',' {
                                
                value_of_pair = neutral_string.clone();

                //println! ("{} / {}", key_of_pair, value_of_pair);
                
                let key = Box::new(Key::new(key_of_pair.clone(), ValueType::StringType, value_of_pair.clone()));                   
                json_object.add_key(key);

                // Cleanup
                neutral_string.clear();
                key_of_pair.clear();
                value_of_pair.clear();

                // Reset
                end_of_string_encountered = false;
                                
                continue;
            }
            // Value handling for non-string types (null, boolean, number)
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && !end_of_string_encountered) /*&& key_of_pair.len() > 0*/ && ch != ' ' && ch != ',' && ch != '\n' {

                //value_of_pair.push(ch);
                neutral_string.push(ch);

                continue;
            }
            // Finalize non-string value when comma encountered
            if (!array_type_encountered && !object_type_encountered && !start_of_string_encountered && !end_of_string_encountered) /*&& key_of_pair.len() > 0*/ && (ch == ',' || ch == '\n') {

                //println! ("{} / {}", key_of_pair, value_of_pair);

                value_of_pair = neutral_string.clone();

                if value_of_pair.clone().trim() == "null" { // Null

                    let key = Box::new(Key::new(key_of_pair.clone(), ValueType::NullType, value_of_pair.clone()));
                    json_object.add_key(key);
  
                } else if value_of_pair.clone().trim() == "true" || value_of_pair.clone().trim() == "false" { // Boolean

                    let key = Box::new(Key::new(key_of_pair.clone(), ValueType::BooleanType, value_of_pair.clone()));
                    json_object.add_key(key);

                } else { // Number
                    if value_of_pair.len() > 0 {

                        let key = Box::new(Key::new(key_of_pair.clone(), ValueType::NumberType, value_of_pair.clone()));
                        json_object.add_key(key);
                    }
                }
                
                // Cleanup
                neutral_string.clear();
                key_of_pair.clear();
                value_of_pair.clear();

                continue;
            }            
        }    

        i = i + 1;
    }

    if json_object.get_n() > 0 {

        Ok(Some(Box::new(json_object)))
    } else {

        Ok(None)
    }    
}
        


