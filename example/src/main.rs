/*
     src/main.rs
     This file is part of the program which uses JSON-rust crate.
     Written by, Q@khaa.pk
 */

use std::{env, io};

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
use json_rust::{json_object::{ValueType, Key, JsonKeyPtr, JsonObject}, json::json_main};

fn main() -> Result<(), io::Error> {

    // Get current directory and build path
    let current_dir = env::current_dir()?;
    let json_path = current_dir.join("src").join("png.json"); 
        
    let json_object: Result<Option<Box<JsonObject>>, io::Error> = json_main(json_path.to_str().unwrap());

    /*
    match json_object {

      /*Ok(json_object) => println!("JSON object: {:#?}", json_object),*/
      Ok(json_object) => {

          println!("JSON object: {:#?}", json_object);     
      },

      Err(e) => println!("Error: {}", e),
    }
     */
      
   match json_object {

       Ok(Some(obj)) => {

            obj.pretty_print();

            Ok(())
       },
       Ok(None) => {

            println!("No JSON object returned."); 

            Ok(())
       },
       Err(e) => {

            println!("Error: {}", e); 

            Ok(())
       },
   }   
}

