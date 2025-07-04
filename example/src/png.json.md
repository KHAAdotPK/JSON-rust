```
ROOT OBJECT {}
│
├─ "name": "test.png" (String)
│
└─ "chunks": ARRAY []
   │
   ├─ OBJECT {} (IHDR)
   │  ├─ "type": "ihdr" (String)
   │  └─ "data": ARRAY []
   │     └─ OBJECT {}
   │        ├─ "width": 1300 (Number)
   │        ├─ "height": 1300 (Number)
   │        └─ ... (Other keys)
   │
   └─ OBJECT {} (IDAT)
      ├─ "type": "idat" (String)
      └─ "data": ARRAY []
         └─ OBJECT {}
            └─ "image_data": "FF0000" (String)
```
```C++
enum ValueTypes {
    STRING_TYPE,
    NUMBER_TYPE,
    BOOLEAN_TYPE,
    NULL_TYPE,
    OBJECT_TYPE,
    ARRAY_TYPE
};

struct Key {
    private:
        char* name; // Null terminated
        ValueType type;
        char* value; // Null terminated
	JSON_KEY_PTR ptr;
	size_t n;
        struct key* next;
        struct key* prev;
    public:
        // Define getter and setters here
};

typedef struct key JSON_KEY;
typedef JSON_KEY* JSON_KEY_PTR;

struct JsonObject {
    JSON_KEY_PTR ptr;
    size_t n;
};
```            