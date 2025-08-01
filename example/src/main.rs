/*
     src/main.rs
     This file is part of the program which uses JSON-rust crate.
     Written by, Q@khaa.pk
 */

use std::{env, io, fs::File, io::Read, io::Write};

// Choose one of the two following statements
/*
     1. Either this. It is a wildcard import which brings all items from the crate into scope.
     This is generally discouraged as it can lead to naming conflicts.
 */
//use json_rust::*;
/*
     2. Or this one...
     This is a more explicit import which only brings the specific items we need into scope.
     This is generally recommended as it provides better control and clarity.
 */
use json_rust::{json_object::{ValueType, Key, JsonKeyPtr, JsonObject}, json::parser};
use json_rust::helper::{traverse, TraverseError};

fn main() -> Result<(), io::Error> {

    // Get current directory and build path
    let current_dir = env::current_dir()?;
  
    let json_path = current_dir.join("src").join("test.json"); 

    let json_object: Result<Option<Box<JsonObject>>, io::Error> = parser (json_path.to_str().unwrap());

    //traverse(&json_object);

    match traverse(&json_object) {
        Ok(()) => {

            println!("Traversal completed successfully");
        },
        Err(TraverseError::IoError(io_err_msg)) => {

            eprintln!("IO error during traversal: {}", io_err_msg);
            // Handle IO error specific logic
        },
        Err(TraverseError::NoJsonObject) => {
            
            eprintln!("No JSON object found to traverse");
            // Handle the case where there's no JSON object
        }
    }
           
    Ok(())
}

