# Nested Structures and Version Compatibility

When working with nested structures in FlatMessage, understanding how different struct types behave during version changes is crucial for maintaining compatibility between different versions of your application. This chapter explains the compatibility implications of using `FlatMessageStruct` and `FlatMessagePacked` types as nested structures.

## Overview

FlatMessage supports two types of nested structures, each with different characteristics and compatibility behaviors:

1. **`FlatMessageStruct`** - Hash-based structures with flexible compatibility
2. **`FlatMessagePacked`** - Hash-validated structures with strict compatibility

The choice between these types significantly impacts how your data structures can evolve over time.

## FlatMessageStruct - Hash-Based Structures

`FlatMessageStruct` uses a hash table approach for field storage and lookup, making it more flexible for version evolution.

### Key Characteristics

- **Field Lookup**: Uses hash tables for field identification and access
- **Memory Layout**: Not sequential - fields can be accessed in any order
- **Metadata Support**: Supports `Timestamp` and `UniqueID` metadata fields (though they are ignored during serialization in nested contexts)
- **Option Support**: Can be wrapped in `Option<T>`
- **Validation**: Supports field-level validation attributes
- **Compatibility**: Can be compatible with different versions of the same struct (pending a proper usage of `mandatory` and `validate` attributes)

### Basic Usage

```rust
use flat_message::*;

#[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
struct UserProfile {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
struct User {
    pub id: u8,
    #[flat_message_item(kind = struct, align = 4)]
    pub profile: UserProfile,
}
```

## FlatMessagePacked - Hash-Validated Structures

`FlatMessagePacked` uses a sequential memory layout with hash validation for structure integrity.

### Key Characteristics

- **Memory Layout**: Sequential memory layout for optimal performance
- **Hash Validation**: Uses structure hash for validation during deserialization
- **No Metadata**: No support for `Timestamp` or `UniqueID` metadata fields
- **No Options**: Cannot be wrapped in `Option<T>`
- **Field Ordering**: Fields are automatically reordered by alignment for optimal packing
- **Compatibility**: Strict - any structural change breaks compatibility

### Basic Usage

```rust
use flat_message::*;

#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
struct Coordinates {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
struct Location {
    pub id: u8,
    #[flat_message_item(kind = packed, align = 4)]
    pub coords: Coordinates,
}
```

## Version Compatibility Scenarios

The following scenarios demonstrate how different structural changes affect compatibility between versions. These are based on comprehensive test cases that verify the actual behavior.

### Scenario 1: FlatMessageStruct - Adding Mandatory Fields with Strict Validation

**Version 1:**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
pub struct NestedStruct {
    pub value: u32,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
pub struct Test {
    pub id: u8,
    #[flat_message_item(kind = struct, align = 4)]
    pub nested: NestedStruct,
}
```

**Version 2:**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
pub struct NestedStruct {
    pub value: u32,
    pub new_field: u16, // New mandatory field added
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
pub struct Test {
    pub id: u8,
    #[flat_message_item(kind = struct, align = 4, validate = strict)]
    pub nested: NestedStruct,
}
```

**Compatibility:**
- **V1 → V2**: ❌ **FAILS** - Missing mandatory field causes deserialization failure
- **V2 → V1**: ✅ **SUCCEEDS** - Extra fields are ignored

### Scenario 2: FlatMessageStruct - Adding Optional Fields

**Version 2 (Modified):**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
pub struct NestedStruct {
    pub value: u32,
    #[flat_message_item(mandatory = false, validate = fallback, default = 42)]
    pub new_field: u16, // New optional field with default
}
```

**Compatibility:**
- **V1 → V2**: ✅ **SUCCEEDS** - Missing optional field uses default value (42)
- **V2 → V1**: ✅ **SUCCEEDS** - Extra fields are ignored

### Scenario 3: FlatMessageStruct - Fallback Validation on Parent Field

**Version 2 (Modified):**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessage)]
pub struct Test {
    pub id: u8,
    #[flat_message_item(kind = struct, align = 4, validate = fallback)]
    pub nested: NestedStruct, // validate = fallback on the field itself
}

impl Default for NestedStruct {
    fn default() -> Self {
        Self { value: 0, new_field: 0 }
    }
}
```

**Compatibility:**
- **V1 → V2**: ✅ **SUCCEEDS** - When struct deserialization fails, uses `Default::default()`
- **V2 → V1**: ✅ **SUCCEEDS** - Extra fields are ignored

### Scenario 4: FlatMessageStruct - Fallback Validation on Child Fields

**Version 2 (Modified):**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
pub struct NestedStruct {
    #[flat_message_item(validate = fallback)]
    pub value: u32,
    #[flat_message_item(validate = fallback)]
    pub new_field: u16, // validate = fallback on individual fields
}
```

**Compatibility:**
- **V1 → V2**: ❌ **FAILS** - Field-level fallback doesn't help with missing mandatory fields
- **V2 → V1**: ✅ **SUCCEEDS** - Extra fields are ignored

**Key Insight**: `validate = fallback` on individual struct fields only helps with field-level validation issues, not with missing mandatory fields at the struct level.

### Scenario 5: FlatMessageStruct - Mandatory = False on Parent Field

**Version 2 (Modified):**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessage)]
pub struct Test {
    pub id: u8,
    #[flat_message_item(kind = struct, align = 4, mandatory = false)]
    pub nested: NestedStruct, // mandatory = false on the field
}
```

**Compatibility:**
- **V1 → V2**: ❌ **FAILS** - `mandatory = false` doesn't help if the struct content has structural changes
- **V2 → V1**: ✅ **SUCCEEDS** - Extra fields are ignored

**Key Insight**: Setting `mandatory = false` on the parent field doesn't help when the nested struct itself has incompatible changes.

### Scenario 6: FlatMessageStruct - Removing Fields

**Version 1:**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
pub struct NestedStruct {
    pub value: u32,
    pub old_field: u16,
}
```

**Version 2:**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessageStruct)]
pub struct NestedStruct {
    pub value: u32, // old_field removed
}
```

**Compatibility:**
- **V1 → V2**: ✅ **SUCCEEDS** - Extra fields in data are ignored
- **V2 → V1**: ❌ **FAILS** - Missing mandatory field causes deserialization failure

### Scenario 7: FlatMessagePacked - Adding Fields

**Version 1:**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
pub struct NestedStruct {
    pub value: u32,
}
```

**Version 2:**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
pub struct NestedStruct {
    pub value: u32,
    pub new_field: u16, // Any change breaks compatibility
}
```

**Compatibility:**
- **V1 → V2**: ❌ **FAILS** - Hash validation detects structural change
- **V2 → V1**: ❌ **FAILS** - Hash validation detects structural change

**Key Insight**: ANY structural change to a packed struct breaks compatibility due to hash validation.

### Scenario 8: FlatMessagePacked - Fallback Validation on Parent Field

**Version 2 (Modified):**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessage)]
pub struct Test {
    pub id: u8,
    #[flat_message_item(kind = packed, align = 1, validate = fallback)]
    pub nested: NestedStruct,
}

impl Default for NestedStruct {
    fn default() -> Self {
        Self { value: 999, new_field: 888 }
    }
}
```

**Compatibility:**
- **V1 → V2**: ✅ **SUCCEEDS** - When packed struct validation fails, uses `Default::default()`
- **V2 → V1**: ❌ **FAILS** - Hash validation still detects structural change

### Scenario 9: FlatMessagePacked - Mandatory = False on Parent Field

**Version 2 (Modified):**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessage)]
pub struct Test {
    pub id: u8,
    #[flat_message_item(kind = packed, align = 1, mandatory = false)]
    pub nested: NestedStruct,
}
```

**Compatibility:**
- **V1 → V2**: ❌ **FAILS** - Hash validation fails before `mandatory = false` can take effect
- **V2 → V1**: ❌ **FAILS** - Hash validation detects structural change

### Scenario 10: FlatMessagePacked - Type Changes

**Version 1:**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
pub struct NestedStruct {
    pub value: u32,
    pub data: u8,
}
```

**Version 2:**
```rust
#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
pub struct NestedStruct {
    pub value: u32,
    pub data: u16, // Type changed from u8 to u16
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
pub struct Test {
    pub id: u8,
    #[flat_message_item(kind = packed, align = 1, validate = fallback)]
    pub nested: NestedStruct,
}
```

**Compatibility:**
- **V1 → V2**: ✅ **SUCCEEDS** - Hash validation fails, falls back to default
- **V2 → V1**: ❌ **FAILS** - Hash validation detects structural change

## Summary and Best Practices

### FlatMessageStruct Compatibility Rules

✅ **Compatible Changes:**
- Adding optional fields (`mandatory = false`)
- Removing fields (forward compatibility only)
- Changing field order (fields are hash-based)

❌ **Incompatible Changes:**
- Adding mandatory fields without fallback strategies
- Removing fields that newer versions still expect

### FlatMessagePacked Compatibility Rules

✅ **Compatible Changes:**
- None - packed structs are designed for performance, not evolution

❌ **Incompatible Changes:**
- Adding any fields
- Removing any fields  
- Changing field types
- Changing field order (though order is automatically optimized)

### Mitigation Strategies

#### For FlatMessageStruct

1. **Use Optional Fields**: Mark new fields as `mandatory = false`
   ```rust
   #[flat_message_item(mandatory = false, default = 42)]
   pub new_field: u16,
   ```

2. **Use Fallback Validation on Parent**: Apply `validate = fallback` to the struct field
   ```rust
   #[flat_message_item(kind = struct, align = 4, validate = fallback)]
   pub nested: NestedStruct,
   ```

3. **Implement Default Carefully**: Ensure `Default` implementations make sense for your domain
   ```rust
   impl Default for NestedStruct {
       fn default() -> Self {
           Self { value: 0, new_field: 100 }
       }
   }
   ```

#### For FlatMessagePacked

1. **Use Fallback Validation**: Apply `validate = fallback` to the packed struct field
   ```rust
   #[flat_message_item(kind = packed, align = 4, validate = fallback)]
   pub packed_data: PackedStruct,
   ```

2. **Design for Immutability**: Treat packed structs as immutable once deployed

3. **Version at Container Level**: Use versioning on the parent `FlatMessage` struct instead

### When to Use Each Type

**Choose FlatMessageStruct when:**
- You need version evolution capabilities
- Field access patterns are random or sparse
- You have optional/metadata fields
- Schema flexibility is important

**Choose FlatMessagePacked when:**
- Maximum performance is critical
- Memory layout optimization is important  
- The structure is stable and won't change
- You need the smallest possible serialized size


