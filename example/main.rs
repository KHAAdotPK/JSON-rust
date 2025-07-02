/*
     src/main.rs
     This file is part of the program which uses JSON-rust crate.
     Written by, Q@khaa.pk
 */

use std::io;
use regex::Regex;
use json_rust::{json_object::{ValueType, Key, JsonKeyPtr, JsonObject}, json::json_main};

fn main() -> Result<(), io::Error> {
    
   let json_object: Result<Option<Box<JsonObject>>, io::Error> = json_main("src/png.json");

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

            obj.pretty_print()
       },
       Ok(None) => {

            println!("No JSON object returned.") 
       },
       Err(e) => {

            println!("Error: {}", e) 
       },
   }

   Ok(())
}

