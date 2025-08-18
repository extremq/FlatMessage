# Variant Enums

Variant enums (also known as algebraic data types or tagged unions) are supported for serialization and deserialization. They allow you to define an enum where each variant can contain different types of data.

| Data Type                                                 | Object | Slice | Vector | Option |
| --------------------------------------------------------- | ------ | ----- | ------ | ------ |
| Custom variant enums with `#[derive(FlatMessageVariant)]` | Yes    | -     | -      | Yes    |

**Supported variant types:**
- Basic types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `f32`, `f64`, `bool`
- Strings: `String`, `&str`
- Collections: `Vec<T>`, `&[T]` (where T is a supported type)
- Custom enums with `#[derive(FlatMessageEnum)]`
- Flags with `#[derive(FlatMessageFlags)]`
- Structs with `#[derive(FlatMessageStruct)]`
- Nested variant enums
- Unit variants (variants without data)
- Options: `Option<T>` for any supported type T

**Memory alignment:**
- Variant enums automatically determine their alignment based on the largest alignment requirement of their variants
- Supported alignments: 1, 2, 4, 8, 16 bytes
- When using variant enums in structs, you need to specify the alignment: `#[flat_message_item(kind = variant, align = N)]`

**Remarks:**
- Variant enums must derive `FlatMessageVariant`
- Each variant can contain at most one field (tuple-style variants)
- Named struct-style variants are not supported
- Unit variants (no associated data) are supported
- Variant enums can be marked as `#[sealed]` for stricter version compatibility
- Sealed variant enums include all variant names and types in their hash, making them incompatible with any version that adds, removes, or modifies variants
- Non-sealed variant enums only include the enum name in their hash, allowing forward compatibility when adding new variants
- When using complex types (enums, flags, structs) within variants, you must specify additional attributes: `#[flat_message_item(kind = enum/flags/struct, repr = type, align = N)]`
- Deserialization using `deserialize_from` validates variant types and data. If you are certain the data is valid, you can use `deserialize_from_unchecked` to skip validation and improve performance

## Examples

1. Basic variant enum with primitive types:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum Value {
        Integer(i32),
        Float(f64),
        Text(String),
        Flag(bool),
    }

    #[derive(Debug, FlatMessage)]
    struct Message {
        #[flat_message_item(kind = variant, align = 8)]
        data: Value,
    }
    ```

2. Variant enum with collections:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum Container {
        Numbers(Vec<i32>),
        Words(Vec<String>),
        Bytes(Vec<u8>),
        Empty,  // Unit variant
    }

    #[derive(Debug, FlatMessage)]
    struct Document {
        #[flat_message_item(kind = variant, align = 4)]
        content: Container,
    }
    ```

3. Variant enum with custom enums:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Priority {
        Low = 1,
        Medium = 2,
        High = 3,
    }

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum TaskData {
        Text(String),
        #[flat_message_item(kind = enum, repr = u8)]
        PriorityLevel(Priority),
        Urgent,  // Unit variant for urgent tasks
    }

    #[derive(Debug, FlatMessage)]
    struct Task {
        id: u32,
        #[flat_message_item(kind = variant, align = 1)]
        data: TaskData,
    }
    ```

4. Variant enum with flags:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(Read, Write, Execute)]
    struct Permissions(u8);
    impl Permissions {
        add_flag!(Read = 1);
        add_flag!(Write = 2);
        add_flag!(Execute = 4);
    }

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum FileInfo {
        Name(String),
        Size(u64),
        #[flat_message_item(kind = flags, repr = u8)]
        Access(Permissions),
    }

    #[derive(Debug, FlatMessage)]
    struct FileEntry {
        #[flat_message_item(kind = variant, align = 8)]
        info: FileInfo,
    }
    ```

5. Variant enum with structs:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum ShapeData {
        #[flat_message_item(kind = struct, align = 4)]
        Position(Point),
        #[flat_message_item(kind = struct, align = 1)]
        Appearance(Color),
        Label(String),
    }

    #[derive(Debug, FlatMessage)]
    struct Shape {
        #[flat_message_item(kind = variant, align = 4)]
        data: ShapeData,
    }
    ```

6. Variant enum with Option types:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum OptionalData {
        Text(Option<String>),
        Number(Option<i32>),
        Present,
        Absent,
    }

    #[derive(Debug, FlatMessage)]
    struct Record {
        #[flat_message_item(kind = variant, align = 4)]
        data: OptionalData,
    }
    ```

7. Sealed variant enum for strict validation:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    #[sealed]
    enum Protocol {
        Http(String),
        Https(String),
        Ftp(String),
        Unknown,
    }

    #[derive(Debug, FlatMessage)]
    struct Connection {
        #[flat_message_item(kind = variant, align = 1)]
        protocol: Protocol,
    }
    ```

8. Using Option with variant enums:
    ```rust
    use flat_message::*;

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum Response {
        Success(String),
        Error(u32),
        Timeout,
    }

    #[derive(Debug, FlatMessage)]
    struct ApiCall {
        endpoint: String,
        #[flat_message_item(kind = variant, align = 4)]
        response: Option<Response>,  // Optional response
    }
    ```

## Version Compatibility

Variant enums support forward compatibility when adding new variants:

```rust
// Version 1
#[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
enum Message {
    Text(String),
    Number(i32),
}

// Version 2 - compatible with version 1 data
#[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
enum Message {
    Text(String),
    Number(i32),
    Binary(Vec<u8>),  // New variant added
    Timestamp(u64),   // Another new variant
}
```

**Compatibility rules:**
- **Non-sealed variants**: Data serialized with version 1 can be successfully deserialized with version 2, as long as the specific variants used in the serialized data exist in both versions
- **Sealed variants**: Will fail to deserialize if any variants have been added, removed, or modified between versions
- **Existing variants**: Must maintain the same type signature and attributes across versions
- **New variants**: Can be safely added to non-sealed variant enums without breaking compatibility

**Breaking changes:**
- Removing variants
- Changing the type of data in existing variants
- Changing attributes (kind, repr, align) of existing variants
- Making a non-sealed variant enum sealed
