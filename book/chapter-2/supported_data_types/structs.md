# Structures

Custom structs with explicit alignment are supported for serialization and deserialization.

| Data Type                                          | Object | Slice | Vector | Option |
| -------------------------------------------------- | ------ | ----- | ------ | ------ |
| Custom structs with `#[derive(FlatMessageStruct)]` | Yes    | -     | -      | Yes    |

**Supported alignments:**
- 4-byte alignment (default)
- 8-byte alignment (if one of the fields requires 64 bits alignament - such as Vec<u64>)
- 16-byte alignment (if one of the fields requires 128 bits alignament - such as Vec<u128>)

**Remarks:**
- Structs must derive `FlatMessageStruct` to be used as nested structures within other FlatMessage types.
- When using structs in other structs, you must specify the alignment in the field attribute: `#[flat_message_item(align = 4, kind = struct)]`.
- The alignment must match the struct's actual memory alignment requirements based on its fields.
- Structs automatically determine their required alignment based on their largest field's alignment requirements.
- This type of serialization does not support metadata fields like `Timestamp` and `UniqueID`. You can add them but they will ont be serialized and in deserialization phase they will be defaulted to 0.
- Fields can be marked with `#[flat_message_item(ignore = true)]` to exclude them from serialization.


## Example

1. Basic struct usage:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyData {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }

    #[derive(Debug, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        data: MyData,
        y: u8,
    }
    ```

2. Struct with 8-byte alignment (contains u64 vectors or slices):
    ```rust
    use flat_message::*;

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyData {
        a: u8,
        b: u32,
        c: u16,
        d: String,
        values: Vec<u64>,  // Requires 8-byte alignment
    }

    #[derive(Debug, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 8, kind = struct)]
        data: MyData,
        y: u8,
    }
    ```

3. Struct with 16-byte alignment (contains u128 vectors or slices):
    ```rust
    use flat_message::*;

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyData {
        a: u8,
        values: Vec<u128>,  // Requires 16-byte alignment
    }

    #[derive(Debug, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 16, kind = struct)]
        data: MyData,
        y: u8,
    }
    ```

4. Structs with metadata fields:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct EventData {
        a: u8,
        b: u32,
        timestamp: Timestamp,  // ignored - will be defaulted to 0
        unique_id: UniqueID,   // ignored - will be defaulted to 0
    }

    #[derive(Debug, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Event {
        #[flat_message_item(align = 4, kind = struct)]
        data: EventData,
    }
    ```

5. Using Option with structs:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct Configuration {
        timeout: u32,
        retries: u8,
    }

    #[derive(Debug, FlatMessage)]
    struct Request {
        #[flat_message_item(align = 4, kind = struct)]
        config: Option<Configuration>,
    }
    ```

6. Structs with ignored fields:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct ProcessData {
        pid: u32,
        name: String,
        #[flat_message_item(ignore = true)]
        runtime_state: String,  // Not serialized
    }

    #[derive(Debug, FlatMessage)]
    struct ProcessList {
        #[flat_message_item(align = 4, kind = struct)]
        processes: Vec<ProcessData>,
    }
    ```

7. Nested structs:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct LevelTwo {
        a: bool,
        l: Vec<i8>,
    }
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct LevelOne {
        a: u8,
        b: u32,
        c: u16,
        d: String,
        #[flat_message_item(align = 4, kind = struct)]
        e: LevelTwo,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: LevelOne,
        a: u8,
    }
    ```

## Serialization Behavior

When structs are serialized:

1. **Field Ordering**: Fields are reordered during serialization based on their alignment requirements (largest alignment first) to optimize memory layout.

2. **Hash Table**: Each struct maintains a hash table of its fields for efficient deserialization and version compatibility.

3. **Reference Table**: Offset information for each field is stored to enable random access during deserialization.


