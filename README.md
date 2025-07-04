# JSON-rust

A lightweight, JSON parsing library written in pure Rust — **without using any third-party JSON crates**.  
This library uses a custom recursive linked-list data structure to represent JSON objects and arrays, providing full control over structure, memory, and traversal.

---

## 📦 Features

- ✅ Full support for core JSON types:
  - `String`
  - `Number`
  - ~~`Boolean`~~
  - ~~`Null`~~
  - `Object`
  - `Array`
- ✅ Recursive parsing for deeply nested JSON structures
- ✅ Linked list–based internal representation
- ✅ Modular code with separate tokenizer, parser, and data model
- ✅ No external JSON libraries — fully handwritten!
- ✅ Supports inspection and traversal of the parsed structure

---

## 🧠 How It Works

This parser is designed to read a JSON file and transform its contents into a custom, navigable tree structure built with Rust. The core of the library revolves around two primary structs: `JsonObject` and `Key`. The entire JSON file is parsed into a single root `JsonObject`, which acts as the entry point to the data.

The parser operates by reading the file line by line and using regular expressions to identify the distinct components of the JSON syntax. It recognizes keys, values, and structural elements like `{`, `}`, `[`, and `]`. This approach allows it to handle complex, nested structures that span multiple lines.

Here's how JSON elements are represented in the parsed structure:

* **Objects (`{...}`)**: An object is represented as a linked list of `Key` nodes. Each `Key` in the list corresponds to a key-value pair within that object.
* **Arrays (`[...]`)**: An array is represented by a parent `Key` with `value_type` set to `ArrayType`. This key's internal pointer (`ptr`) then points to a linked list of `Key` nodes, where each node represents an element of the array. For elements that are objects or values without an explicit key (like in an array of objects), the `Key`'s `name` field is empty.
* **Key-Value Pairs**: Each key-value pair is parsed into a `Key` struct, which stores its `name`, its `value` as a string, and a `ValueType` enum (e.g., `StringType`, `NumberType`, `ObjectType`, `ArrayType`).

A key aspect of the parser is its recursive nature. When it encounters the beginning of a nested object or array, it delegates the processing of that block to a specialized function that builds the sub-tree. This allows it to correctly interpret deeply nested data. However, it's worth noting that in its current implementation, an array of simple values (like numbers, e.g., `[255, 0, 0]`) is parsed into a single `Key` of `StringType` with the contents concatenated into the `value` field.

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

## 📁 Directory Structure

```
src/
│
├── constants.rs       # Constants for JSON parsing
├── file_contents.rs   # File contents module
├── json.rs            # JSON parsing module (recursive descent)
├── json_objects.rs    # JSON objects and key definitions module, (data model)
├── lib.rs             # Library module
```
---

## 🚀 Usage of JSON-rust crate
For example say you are developing a program that needs to parse a JSON file and extract some information from it. You can use the JSON-rust crate to parse the JSON file and extract the information you need. The directory structure of program which uses JSON-rust crate is as follows:

```
json2png/
    ├── lib/
    │   ├── JSON-rust/                 # JSON-rust crate
    │   │   ├── src/
    │   │   └── Cargo.toml             # Cargo.toml file
    │   └── src/
    │       ├── main.rs                # Main module    
    │       └── png.json               # JSON file, input file which will be parsed
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
                    "interlace_method": "none"
                }
            ]
        },
        {
            "type": "idat",
            "data": [
                {
                    "image_data": [255, 0, 0]
                }
            ]
        }
    ]
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
JSON object: Some(
    JsonObject {
        ptr: Some(
            Key {
                name: "name",
                value_type: StringType,
                value: "test.png",
                ptr: None,
                n: 0,
                next: Some(
                    Key {
                        name: "chunks",
                        value_type: ArrayType,
                        value: "",
                        ptr: Some(
                            Key {
                                name: "",
                                value_type: ObjectType,
                                value: "",
                                ptr: Some(
                                    Key {
                                        name: "type",
                                        value_type: StringType,
                                        value: "ihdr",
                                        ptr: None,
                                        n: 0,
                                        next: Some(
                                            Key {
                                                name: "data",
                                                value_type: ArrayType,
                                                value: "",
                                                ptr: Some(
                                                    Key {
                                                        name: "",
                                                        value_type: ObjectType,
                                                        value: "",
                                                        ptr: Some(
                                                            Key {
                                                                name: "width",
                                                                value_type: NumberType,
                                                                value: "1300",
                                                                ptr: None,
                                                                n: 0,
                                                                next: Some(
                                                                    Key {
                                                                        name: "height",
                                                                        value_type: NumberType,
                                                                        value: "1300",
                                                                        ptr: None,
                                                                        n: 0,
                                                                        next: Some(
                                                                            Key {
                                                                                name: "bit_depth",
                                                                                value_type: NumberType,
                                                                                value: "8",
                                                                                ptr: None,
                                                                                n: 0,
                                                                                next: Some(
                                                                                    Key {
                                                                                        name: "color_type",
                                                                                        value_type: StringType,
                                                                                        value: "rgb",
                                                                                        ptr: None,
                                                                                        n: 0,
                                                                                        next: Some(
                                                                                            Key {
                                                                                                name: "compression_method",
                                                                                                value_type: StringType,
                                                                                                value: "deflate",
                                                                                                ptr: None,
                                                                                                n: 0,
                                                                                                next: Some(
                                                                                                    Key {
                                                                                                        name: "filter_method",
                                                                                                        value_type: StringType,
                                                                                                        value: "adaptive",
                                                                                                        ptr: None,
                                                                                                        n: 0,
                                                                                                        next: Some(
                                                                                                            Key {
                                                                                                                name: "interlace_method",
                                                                                                                value_type: StringType,
                                                                                                                value: "none",
                                                                                                                ptr: None,
                                                                                                                n: 0,
                                                                                                                next: None,
                                                                                                                prev: None,
                                                                                                            },
                                                                                                        ),
                                                                                                        prev: None,
                                                                                                    },
                                                                                                ),
                                                                                                prev: None,
                                                                                            },
                                                                                        ),
                                                                                        prev: None,
                                                                                    },
                                                                                ),
                                                                                prev: None,
                                                                            },
                                                                        ),
                                                                        prev: None,
                                                                    },
                                                                ),
                                                                prev: None,
                                                            },
                                                        ),
                                                        n: 7,
                                                        next: None,
                                                        prev: None,
                                                    },
                                                ),
                                                n: 1,
                                                next: None,
                                                prev: None,
                                            },
                                        ),
                                        prev: None,
                                    },
                                ),
                                n: 2,
                                next: Some(
                                    Key {
                                        name: "",
                                        value_type: ObjectType,
                                        value: "",
                                        ptr: Some(
                                            Key {
                                                name: "type",
                                                value_type: StringType,
                                                value: "idat",
                                                ptr: None,
                                                n: 0,
                                                next: Some(
                                                    Key {
                                                        name: "data",
                                                        value_type: ArrayType,
                                                        value: "",
                                                        ptr: Some(
                                                            Key {
                                                                name: "",
                                                                value_type: ObjectType,
                                                                value: "",
                                                                ptr: Some(
                                                                    Key {
                                                                        name: "image_data",
                                                                        value_type: StringType,
                                                                        value: "255, 0, 0",
                                                                        ptr: None,
                                                                        n: 0,
                                                                        next: None,
                                                                        prev: None,
                                                                    },
                                                                ),
                                                                n: 1,
                                                                next: None,
                                                                prev: None,
                                                            },
                                                        ),
                                                        n: 1,
                                                        next: None,
                                                        prev: None,
                                                    },
                                                ),
                                                prev: None,
                                            },
                                        ),
                                        n: 2,
                                        next: None,
                                        prev: None,
                                    },
                                ),
                                prev: None,
                            },
                        ),
                        n: 2,
                        next: None,
                        prev: None,
                    },
                ),
                prev: None,
            },
        ),
        n: 2,
    },
)
```
---

## 🔧 Future Improvements

- [ ] Add pretty-printer for JSON output
- [ ] Add serialization (back to string from JsonObject)
- [ ] Support UTF-8 escape sequences in strings
- [ ] Add `from_str()` constructor for direct parsing
- [ ] Performance improvements (switch to Vecs?)
- [ ] Optional Serde interop

---

## 📜 License

This project is governed by a license, the details of which can be located in the accompanying file named 'LICENSE.' Please refer to this file for comprehensive information.

