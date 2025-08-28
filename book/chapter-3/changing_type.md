# Changing Type

When evolving your data structures over time, you may need to change the type of a field. FlatMessage provides specific behavior and compatibility rules when field types change between versions. Understanding these rules is crucial for maintaining backward and forward compatibility.

## Overview

FlatMessage identifies fields using a combination of:
- Field name
- Type information (including representation for complex types)

When you change a field's type, FlatMessage's behavior depends on whether the field can still be identified and matched between versions.

## Field Identification Rules

### Primitive Types
For primitive types (`u8`, `u16`, `String`, `Option<T>`, etc.), the type is part of the field's identity. Changing a primitive type means the field will not be found during deserialization.

**Example:**
```rust
// Version 1
struct Message {
    value: u8,  // Field identified as "value:u8"
}

// Version 2
struct Message {
    value: u16, // Field identified as "value:u16" - different from v1
}
```

### Complex Types (Enums, Flags, Variants)
For complex types with `repr` attributes, FlatMessage uses the representation type for field identification:

**Same Representation (Field Found):**
```rust
// Version 1
#[derive(FlatMessageEnum)]
#[repr(u8)]
enum Color { Red, Green, Blue }

struct Message {
    #[flat_message_item(repr = u8, kind = enum)]
    color: Color,  // Field identified as "color:u8"
}

// Version 2
#[derive(FlatMessageEnum)]
#[repr(u8)]
enum Shade { Light, Dark }

struct Message {
    #[flat_message_item(repr = u8, kind = enum)]
    color: Shade,  // Field identified as "color:u8" - SAME as v1
}
```

**Different Representation (Field Not Found):**
```rust
// Version 1
#[derive(FlatMessageEnum)]
#[repr(u8)]
enum Color { Red, Green, Blue }

struct Message {
    #[flat_message_item(repr = u8, kind = enum)]
    color: Color,  // Field identified as "color:u8"
}

// Version 2
#[derive(FlatMessageEnum)]
#[repr(u16)]
enum Shade { Light, Dark }

struct Message {
    #[flat_message_item(repr = u16, kind = enum)]
    color: Shade,  // Field identified as "color:u16" - DIFFERENT from v1
}
```

## Compatibility Behavior

The behavior when deserializing depends on two field attributes and whether the field is found:

### When Field is Found (Same Name + Compatible Type)

| `mandatory` | `validate` | Result | Behavior |
|-------------|------------|--------|----------|
| `true/false` | `strict` | **FAIL** | Type validation fails, deserialization error |
| `true/false` | `fallback` | **SUCCESS** | Type validation fails, uses default value |

**Key Insight:** When a field is found but types don't match exactly, the `mandatory` attribute is irrelevant because the field exists. Only `validate` determines the outcome.

### When Field is Not Found (Different Name or Incompatible Type)

| `mandatory` | `validate` | Result | Behavior |
|-------------|------------|--------|----------|
| `true` | `strict/fallback` | **FAIL** | Required field missing, deserialization error |
| `false` | `strict/fallback` | **SUCCESS** | Optional field missing, uses default value |

**Key Insight:** When a field is not found, the `validate` attribute is irrelevant because no validation occurs. Only `mandatory` determines the outcome.

## Type Change Scenarios

### Primitive Type Changes
All primitive type changes result in field not being found:

```rust
// v1: value: u8 → v2: value: u16
// v1: text: String → v2: text: u32  
// v1: data: Option<u8> → v2: data: Option<u16>
```

**Outcome:** Field not found → `mandatory` attribute determines success/failure

### Complex Type Changes - Same Representation

```rust
// Enums with same repr
#[repr(u8)] enum Color → #[repr(u8)] enum Shade

// Flags with same repr  
#[repr(u8)] struct Permissions → #[repr(u8)] struct Rights

// Variants with same repr
#[repr(u8)] enum Status → #[repr(u8)] enum Mode
```

**Outcome:** Field found → `validate` attribute determines success/failure

### Complex Type Changes - Different Representation

```rust
// Enums with different repr
#[repr(u8)] enum Color → #[repr(u16)] enum Shade

// Flags with different repr
#[repr(u8)] struct Permissions → #[repr(u16)] struct Rights

// Variants with different repr
#[repr(u8)] enum Status → #[repr(u16)] enum Mode
```

**Outcome:** Field not found → `mandatory` attribute determines success/failure

## Default Values

When deserialization succeeds but uses fallback behavior, FlatMessage uses the default value for the target type:

- **Primitive types:** Language defaults (`0` for integers, `""` for strings, `None` for options)
- **Enums:** The variant marked with `#[default]`
- **Flags:** Empty flags (no flags set)
- **Variants:** The variant marked with `#[default]`

## Best Practices

### For Backward Compatibility
1. **Avoid changing primitive types** - they always break compatibility
2. **Keep representation consistent** for complex types when possible
3. **Use `mandatory = false`** for fields that might be removed or changed
4. **Use `validate = fallback`** to gracefully handle type mismatches

### For Forward Compatibility
1. **Plan representation carefully** - changing `repr` breaks compatibility
2. **Consider using `Option<T>`** to make fields truly optional
3. **Document default values** clearly for fallback scenarios

### Migration Strategies
1. **Gradual migration:** Introduce new field, deprecate old field
2. **Representation preservation:** Keep same `repr` when changing complex types
3. **Fallback-friendly defaults:** Ensure default values are meaningful for your application

## Error Types

When type changes cause deserialization failures:

- **`Error::FieldIsMissing`**: Field not found (different types or names)
- **`Error::FailToDeserialize`**: Field found but type validation failed with `strict` validation
