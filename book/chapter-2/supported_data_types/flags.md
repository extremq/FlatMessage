# Flags

Flags (bit fields) with explicitly defined backing types are supported for serialization and deserialization.

| Data Type                                                                  | Object | Slice | Vector | Option |
| -------------------------------------------------------------------------- | ------ | ----- | ------ | ------ |
| Custom flags with `#[derive(FlatMessageFlags)]` and `#[repr(transparent)]` | Yes    | Yes   | Yes    | Yes    |

**Supported representations:**
- `u8`, `u16`, `u32`, `u64`, `u128`

**Remarks:**
- Flags must derive `FlatMessageFlags` and have an explicit `#[repr(transparent)]` attribute.
- Flags must declare available flag names in the `#[flags(...)]` attribute.
- Individual flag values are defined using the `add_flag!` macro or as public constants.
- When using flags in structs, you must specify both the representation and kind in the field attribute: `#[flat_message_item(repr = u8, kind = flags)]`.
- Flags can be marked as `#[sealed]` for stricter version compatibility. Sealed flags include all flag names and values in their hash, making them incompatible with any version that adds, removes, or modifies flags. Non-sealed flags only include the flag struct name in their hash, allowing forward compatibility when adding new flags.
- Both sealed and non-sealed flags validate that deserialized values contain only valid flag combinations. If invalid flags are detected during deserialization, an error is returned.
- Deserialization using `deserialize_from` validates flag values. If you are certain the data is valid, you can use `deserialize_from_unchecked` to skip validation and improve performance.

## Available Methods

Flags automatically implement the `FlagsSupport` trait, providing the following methods:
- `from_value(value: T)` - Creates flags from raw value, validates against known flags
- `to_value(&self)` - Returns the raw underlying value
- `any_set(&self, flag: Self)` - Checks if any of the specified flags are set
- `all_set(&self, flag: Self)` - Checks if all of the specified flags are set
- `is_empty(&self)` - Checks if no flags are set
- `set(&mut self, flag: Self)` - Sets the specified flags
- `unset(&mut self, flag: Self)` - Unsets the specified flags
- `toggle(&mut self, flag: Self)` - Toggles the specified flags
- `clear(&mut self)` - Clears all flags

Flags also support standard bitwise operations: `|` (OR), `&` (AND), `^` (XOR), and their assignment variants (`|=`, `&=`, `^=`).

## Example

1. Basic flags usage:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    #[flags(Read, Write, Execute)]
    struct Permissions(u8);

    impl Permissions {
        add_flag!(Read = 1);
        add_flag!(Write = 2);
        add_flag!(Execute = 4);
    }

    #[derive(Debug, FlatMessage)]
    struct FileInfo {
        #[flat_message_item(repr = u8, kind = flags)]
        permissions: Permissions,
    }
    ```

2. Using flags with bit operations:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C)]
    struct Features(u32);

    impl Features {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
    }

    // Combining flags
    let mut features = Features::A | Features::B;
    
    // Checking flags
    assert!(features.all_set(Features::A));
    assert!(features.any_set(Features::A | Features::C));
    
    // Modifying flags
    features.set(Features::C);
    features.unset(Features::A);
    assert!(features.all_set(Features::B | Features::C));
    ```

3. Flags slices:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    #[flags(Low, Medium, High)]
    struct Priority(u16);

    impl Priority {
        add_flag!(Low = 1);
        add_flag!(Medium = 2);
        add_flag!(High = 4);
    }

    #[derive(Debug, FlatMessage)]
    struct TaskList<'a> {
        #[flat_message_item(repr = u16, kind = flags)]
        priorities: &'a [Priority],
    }
    ```

4. Flags vectors:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    #[flags(Debug, Info, Warning, Error)]
    struct LogLevel(u32);

    impl LogLevel {
        add_flag!(Debug = 1);
        add_flag!(Info = 2);
        add_flag!(Warning = 4);
        add_flag!(Error = 8);
    }

    #[derive(Debug, FlatMessage)]
    struct LogConfig {
        #[flat_message_item(repr = u32, kind = flags)]
        enabled_levels: Vec<LogLevel>,
    }
    ```

5. Sealed flags for strict validation:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    #[sealed]
    #[flags(Http, Https, Ftp)]
    struct Protocol(u8);

    impl Protocol {
        add_flag!(Http = 1);
        add_flag!(Https = 2);
        add_flag!(Ftp = 4);
    }

    #[derive(Debug, FlatMessage)]
    struct Connection {
        #[flat_message_item(repr = u8, kind = flags)]
        supported_protocols: Protocol,
    }
    ```

6. Using Option with flags:
    ```rust
    use flat_message::*;

    #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    #[flags(Cache, Compress, Encrypt)]
    struct Options(u64);

    impl Options {
        add_flag!(Cache = 1);
        add_flag!(Compress = 2);
        add_flag!(Encrypt = 4);
    }

    #[derive(Debug, FlatMessage)]
    struct RequestConfig {
        #[flat_message_item(repr = u64, kind = flags)]
        options: Option<Options>,
    }
    ```

## Version Compatibility

Flags support forward compatibility when adding new flags:

```rust
// Version 1
#[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug)]
#[repr(transparent)]
#[flags(Read, Write)]
struct Permissions(u8);

impl Permissions {
    add_flag!(Read = 1);
    add_flag!(Write = 2);
}

// Version 2 - compatible with version 1 data
#[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug)]
#[repr(u8)]
#[flags(Read, Write, Execute)]
struct Permissions(u8);

impl Permissions {
    add_flag!(Read = 1);
    add_flag!(Write = 2);
    add_flag!(Execute = 4);  // New flag added
}
```

Data serialized with version 1 can be successfully deserialized with version 2 for non-sealed flags, as long as the specific flag values used in the serialized data are valid in both versions. However, if version 2 data contains new flags (like `Execute`) and is deserialized with version 1, it will fail validation since version 1 doesn't recognize the new flag.

Sealed flags will fail to deserialize if any flags have been added, removed, or modified between versions, providing stricter version control at the cost of forward compatibility.