# JSON-rust

A lightweight, JSON parsing library written in pure Rust — **without using any third-party JSON crates**.  
This library uses a custom recursive linked-list data structure to represent JSON objects and arrays, providing full control over structure, memory, and traversal.

---

## 📦 Features

- ✅ Full support for core JSON types:
  - `String`
  - `Number`
  - `Boolean`
  - `Null`
  - `Object`
  - `Array`
- ✅ Recursive parsing for deeply nested JSON structures (nested objects/arrays with arbitrary depth)
- ✅ Mixed-type for arrays/objects
- ✅ Linked list–based internal representation
- ✅ No external JSON libraries — fully handwritten!

### **Current Limitations**:
- No Unicode/escape character support
- No error handling for malformed JSON
- No validation of number formats
- No support for scientific notation
- Duplicate keys are allowed

---

### 🧠 How It Works (Implementation of core functions)

#### `parser(file_name: &str) -> Result<Option<Box<JsonObject>>, io::Error>`
The main entry point that:
- Reads a JSON file line by line
- Orchestrates the parsing process
- Handles top-level key-value pairs
- Delegates complex structures to `helper_for_object_and_array_types`

#### `helper_for_object_and_array_types(line: &str, key: &mut Key)`
Recursive helper that:
- Processes nested objects/arrays using a state machine
- Handles string/number/boolean/null values
- Maintains parsing context through mutable state

#### Here's how JSON elements are represented in the parsed structure:

* **Objects (`{...}`)**: An object is represented as a linked list of `Key` nodes. Each `Key` in the list corresponds to a key-value pair within that object.
* **Arrays (`[...]`)**: An array is represented by a parent `Key` with `value_type` set to `ArrayType`. This key's internal pointer (`ptr`) then points to a linked list of `Key` nodes, where each node represents an element of the array. For elements that are objects or values without an explicit key (like in an array of objects), the `Key`'s `name` field is empty.
* **Key-Value Pairs**: Each key-value pair is parsed into a `Key` struct, which stores its `name`, its `value` as a string, and a `ValueType` enum (e.g., `StringType`, `NumberType`, `ObjectType`, `ArrayType`).

A key aspect of the parser is its recursive nature. When it encounters the beginning of a nested object or array, it delegates the processing of that block to a specialized function that builds the sub-tree. This allows it to correctly interpret deeply nested data.

### Structure Overview

The JSON structure is parsed into a linked list of custom `Key` structs, each representing a name-value pair. Nested arrays and objects are handled via recursive `ptr` pointers inside each `Key`.

```rust
pub enum ValueType {
    StringType,
    NumberType,
    BooleanType,
    NullType,
    ObjectType,
    ArrayType,
}
```

### Data Model

```rust
pub struct Key {
    name: String,
    value_type: ValueType,
    value: String,
    ptr: JsonKeyPtr, // optional child for nested arrays/objects
    next: Option<Box<Key>>,
    prev: Option<Box<Key>>,
}
```

```rust
pub struct JsonObject {
    ptr: Option<Box<Key>>, // first key in the list
    n: usize,              // number of keys
}
```

---

### 📁 Directory Structure

```
src/
│
├── constants.rs       # Constants for JSON parsing~~
├── file_contents.rs   # File contents module
├── json.rs            # JSON parsing module (recursive descent)
├── json_objects.rs    # JSON objects and key definitions module, (data model)
├── lib.rs             # Library module
```
---

### 🚀 Usage of JSON-rust crate
For example say you are developing a program that needs to parse a JSON file and extract some information from it. You can use the JSON-rust crate to parse the JSON file and extract the information you need. The directory structure of program which uses JSON-rust crate is as follows:

```
json2png/
    ├── lib/
    │   ├── JSON-rust/                 # JSON-rust crate
    │   │   ├── src/
    │   │   └── Cargo.toml             # Cargo.toml file
    ├──src/
    │   ├── main.rs                    # Main module    
    │   └── png.json                   # JSON file, input file which will be parsed
    └── Cargo.toml                     # Cargo.toml file
```

### 1. Clone the repository

```bash
cd json2png/lib
git clone https://github.com/KHAAdotPK/JSON-rust.git
cd ..
```

### 2. Add the JSON-rust crate to your project's json2png/Cargo.toml file

```toml
[dependencies]
json-rust = { path = "lib/JSON-rust" }
```

### 2. Build the project

```bash
cargo build
```
---

### 📄 Example Input

Given the following `png.json`:

```json
{
    "data": 
    { 
        "internal": [1, 2, 3, 4, 5, 6], 
        [7, 8, 9], 
        [10, 11, 12, 13, 14, 15]  
    },
    "salaray": 10000,
    "paid": false, 
    "image_data": [
            [255, 204, 153], [255, 204, 153], [0, 0, 0], [255, 255, 255]
    ],
    "name": "test.png",
    "chunks": [
        {
            "type": "ihdr",
            "data": [
                {
                    "width": 1300,
                    "height": 1300,
                    "bit_depth": 8,
                    "color_type": "rgb",
                    "compression_method": "deflate",
                    "filter_method": "adaptive",
                    "interlace_method": "none",
                    "level1": {
                        "level2": {
                            "level3": [
                                        {
                                            "level4a": [
                                                        {"level5": "deep"}
                                             ],
                                            "level4b": {}
                                        }
                            ]
                        }
                    }
                },
                "bonus_paid": true
            ],
            {
                "empty_structures": { 
                "empty_object": {}, 
                "empty_array": [] 
                },
                "sparse_arrays": [1, , 3],
                "trailing_commas": { "a": 1, },
                "unquoted_keys": { key: "value" }
            }
            "bonus_paid": true
        },        
        {
            "type": "idat",
            "data": [
                {
                    "image_data": [255, 0, 0]
                },
                "bonus": 90000
            ]
        },        
        "bonus_paid": true,
        "number": 19999
    ],
    "mixed_array": [
        null,
        42,
        "string",
        true,
        false,
        {"key": "value"},
        [1, 2, 3],
        -3.14
    ],
    "bonus_paid": true,
    "number": 10000
}
```

### Write the code to parse the JSON file

```Rust
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
```

### Output

```text
data: ObjectType
    internal: ArrayType
        : NumberType
        : NumberType
        : NumberType
        : NumberType
        : NumberType
        : NumberType
    : ArrayType
        : NumberType
        : NumberType
        : NumberType
    : ArrayType
        : NumberType
        : NumberType
        : NumberType
        : NumberType
        : NumberType
        : NumberType
salaray: NumberType
paid: BooleanType
image_data: ArrayType
    : ArrayType
        : NumberType
        : NumberType
        : NumberType
    : ArrayType
        : NumberType
        : NumberType
        : NumberType
    : ArrayType
        : NumberType
        : NumberType
        : NumberType
    : ArrayType
        : NumberType
        : NumberType
        : NumberType
name: StringType
chunks: ArrayType
    : ObjectType
        type: StringType
        data: ArrayType
            : ObjectType
                width: NumberType
                height: NumberType
                bit_depth: NumberType
                color_type: StringType
                compression_method: StringType
                filter_method: StringType
                interlace_method: StringType
                level1: ObjectType
                    level2: ObjectType
                        level3: ArrayType
                            : ObjectType
                                level4a: ArrayType
                                    : ObjectType
                                        level5: StringType
                                level4b: ObjectType
            bonus_paid: BooleanType
        : ObjectType
            empty_structures: ObjectType
                empty_object: ObjectType
                empty_array: ArrayType
            sparse_arrays: ArrayType
                : NumberType
                : NumberType
            trailing_commas: ObjectType
                a: NumberType
            unquoted_keys: ObjectType
                : StringType
        bonus_paid: BooleanType
    : ObjectType
        type: StringType
        data: ArrayType
            : ObjectType
                image_data: ArrayType
                    : NumberType
                    : NumberType
                    : NumberType
            bonus: NumberType
    bonus_paid: BooleanType
    number: NumberType
mixed_array: ArrayType
    : NullType
    : NumberType
    : StringType
    : BooleanType
    : BooleanType
    : ObjectType
        key: StringType
    : ArrayType
        : NumberType
        : NumberType
        : NumberType
    : NumberType
bonus_paid: BooleanType
number: NumberType
Processing node 0: data
Processing node 1: salaray
Processing node 2: paid
Processing node 3: image_data
Processing node 4: name
Processing node 5: chunks
Processing node 6: mixed_array
Processing node 7: bonus_paid
Processing node 8: number
```
---

## 🔧 Future Improvements

- ~~[ ] Add pretty-printer for JSON output~~
- ~~[ ] Add serialization (back to string from JsonObject)~~
- ~~[ ] Support UTF-8 escape sequences in strings~~
- ~~[ ] Add `from_str()` constructor for direct parsing~~
- ~~[ ] Performance improvements (switch to Vecs?)~~
- ~~[ ] Optional Serde interop~~

---

## 📜 License

This project is governed by a license, the details of which can be located in the accompanying file named 'LICENSE.' Please refer to this file for comprehensive information.

