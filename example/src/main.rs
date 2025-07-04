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
//use png::{png::Png, png::PngChunk, png::PngChunkData, png::PngChunkData::PngChunkDataIHDR, png::PngChunkData::PngChunkDataIDAT, png::PngChunkData::PngChunkDataIEND};

fn main() -> Result<(), io::Error> {

    // Get current directory and build path
    let current_dir = env::current_dir()?;
    let json_path = current_dir.join("src").join("png.json"); 
        
    let json_object: Result<Option<Box<JsonObject>>, io::Error> = json_main(json_path.to_str().unwrap());

    /*
    // To print the very long (relatively) tree structure of the JSON object
    match json_object {

      /*Ok(json_object) => println!("JSON object: {:#?}", json_object),*/
      Ok(json_object) => {

          println!("JSON object: {:#?}", json_object);     
      },

      Err(e) => println!("Error: {}", e),
    }
     */
      
   match json_object {

       Ok(Some(jobj)) => {

            jobj.pretty_print();

            println!("Number of nodes in jobj is {}", jobj.get_n());

            let mut ptr: &Box<Key> = jobj.get_ptr().as_ref().unwrap();
            
            let mut i: usize = 0;

            // Option 1.
            /*loop*/ {
                
                //println!("Processing node {}: {}", i, ptr.get_name());

                // Process the node here

                // Option 1.1: Note: -1 because we start from the first node and also the next of (n - 1)th node has None as value. Unwrapping None will cause a panic                
                /*if i >= jobj.get_n() - 1 {
                  
                    break;
                }

                ptr = ptr.get_next().unwrap();

                i += 1;*/

                // Option 1.2: Check for None before unwrapping
                /*if let Some(next_ptr) = ptr.get_next() {

                    ptr = next_ptr;

                    i += 1;
                } else {

                    break;
                }*/
            }

            // Option 2: Use a while loop with proper condition
            /*while i < jobj.get_n() {

                println!("Processing node {}: {}", i, ptr.get_name());

                // Process the node here
                
                if let Some(next) = ptr.get_next() {

                    ptr = next;
                } else {

                    break;
                }
                i += 1;
            }*/

            // Option 3: Iterate through all nodes (simplest)
            loop {

                println!("Processing node {}: {}", i, ptr.get_name());

                //Process the node here  
                
                match ptr.get_next() {

                    Some(next) => {

                        ptr = next;
                        i += 1;
                    },

                    None => break,
                }
            }

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

