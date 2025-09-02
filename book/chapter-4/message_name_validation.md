# Message Name Validation

FlatMessage provides a robust name validation system that allows you to control whether structure names are stored during serialization and validated during deserialization. This feature helps ensure type safety and prevents accidental deserialization of incompatible data structures.

## Overview

The name validation system uses two key attributes that work together:

- **`store_name`**: Controls whether the structure name hash is stored in the serialized data
- **`validate_name`**: Controls whether the structure name is validated during deserialization

## Attributes

### `store_name` Attribute

```rust
#[derive(FlatMessage)]
#[flat_message_options(store_name = true)]  // Default: true
struct MyStruct {
    value: u32,
}
```

When `store_name = true`:
- A 32-bit hash of the structure name is stored in the serialized data
- Adds 4 bytes to the serialized size
- Enables name validation during deserialization

When `store_name = false`:
- No name information is stored in the serialized data
- Saves 4 bytes of storage space
- Name validation is impossible during deserialization

### `validate_name` Attribute

```rust
#[derive(FlatMessage)]
#[flat_message_options(validate_name = true)] // Default: false
struct MyStruct {
    value: u32,
}
```

When `validate_name = true`:
- During deserialization, the stored name hash is compared with the target structure's name
- Throws `Error::UnmatchedName` if names don't match
- Throws `Error::NameNotStored` if no name was stored during serialization

When `validate_name = false` (default):
- No name validation is performed during deserialization
- Allows deserialization between different structure types (if fields are compatible)

## Attribute Combinations

### 1. Default Behavior: `store_name = true`, `validate_name = false`

```rust
#[derive(Debug, PartialEq, Eq, FlatMessage)]
struct StructA {
    value: u64,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
struct StructB {
    value: u64,
}
```

**Behavior:**
- Name is stored during serialization (4 extra bytes)
- No validation during deserialization
- Cross-structure deserialization is allowed if fields are compatible

```rust
let a = StructA { value: 42 };
let mut storage = Storage::default();
a.serialize_to(&mut storage, Config::default()).unwrap();

// This works - same structure
let restored_a = StructA::deserialize_from(&storage).unwrap();

// This also works - different structure but compatible fields
let restored_b = StructB::deserialize_from(&storage).unwrap();
```

### 2. Strict Validation: `store_name = true`, `validate_name = true`

```rust
#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = true, validate_name = true)]
struct StrictStruct {
    value: u64,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(validate_name = true)]
struct OtherStruct {
    value: u64,
}
```

**Behavior:**
- Name is stored during serialization
- Name is validated during deserialization
- Cross-structure deserialization fails with `Error::UnmatchedName`

```rust
let strict = StrictStruct { value: 42 };
let mut storage = Storage::default();
strict.serialize_to(&mut storage, Config::default()).unwrap();

// This works - correct structure
let restored = StrictStruct::deserialize_from(&storage).unwrap();

// This fails - different structure name
match OtherStruct::deserialize_from(&storage) {
    Err(Error::UnmatchedName) => {
        println!("Name validation prevented cross-type deserialization");
    }
    _ => panic!("Expected name validation error"),
}
```

### 3. Space Optimized: `store_name = false`, `validate_name = false`

```rust
#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false, validate_name = false)]
struct CompactStruct {
    value: u64,
}
```

**Behavior:**
- No name is stored (saves 4 bytes)
- No validation is performed
- Maximum interoperability between compatible structures

```rust
let compact = CompactStruct { value: 42 };
let mut storage = Storage::default();
compact.serialize_to(&mut storage, Config::default()).unwrap();

// Works with any compatible structure
let restored = AnyCompatibleStruct::deserialize_from(&storage).unwrap();
```

### 4. Invalid Combination: `store_name = false`, `validate_name = true`

```rust
#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false, validate_name = true)]
struct InvalidConfig {
    value: u64,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false)]
struct DataSource {
    value: u64,
}
```

**Behavior:**
- No name is stored during serialization
- Validation is attempted during deserialization
- Always fails with `Error::NameNotStored`

```rust
let source = DataSource { value: 42 };
let mut storage = Storage::default();
source.serialize_to(&mut storage, Config::default()).unwrap();

// This always fails because no name was stored
match InvalidConfig::deserialize_from(&storage) {
    Err(Error::NameNotStored) => {
        println!("Cannot validate name when none was stored");
    }
    _ => panic!("Expected NameNotStored error"),
}
```

## Error Types

### `Error::UnmatchedName`

Occurs when:
- `validate_name = true` on the target structure
- A name hash was stored in the serialized data
- The stored name hash doesn't match the target structure's name hash

### `Error::NameNotStored`

Occurs when:
- `validate_name = true` on the target structure
- No name hash was stored in the serialized data (source had `store_name = false`)

## Use Cases

### Type Safety in APIs

Use strict validation when deserializing data from external sources:

```rust
#[derive(FlatMessage)]
#[flat_message_options(store_name = true, validate_name = true)]
struct ApiRequest {
    user_id: u64,
    action: String,
}

fn handle_request(data: &[u8]) -> Result<(), Error> {
    // This will fail if the data wasn't serialized as ApiRequest
    let request = ApiRequest::deserialize_from(data)?;
    // Process request safely...
    Ok(())
}
```

## Best Practices

1. **Use strict validation** (`store_name = true, validate_name = true`) for:
   - Network protocol messages
   - API endpoints
   - Critical data structures where type safety is paramount

2. **Use flexible validation** (`store_name = true, validate_name = false`) for:
   - Data persistence with potential schema evolution
   - Internal application data structures
   - When you need some metadata but want flexibility

3. **Disable name storage** (`store_name = false`) for:
   - High-frequency data (sensor readings, telemetry)
   - Memory-constrained environments
   - When every byte counts and type safety is managed elsewhere

4. **Avoid the invalid combination** (`store_name = false, validate_name = true`):
   - This configuration will always fail during deserialization
   - The compiler doesn't prevent this, so be careful with your configuration

## Runtime Introspection

You can inspect stored names using `StructureInformation`:

```rust
use flat_message::*;

#[derive(FlatMessage)]
#[flat_message_options(store_name = true)]
struct MyStruct {
    value: u32,
}

fn inspect_name(storage: &Storage) -> Result<(), Error> {
    let info = StructureInformation::try_from(storage)?;
    
    if let Some(name) = info.name() {
        match name {
            name!("MyStruct") => println!("Found MyStruct data"),
            name!("OtherStruct") => println!("Found OtherStruct data"),
            _ => println!("Found unknown structure: {:?}", name),
        }
    } else {
        println!("No name information stored");
    }
    
    Ok(())
}
```

The name validation system provides a flexible balance between type safety, storage efficiency, and interoperability, allowing you to choose the right trade-offs for your specific use case.