# Enums

Enums with explicitly defined backing types are supported for serialization and deserialization.

| Data Type                                                               | Object | Slice | Vector | Option |
| ----------------------------------------------------------------------- | ------ | ----- | ------ | ------ |
| Custom enums with `#[derive(FlatMessageEnum)]` and `#[repr(primitive)]` | Yes    | Yes   | Yes    | Yes    |

**Supported representations:**
- `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`

**Remarks:**
- Enums must derive `FlatMessageEnum` and have an explicit `#[repr(...)]` attribute specifying the underlying primitive type.
- Enum variants can have explicit values assigned, or they will use the default incrementing values starting from 0.
- When using enums in structs, you must specify both the representation and kind in the field attribute: `#[flat_message_item(repr = u8, kind = enum)]`.
- Enums can be marked as `#[sealed]` for stricter version compatibility. Sealed enums include all variant names and values in their hash, making them incompatible with any version that adds, removes, or modifies variants. Non-sealed enums only include the enum name in their hash, allowing forward compatibility when adding new variants.
- Both sealed and non-sealed enums validate that deserialized values match known variants in the current enum definition. The difference is in version compatibility: non-sealed enums allow adding variants without breaking hash compatibility, while sealed enums will fail to deserialize if the enum definition has changed in any way.
- Deserialization using `deserialize_from` validates enum variant values. If you are certain the data is valid, you can use `deserialize_from_unchecked` to skip validation and improve performance.

## Example

1. Basic enum usage:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Color {
        Red = 1,
        Green = 10,
        Blue = 100,
    }

    #[derive(Debug, FlatMessage)]
    struct Example {
        #[flat_message_item(repr = u8, kind = enum)]
        color: Color,
    }
    ```

2. Enum slices:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u16)]
    enum Priority {
        Low = 1,
        Medium = 100,
        High = 1000,
    }

    #[derive(Debug, FlatMessage)]
    struct Example<'a> {
        #[flat_message_item(repr = u16, kind = enum)]
        priorities: &'a [Priority],
    }
    ```

3. Enum vectors:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Status {
        Pending = 1,
        Processing = 2,
        Complete = 3,
    }

    #[derive(Debug, FlatMessage)]
    struct Example {
        #[flat_message_item(repr = u32, kind = enum)]
        statuses: Vec<Status>,
    }
    ```

4. Sealed enums for strict validation:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    #[sealed]
    enum Protocol {
        Http = 1,
        Https = 2,
        Ftp = 3,
    }

    #[derive(Debug, FlatMessage)]
    struct Example {
        #[flat_message_item(repr = u8, kind = enum)]
        protocol: Protocol,
    }
    ```

5. Using Option with enums:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(i16)]
    enum Temperature {
        Freezing = -100,
        Cold = -10,
        Warm = 20,
        Hot = 40,
    }

    #[derive(Debug, FlatMessage)]
    struct Example {
        #[flat_message_item(repr = i16, kind = enum)]
        temperature: Option<Temperature>,
    }
    ```

## Version Compatibility

Enums support forward compatibility when adding new variants:

```rust
// Version 1
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 10,
    Blue = 100,
}

// Version 2 - compatible with version 1 data
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 10,
    Blue = 100,
    Yellow = 200,  // New variant added
}
```

Data serialized with version 1 can be successfully deserialized with version 2 for non-sealed enums, as long as the specific variant values used in the serialized data exist in both versions. Sealed enums will fail to deserialize if any variants have been added, removed, or modified between versions.