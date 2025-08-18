# Deriving FlatMessage

To make a struct serializable with FlatMessage, simply add `#[derive(FlatMessage)]`:

```rust
use flat_message::*;

#[derive(FlatMessage)]
struct Person {
    name: String,
    age: u32,
    active: bool,
}
```

This automatically generates the necessary code to implement the `FlatMessage` trait for your struct.

## Structure-Level Configuration (Optional)

You can configure the entire structure using the `#[flat_message_options(...)]` attribute in the following way:

```
#[flat_message_options(option-1 : value-1, option-2 : value-2, ...)]
```
or
```
#[flat_message_options(option-1 = value-1, option-2 = value-2, ...)]
```

For example:

```rust
#[derive(FlatMessage)]
#[flat_message_options(store_name = false, version = 1, checksum = true)]
struct OptimizedStruct {
    data: Vec<u8>,
}
```

### Available Structure Options (Optional)

| Option                     | Type                                | Default  | Description                                                                                                                                                                                |
| -------------------------- | ----------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `store_name`               | *bool*                              | *true*   | Whether to store a hash of the structure name in the serialized data                                                                                                                       |
| `version`                  | *u8*                                | *0*      | Version number for this structure (1-255). Value `0` means that the structure is not versioned.                                                                                            |
| `checksum`                 | *bool*                              | *false*  | Whether to include CRC32 checksum for data integrity                                                                                                                                       |
| `validate_name`            | *bool*                              | *false*  | Whether to validate structure name during deserialization (this implies that the `store_name` is also set to `true`)                                                                       |
| `validate_checksum`        | *"auto"* or *"always"* or *"never"* | *"auto"* | When to validate checksums                                                                                                                                                                 |
| `compatible_versions`      | *string*                            | none     | Version compatibility specification                                                                                                                                                        |
| `optimized_unchecked_code` | *bool*                              | *true*   | Whether to generate optimized unchecked code for deserialization or not. If not set the code generated for `deserialize_from_unchecked` will be the same as the one for `deserialize_from` |

**Remarks:** 
- The `store_name` option does not store the actual structure name, but a hash of it. That hash is being used to check if the structure you are deserializing into is the same as the one you serialized. However, this is not always neccesary (especially when talking about versioning and compabibility). If this is not needed, you should set the `store_name` option to `false` to save some space on the serialized buffer.
- You can read more about versioning and compabibility in the [Versioning](../chapter-3/versioning.md) chapter.
- The `checksum` option is set to `false` by default. If set to `true`, the CRC32 checksum of the serialized data is being calculated and stored in the serialized buffer. This is useful to ensure data integrity during deserialization (usually when you are sending data over a network). You can read more on how checksums and validation work in the [Checksum and Validation](../chapter-3/checksums_and_validation.md) chapter.

## Field-Level Configuration

Individual fields can be configured using `#[flat_message_item(...)]` in the following way:
```
#[flat_message_item(option-1 : value-1, option-2 : value-2, ...)]
```
or
```
#[flat_message_item(option-1 = value-1, option-2 = value-2, ...)]
```

This is useful when you want to specify how a field should be serialized / deserialized. 
The following example shows how to use the `#[flat_message_item(repr = u8, kind = enum)]` attribute to serialize an enum as a `u8` value.

```rust
#[derive(FlatMessage)]
struct Example {
    normal_field: u32,
    
    #[flat_message_item(repr = u8, kind = enum)]
    status: Status,
}

#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Status {
    Active = 1,
    Inactive = 2,
}
```

**Remarks:**
- You can read more on how enums are being serialized in the [Enums](../chapter-2/supported_data_types/enums.md) chapter or the [Flags](../chapter-2/supported_data_types/flags.md) chapter.

### Field Options

| Option             | Values                                               | Description                                                 |
| ------------------ | ---------------------------------------------------- | ----------------------------------------------------------- |
| `repr`             | `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64` | Representation type                                         |
| `kind`             | `enum`, `flags`, `struct`, `variant`                 | Marks field as enum , flags, variant or a structure type    |
| `align`            | `1`, `2`, `4`, `8`, `16`                             | Alignment of the field (only for structures and variants)   |
| `ignore` or `skip` | `true` or `false` (default is **false**)             | Ignores the field during serialization and deserialization  |
| `mandatory`        | `true` or `false` (default is **true**)              | Marks the field as mandatory (required) for deserialization |

**Remarks:**
- Fields of type `PhantomData<T>` are automatically ignored during serialization:
    ```rust
    use std::marker::PhantomData;

    #[derive(FlatMessage)]
    struct GenericStruct<T> {
        data: u32,
        _phantom: PhantomData<T>,  // This field is ignored
    }
    ```
- You can use the `ignore` or `skip` option to ignore a field during serialization and deserialization. This is useful when you want to skip a field that is not part of the structure you are deserializing into. The fields have to implement the `Default` trait.
    ```rust
    use std::marker::PhantomData;

    #[derive(Default)]
    struct MyNonSerializableData {
        a: u8,
        b: u32,
    }
    #[derive(FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Data {
        x: u8,
        #[flat_message_item(ignore = true)]
        y: MyNonSerializableData,
    }
    ```
- Mandatory fields are required for deserialization. If a mandatory field is not present in the serialized data, the deserialization will fail. On the other hand, if a field is not mandatory, and it is not found in the serialized data or there are some issues trying to deserialize it, it will be defaulted to the default value of the type. This implies that the trait `Default` is implemented for that type.

## Generated Code

When you derive `FlatMessage`, the following methods are automatically implemented:

```rust
impl<'a> FlatMessage<'a> for YourStruct {
    fn serialize_to(&self, output: &mut Storage, config: Config) -> Result<(), Error>;
    fn deserialize_from(input: &'a Storage) -> Result<Self, Error>;
    unsafe fn deserialize_from_unchecked(input: &'a Storage) -> Result<Self, Error>;
}
```

## Complete Example

```rust
use flat_message::*;

#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(FlatMessage, Debug, PartialEq)]
#[flat_message_options(version = 1, store_name = true, checksum = true)]
struct Task {
    title: String,
    description: Option<String>,
    completed: bool,
    
    #[flat_message_item(repr = u8, kind = enum)]
    priority: Priority,
    
    tags: Vec<String>,
    created: Timestamp,
    id: UniqueID,
}

fn main() {
    let task = Task {
        title: "Learn FlatMessage".to_string(),
        description: Some("Read the documentation".to_string()),
        completed: false,
        priority: Priority::High,
        tags: vec!["learning".to_string(), "rust".to_string()],
    };

    // Create a serialization storage buffer
    let mut storage = Storage::default();
    if let Err(e) = task.serialize_to(&mut storage, Config::default()) {
        panic!("Error serializing task: {}", e);
    }

    // print the buffer
    println!("Buffer: {:?}", storage.as_slice());

    // Deserialize from buffer
    match Task::deserialize_from(&storage) {
        Ok(restored_task) => {
            assert_eq!(task, restored_task);
            println!("Task serialized and deserialized successfully");
        }
        Err(e) => {
            panic!("Error deserializing task: {}", e);
        }
    }
}
```

Upon execution, the following output is being printed:

```
Buffer: [70, 76, 77, 1, 5, 0, 1, 12, 22, 82, 101, 97, 100, 32, 116, 104, 101, 
         32, 100, 111, 99, 117, 109, 101, 110, 116, 97, 116, 105, 111, 110, 113, 
         190, 208, 155, 3, 17, 76, 101, 97, 114, 110, 32, 70, 108, 97, 116, 77, 
         101, 115, 115, 97, 103, 101, 2, 8, 108, 101, 97, 114, 110, 105, 110, 103, 
         4, 114, 117, 115, 116, 0, 0, 0, 14, 59, 111, 52, 19, 227, 228, 148, 14, 
         181, 101, 152, 142, 235, 22, 244, 13, 129, 116, 244, 8, 31, 36, 54, 69, 
         108, 136, 72, 244, 245, 75, 146, 228]
Task serialized and deserialized successfully
```
