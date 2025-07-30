/*
    src/json_object.rs
    This file is part of the JSON-rust crate.
    Written by, Q@khaa.pk
 */

/// Represents the possible types of JSON values.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    StringType,    // A string value
    NumberType,    // A numeric value
    BooleanType,   // A boolean value (true/false)
    NullType,      // A null value
    ObjectType,    // A nested JSON object
    ArrayType,     // A JSON array
}

/// Represents a key-value pair in a JSON object with linked list capabilities.
/// Each Key contains metadata about the value it holds and can link to other keys.
#[derive(Debug, Clone)]
pub struct Key {
    name: String,           // The name (key) of the JSON property
    value_type: ValueType,  // The type of the associated value
    value: String,          // The string representation of the value
    ptr: JsonKeyPtr,        // Optional pointer to another Key (for nested structures)
    n: usize,               // Possibly a count or size (usage context dependent), number of keys in the object
    next: Option<Box<Key>>, // Next key in the linked list
    prev: Option<Box<Key>>, // Previous key in the linked list
}

impl Key {
    /// Creates a new Key instance.
    ///
    /// # Arguments
    /// * `name` - The name of the JSON property
    /// * `value_type` - The type of the value
    /// * `value` - The string representation of the value
    pub fn new(name: String, value_type: ValueType, value: String) -> Self {
        Key {
            name: name,
            value_type: value_type,
            value: value,
            ptr: None,
            n: 0,
            next: None,
            prev: None,
        }
    }

    pub fn get_ptr(&self) -> &JsonKeyPtr {
        &self.ptr
    }

    pub fn get_n(&self) -> usize {
        self.n
    }

    /// Returns a reference to the key's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the key's value type.
    pub fn get_value_type(&self) -> &ValueType {
        &self.value_type
    }

    /// Returns a reference to the key's value.
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Returns a reference to the next key in the linked list, if any.
    pub fn get_next(&self) -> Option<&Box<Key>> {
        self.next.as_ref()
    }

    /// Returns a reference to the previous key in the linked list, if any.
    pub fn get_prev(&self) -> Option<&Box<Key>> {
        self.prev.as_ref()
    }

    /// Sets the next key in the linked list.
    ///
    /// # Arguments
    /// * `next` - The next key to link to
    pub fn set_next(&mut self, next: Option<Box<Key>>) {
        self.next = next;
    }

    /// Sets the previous key in the linked list.
    ///
    /// # Arguments
    /// * `prev` - The previous key to link to
    pub fn set_prev(&mut self, prev: Option<Box<Key>>) {
        self.prev = prev;
    }

    /// Updates the name of the key.
    ///
    /// # Arguments
    /// * `name` - New name for the key
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Updates the value type of the key.
    ///
    /// # Arguments
    /// * `value_type` - New value type
    pub fn set_value_type(&mut self, value_type: ValueType) {
        self.value_type = value_type;
    }

    /// Updates the value of the key.
    ///
    /// # Arguments
    /// * `value` - New value
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    /// Adds a key to the nested structure pointed to by `ptr`.
    /// This is used for creating nested objects or arrays within a key.
    ///
    /// # Arguments
    /// * `key` - The boxed `Key` to add to the nested list.
    pub fn add_key(&mut self, key: Box<Key>) {    
        match &mut self.ptr {
            None => {
                self.ptr = Some(key);
                self.n = 1;
            },
            Some(ref mut head) => {
                let mut current = head;
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(key);
                self.n += 1;
            }
        }
    }
}

/// Type alias for an optional boxed Key, representing a nullable pointer to a Key.
pub type JsonKeyPtr = Option<Box<Key>>;

/// Represents a JSON object as a linked list of Key-value pairs.
#[derive(Debug, Clone)]
pub struct JsonObject {
    ptr: JsonKeyPtr,  // Pointer to the first key in the object
    n: usize,         // Number of keys in the object
}

impl JsonObject {
    /// Creates a new, empty JsonObject.
    pub fn new() -> Self {
        JsonObject {
            ptr: None,
            n: 0,
        }
    }

    /// Sets the pointer to the first key in the object.
    ///
    /// # Arguments
    /// * `ptr` - New pointer to set
    pub fn set_ptr(&mut self, ptr: JsonKeyPtr) {
        self.ptr = ptr;
    }

    /// Returns a reference to the pointer to the first key.
    pub fn get_ptr(&self) -> &JsonKeyPtr {
        &self.ptr
    }

    /// Returns the count of keys in the object.
    pub fn get_n(&self) -> usize {
        self.n
    }

    /// Sets the count of keys in the object.
    ///
    /// # Arguments
    /// * `n` - New count to set
    pub fn set_n(&mut self, n: usize) {
        self.n = n;
    }

    /// Adds a new key to the end of the JSON object's linked list.
    ///
    /// This function handles two cases:
    /// 1. When the list is empty (head pointer is None)
    /// 2. When the list contains one or more nodes
    ///
    /// Uses pattern matching for cleaner control flow and safer pointer access.
    ///
    /// # Arguments
    /// * `key` - A boxed Key node to be appended to the list (takes ownership)
    ///
    /// # Invariants
    /// - Maintains accurate count of nodes (self.n)
    /// - Preserves list linkage integrity
    /// - Never leaks memory or creates dangling pointers
    pub fn add_key(&mut self, mut key: Box<Key>) {
        // Use pattern matching to handle both empty and non-empty cases
        match &mut self.ptr {
            // Case 1: Empty list
            None => {

                /*if key.get_name() == String::new() {
                    
                    key.set_name("JSON".to_string());
                }*/

                // Set the new key as the head of the list
                self.ptr = Some(key);
                // Initialize node count to 1
                self.n = 1;
            },
        
            // Case 2: Non-empty list
            Some(ref mut head) => {
                // Start traversal at the head node
                let mut current = head;
            
                // Use while-let pattern matching to traverse the list
                // This is safer than manual unwrapping and more idiomatic
                while let Some(ref mut next) = current.next {
                    // Move to the next node
                    current = next;
                }
            
                // After loop exits, current points to the last node
                // Append the new key
                current.next = Some(key);
                // Increment the node counter
                self.n += 1;
            }
        }
    }

    /// Prints a formatted representation of the JSON object to the console.
    /// This method is useful for debugging and visualizing the structure of the JSON data.
    /// It recursively prints nested objects and arrays with indentation.
    pub fn pretty_print(&self) {
        
        fn print_key(key: &Box<Key>, indent: usize) {
            println!("{}{}: {:?}", " ".repeat(indent), key.get_name(), key.get_value_type());
            if let Some(ref nested) = key.ptr {
                print_key(nested, indent + 4);
            }
            if let Some(ref next) = key.next {
                print_key(next, indent);
            }
        }

        if let Some(ref key) = self.ptr {
            print_key(key, 0);
        }
    }

    /// Adds a new key to the end of the JSON object's linked list.
    ///
    /// This function maintains the linked list structure by:
    /// 1. Handling the empty list case separately
    /// 2. Traversing to the end of the list for non-empty cases
    /// 3. Properly updating the node count
    ///
    /// # Arguments
    /// * `key` - A boxed Key node to be added to the list (takes ownership)
    ///
    /// # Examples
    /// ```
    /// let mut obj = JsonObject::new();
    /// obj.add_key(Box::new(Key::new("name".into(), ValueType::StringType, "John".into())));
    /// ```
    pub fn add_key_new(&mut self, key: Box<Key>) {
        // Case 1: The list is empty
        if self.ptr.is_none() {
            // Set this key as the first and only node in the list
            self.ptr = Some(key);
            // Initialize count to 1 since we're adding the first element
            self.n = 1;
        } 
        // Case 2: The list contains nodes
        else {
            // Start traversal at the head node
            // as_mut() gives us a mutable reference without taking ownership
            let mut current = self.ptr.as_mut().unwrap();
        
            // Traverse the list until we find the last node
            // The last node is identified by having next = None
            while current.next.is_some() {
                // Move to the next node
                // as_mut().unwrap() is safe here because we checked is_some()
                current = current.next.as_mut().unwrap();
            }
        
            // We've reached the last node - append the new key
            current.next = Some(key);
            // Increment the node counter
            self.n += 1;
        }
    }

    /// Adds a new key to the end of the object's linked list.
    ///
    /// # Arguments
    /// * `key` - The key to add
    pub fn add_key_old(&mut self, key: Box<Key>) {

        if self.ptr.is_none() {
            // If object is empty, set this as the first key
            self.ptr = Some(key);
            self.n = 1;

        } else {

            // Otherwise, traverse to the end of the list and append
            let mut current = self.ptr.as_mut().unwrap();
    
            while current.next.is_some() {

                current = current.next.as_mut().unwrap();
            }

            current.next = Some(key);            
            self.n = self.n + 1;
        }
    }
}

