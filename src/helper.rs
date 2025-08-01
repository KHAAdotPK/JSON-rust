/*
    src/kson.rs
    This file is part of the JSON-rust crate.
    Written by, Q@khaa.pk
 */

use std::io;
use crate::json_object::{ValueType, Key, JsonKeyPtr, JsonObject}; 

// Define a custom error type for your traverse function
#[derive(Debug)]
pub enum TraverseError {
    IoError(String),  // Change from io::Error to String
    //IoError(io::Error),
    NoJsonObject,

    // Add other error variants as needed
}

impl From<io::Error> for TraverseError {
    fn from(err: io::Error) -> Self {
        TraverseError::IoError(err.to_string())
    }
}

 /*
    A depth-first traversal using a combination of recursion and iteration with a stack
 */
fn worker<'a>(node_stack: &mut Vec<&'a Box<Key>>, mut ptr: &'a Box<Key>) {

    /*
        - Loop through siblings at current level
        - For each sibling: if it has children, recursively process all its children
        - After recursion returns: continue to next sibling
        - When no more siblings: return to parent level
     */
    loop {

        println!("--> Processing node: {}, n = {}", ptr.get_name(), ptr.get_n());

        /* If the node has children then process them */
        if ptr.get_n() > 0 {

            // Push parent node
            node_stack.push(ptr);

            // Get first child
            ptr = ptr.get_ptr().as_ref().unwrap();

            // Do the same for the child, if each child has a child of its own then do the same for that child, if not then stop processing and return             
            worker (node_stack, ptr);

            // After returning pop the parent of the last child, now go get the sibling of this parent 
            ptr = node_stack.pop().unwrap();
        } 
        
        // Now go to the next sibling
        match ptr.get_next() {

            Some(next) => {

                ptr = next;
            },

            None => break, // No more siblings, break the loop and return to parent of the parent
        }
    }
}

pub fn traverse (parsed_json_of_very_simple_file: &Result<Option<Box<JsonObject>>, io::Error>) -> Result<(), TraverseError> {

    let mut node_stack: Vec<&Box<Key>> = Vec::new();

    match parsed_json_of_very_simple_file {

        Ok(Some(jobj)) => {

            let mut ptr = jobj.get_ptr().as_ref().unwrap(); 
           
            loop {

                // Process node here                
                println!("-> Processing node: {}, n = {}", ptr.get_name(), ptr.get_n());

                if ptr.get_n() > 0 {
                
                    node_stack.push(ptr);
                
                    ptr = ptr.get_ptr().as_ref().unwrap();  
                    
                    worker (&mut node_stack, &ptr);

                    ptr = node_stack.pop().unwrap();
                }

                match ptr.get_next() {

                    Some(next) => {

                        ptr = next;                            
                    },

                    None => break,                    
                }                                
            }

            Ok(())
        },
        Ok(None) => {

            // Return an error indicating no JSON object was found
            return Err(TraverseError::NoJsonObject);
        },
        Err(e) => {

            // Convert the io::Error to TraverseError and return it
            //return Err(TraverseError::IoError((*e).clone()));
            return Err(TraverseError::IoError(e.to_string()));
        }
    }    
}
