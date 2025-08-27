# Field Value Validation

Field value validation in FlatMessage provides a powerful mechanism for handling deserialization failures gracefully, making it particularly useful for versioning scenarios. The `validate` attribute allows you to specify how the system should behave when a field cannot be deserialized successfully.

## Validation Modes

FlatMessage supports two validation modes:

- **`validate = strict`** (default): Deserialization fails immediately if any field cannot be deserialized
- **`validate = fallback`**: Falls back to the field's default value if deserialization fails

## Syntax

The `validate` attribute can be applied at two levels:

### Structure Level
```rust
#[derive(FlatMessage)]
#[flat_message_options(validate = fallback)]
struct MyStruct {
    // All fields will use fallback validation by default
}
```

### Field Level
```rust
#[derive(FlatMessage)]
struct MyStruct {
    #[flat_message_item(validate = strict)]
    critical_field: u32,
    
    #[flat_message_item(validate = fallback)]
    optional_field: Color,
}
```
**Remarks:**: The structure level `validate` attribute can be overridden at the field level (by useing #[flat_message_item(validate = "...")]). 


## Common Use Cases

### 1. Enum Evolution with Backward Compatibility

When adding new variants to enums, `validate = fallback` allows older code to handle newer enum values gracefully:

```rust
// Version 1
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum Color {
    #[default]
    Red = 1,
    Green = 10,
    Blue = 100,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false)]
pub struct TestStruct {
    pub value: u8,
    #[flat_message_item(repr = u8, kind = enum, validate = fallback)]
    pub color: Color,
}
```

```rust
// Version 2 - adds Yellow variant
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Color {
    Red = 1,
    Green = 10,
    Blue = 100,
    Yellow = 200,  // New variant
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false)]
pub struct TestStruct {
    pub value: u8,
    #[flat_message_item(repr = u8, kind = enum)]
    pub color: Color,
}
```

When v1 code tries to deserialize data containing `Yellow` (value 200), it will fallback to the default `Red` value instead of failing:

```rust
// v2 serializes data with Yellow
let d_v2 = v2::TestStruct { value: 1, color: v2::Color::Yellow };
d_v2.serialize_to(&mut storage, Config::default()).unwrap();

// v1 deserializes successfully with fallback to default
let d_v1 = v1::TestStruct::deserialize_from(&storage).unwrap();
assert_eq!(d_v1.value, 1);
assert_eq!(d_v1.color, v1::Color::Red); // Fallback to default
```

### 2. Flags Evolution with Backward Compatibility

Similar to enums, flags can evolve by adding new flag variants. Using `validate = fallback` allows older code to handle newer flag combinations gracefully:

```rust
// Version 1
#[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
#[flags(A,B)]
pub struct Flags(u8);
impl Flags {
    add_flag!(A = 1);
    add_flag!(B = 2);
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false)]
pub struct TestStruct {
    pub value: u8,
    #[flat_message_item(repr = u8, kind = flags, validate = fallback)]
    pub flags: Flags,
}
```

```rust
// Version 2 - adds C flag
#[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
#[flags(A,B,C)]
pub struct Flags(u8);
impl Flags {
    add_flag!(A = 1);
    add_flag!(B = 2);
    add_flag!(C = 4);  // New flag
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false)]
pub struct TestStruct {
    pub value: u8,
    #[flat_message_item(repr = u8, kind = flags)]
    pub flags: Flags,
}
```

When v1 code tries to deserialize data containing the new `C` flag, it will fallback to the default empty flags instead of failing:

```rust
// v2 serializes data with C flag (unknown to v1)
let d_v2 = v2::TestStruct { 
    value: 1, 
    flags: v2::Flags::C | v2::Flags::B 
};
d_v2.serialize_to(&mut storage, Config::default()).unwrap();

// v1 deserializes successfully with fallback to default
let d_v1 = v1::TestStruct::deserialize_from(&storage).unwrap();
assert_eq!(d_v1.value, 1);
assert!(d_v1.flags.is_empty()); // Fallback to default (empty flags)
```

### 3. Graceful Schema Evolution

Fallback validation enables smooth transitions when field types change or become incompatible:

```rust
#[derive(FlatMessage)]
#[flat_message_options(validate = fallback)] // Structure-level fallback validation
struct Configuration {
    #[flat_message_item(default = 30)]
    timeout_seconds: u32,
    
    log_level: LogLevel, // Uses structure-level fallback validation
    
    #[flat_message_item(validate = strict)] // Override to strict for critical field
    database_url: String, // Critical field - must not fail
}
```

## How Fallback Validation Works

When `validate = fallback` is specified:

1. **Normal Operation**: If the field can be deserialized normally, it uses the stored value
2. **Fallback Trigger**: If deserialization fails (e.g., enum variant not found, type mismatch), the system:
   - Uses the `default` value if specified in the attribute
   - Uses the type's `Default::default()` implementation
   - For `Option<T>` types, defaults to `None`

## Best Practices

### When to Use Strict Validation
- Critical fields where data integrity is essential
- When you need to detect and handle incompatibilities explicitly
- Fields where a default value doesn't make semantic sense

### When to Use Fallback Validation
- Optional or non-critical fields
- Enum/Flags fields that may evolve over time
- During data migration periods
- When maintaining backward compatibility is important

### Combining with Other Attributes
```rust
#[derive(FlatMessage)]
struct FlexibleStruct {
    // Critical field - must be present and valid
    #[flat_message_item(mandatory = true, validate = strict)]
    user_id: u64,
    
    // Optional field with fallback and custom default
    #[flat_message_item(
        mandatory = false, 
        validate = fallback, 
        default = "guest"
    )]
    username: String,
    
    // Enum that can evolve safely
    #[flat_message_item(
        repr = u8, 
        kind = enum, 
        validate = fallback
    )]
    user_type: UserType,
}
```

## Implementation Notes

- The `validate` attribute affects only the deserialization process
- Serialization is not affected by validation settings
- Fallback validation requires the type to implement `Default` trait
- For enums, the `#[default]` attribute must be specified on one variant
- The validation behavior is determined at compile time through proc macros
