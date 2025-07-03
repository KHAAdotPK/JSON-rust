# JSON-rust

A lightweight, handwritten JSON parsing library written in pure Rust — **without using any third-party JSON crates**.  
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

## 🚀 Usage

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/json-rust.git
cd json-rust
```

### 2. Build the project

```bash
cargo build
```

### 3. Run the parser

```bash
cargo run path/to/your.json
```

Example:

```bash
cargo run test.json
```

The output will print the parsed structure using Rust's `Debug` format.

---

## 📄 Example Input

Given the following `test.json`:

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

The program outputs:

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

## 📚 Learning Goals & Motivation

This project was created as a learning exercise to:

- Deeply understand JSON parsing
- Reinforce concepts of recursive data structures
- Practice Rust memory safety with ownership and borrowing
- Avoid using existing crates like `serde_json`

The entire parser — including the tokenizer, object model, and recursion — is built by hand, using just the standard Rust library and `regex`.

---

## 🧪 Testing

Manual testing is done by providing various JSON files and checking output.  
You can easily write integration tests inside `tests/` or extend the project with a proper test suite using `cargo test`.

---

## 🔧 Future Improvements

- [ ] Add pretty-printer for JSON output
- [ ] Add serialization (back to string from JsonObject)
- [ ] Support UTF-8 escape sequences in strings
- [ ] Add `from_str()` constructor for direct parsing
- [ ] Performance improvements (switch to Vecs?)
- [ ] Optional Serde interop

---

## 👨‍💻 Author

Created by [Q@khaa.pk](mailto:khaa@pk)  
Copyright © 2025  
All rights reserved.

---

## 📜 License

MIT License — Free to use, modify, and distribute with attribution.

