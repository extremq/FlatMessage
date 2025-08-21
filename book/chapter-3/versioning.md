# Versioning

FlatMessage provides explicit version control for your data structures through the `version` and `compatible_versions` attributes. This system allows you to evolve your data formats safely while maintaining precise control over compatibility. Understanding how versioning interacts with field requirements is crucial for designing robust, evolvable data structures.

## Basic Version Declaration

Every FlatMessage structure can declare a version number using the `version` attribute:

```rust
use flat_message::*;

#[derive(FlatMessage)]
#[flat_message_options(version = 1)]
struct UserProfile {
    name: String,
    email: String,
}
```

**Key Points:**
- Version numbers range from 1 to 255 (version 0 means no versioning)
- The version is stored in the 8-byte header of serialized data
- During deserialization, version compatibility is checked BEFORE field validation

## Version Compatibility Control

Use `compatible_versions` to specify which data versions your structure can deserialize:

```rust
#[derive(FlatMessage)]
#[flat_message_options(version = 2, compatible_versions = "1,2")]
struct UserProfileV2 {
    name: String,
    email: String,
    age: u32,  // New field - but is it compatible with v1 data?
}
```

This configuration means:
- The structure declares itself as version 2
- It accepts data from versions 1 and 2
- Attempting to deserialize version 3+ data will fail with `Error::IncompatibleVersion`

**Critical:** Version compatibility only controls which data versions are accepted - it does NOT automatically handle field differences.

## Deserialization Process Order

Understanding the deserialization process is crucial:

```rust
fn deserialize_example() {
    // 1. Header validation (magic number, size checks)
    // 2. Version compatibility check
    if header.version not in compatible_versions {
        return Err(Error::IncompatibleVersion(header.version));
    }
    // 3. Field validation and deserialization
    for each mandatory field {
        if field not found in data {
            return Err(Error::FieldIsMissing(field_hash));
        }
    }
    // 4. Struct construction
}
```

## Version Compatibility Syntax

The `compatible_versions` attribute supports flexible range expressions:

### Specific Versions
```rust
#[flat_message_options(version = 3, compatible_versions = "1,2,3")]
// Accepts exactly versions 1, 2, and 3
```

### Range Operators
```rust
#[flat_message_options(version = 5, compatible_versions = "<5")]
// Accepts versions 1, 2, 3, 4 (less than 5)

#[flat_message_options(version = 4, compatible_versions = "<=4")]
// Accepts versions 1, 2, 3, 4 (less than or equal to 4)
```

### Interval Ranges
```rust
#[flat_message_options(version = 10, compatible_versions = "5-10")]
// or "5:10" or "5..10"
// Accepts versions 5, 6, 7, 8, 9, 10
```

### Combined Expressions
```rust
#[flat_message_options(version = 8, compatible_versions = "1,3,5-8")]
// Accepts versions 1, 3, 5, 6, 7, 8
```

## Real-World Version Evolution Examples

### Scenario 1: Version Check Without Field Compatibility

```rust
mod v1 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 1, compatible_versions = "1")]
    struct Config {
        value: u8,
    }
}

mod v2 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 2, compatible_versions = "1,2")]
    struct Config {
        value: u8,
        value2: u16,  // New mandatory field
    }
}
```

**Results:**
- v1 data → v2 struct: ❌ Fails with `FieldIsMissing` (value2 not in v1 data)
- v2 data → v1 struct: ❌ Fails with `IncompatibleVersion(2)` (v1 only accepts version 1)
- v2 data → v2 struct: ✅ Works

**Key Insight:** Version compatibility and field compatibility are separate concerns!

### Scenario 2: Forward Compatible Versioning

```rust
mod v1 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 1)]  // No compatible_versions = accepts any version
    struct Config {
        value: u8,
    }
}

mod v2 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 2)]  // No compatible_versions = accepts any version
    struct Config {
        value: u8,
        value2: u16,  // New mandatory field
    }
}
```

**Results:**
- v1 data → v2 struct: ❌ Fails with `FieldIsMissing` (value2 not in v1 data)
- v2 data → v1 struct: ✅ Works (v1 ignores extra fields it doesn't need)

### Scenario 3: Safe Evolution with Optional Fields

```rust
mod v1 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 1)]
    struct Config {
        value: u8,
    }
}

mod v2 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 2)]
    struct Config {
        value: u8,
        #[flat_message_item(mandatory = false, default = "3")]
        value2: u16,  // Optional field with default
    }
}
```

**Results:**
- v1 data → v2 struct: ✅ Works (value2 = 3 default)
- v2 data → v1 struct: ✅ Works (v1 ignores extra fields)

## Common Version Compatibility Patterns

### Strict Version Matching
```rust
#[flat_message_options(version = 2, compatible_versions = "2")]
// Only accepts exactly version 2 data
```

**Use case:** When you need strict control and no backward compatibility.

### Backward Compatibility
```rust
#[flat_message_options(version = 3, compatible_versions = "1-3")]
// Version 3 struct can read data from versions 1, 2, and 3
```

**Use case:** When newer code needs to read older data formats. Requires careful field design with optional fields for additions.

### Forward Compatibility
```rust
#[flat_message_options(version = 1, compatible_versions = "1-5")]
// Version 1 struct can read data up to version 5
```

**Use case:** When older code needs to read newer data formats. Only works if newer versions only add optional fields.

### Version Windows
```rust
#[flat_message_options(version = 5, compatible_versions = "3-7")]
// Accepts versions 3, 4, 5, 6, 7 but not 1, 2, or 8+
```

**Use case:** When you want to support a sliding window of versions, dropping support for very old formats.

## Version Introspection

You can check the version of serialized data without full deserialization:

```rust
use flat_message::*;

fn check_version(storage: &Storage) -> Result<(), Error> {
    let info = StructureInformation::try_from(storage)?;
    
    match info.version() {
        Some(version) => {
            println!("Data version: {}", version);
            
            // Make decisions based on version
            if version < 2 {
                println!("Legacy format detected");
            } else if version > 5 {
                println!("Future version - may need migration");
            }
        }
        None => println!("No version information available"),
    }
    
    Ok(())
}
```

## Error Handling

Version-related errors provide specific information:

```rust
match Config::deserialize_from(&storage) {
    Err(Error::IncompatibleVersion(found_version)) => {
        eprintln!("Cannot read version {} data with this struct", found_version);
        // Could attempt migration or request data in supported format
    }
    Err(Error::FieldIsMissing(field_hash)) => {
        eprintln!("Missing required field (hash: 0x{:08X})", field_hash);
        // Field compatibility issue, not version issue
    }
    Err(Error::InvalidHeaderLength(_)) => {
        eprintln!("Corrupted or invalid data");
    }
    Ok(config) => {
        // Successfully deserialized
    }
}
```

## Advanced Version Handling

For complex migration scenarios, implement version-aware deserialization:

```rust
#[derive(FlatMessage)]
#[flat_message_options(version = 3, compatible_versions = "1-3")]
struct Config {
    host: String,
    port: u16,
    
    #[flat_message_item(mandatory = false, default = "30")]
    timeout: u32,    // Optional for v1/v2 compatibility
    
    #[flat_message_item(mandatory = false, default = "3")]
    retries: u8,     // Optional for v1/v2 compatibility
}

impl Config {
    pub fn from_any_version(storage: &Storage) -> Result<Self, Error> {
        // Check version before attempting deserialization
        let info = StructureInformation::try_from(storage)?;
        
        match info.version() {
            Some(1) => {
                println!("Migrating from v1 format");
                // v1 data should work with optional fields
                Self::deserialize_from(storage)
            }
            Some(2) => {
                println!("Reading v2 format");
                Self::deserialize_from(storage)
            }
            Some(3) => {
                println!("Reading current v3 format");
                Self::deserialize_from(storage)
            }
            Some(version) if version > 3 => {
                Err(Error::IncompatibleVersion(version))
            }
            None => {
                println!("No version info - assuming v1");
                Self::deserialize_from(storage)
            }
            _ => unreachable!(),
        }
    }
}
```

## Best Practices

1. **Version from the start**: Always include `version = 1` on new structures
2. **Plan compatibility strategy**: Decide upfront whether you need backward, forward, or bidirectional compatibility
3. **Use optional fields for additions**: New fields should be optional (`mandatory = false`) to maintain backward compatibility
4. **Test all compatibility scenarios**: Include tests for all supported version combinations
5. **Understand the two-phase validation**: Version check happens before field validation
6. **Document breaking changes**: Clearly mark when mandatory fields are added
7. **Use version introspection**: Check versions before deserialization in multi-version systems
8. **Plan deprecation cycles**: Allow time for systems to upgrade before removing compatibility

## Common Pitfalls

1. **Assuming version compatibility handles fields**: `compatible_versions` only controls version acceptance, not field compatibility
2. **Adding mandatory fields to backward-compatible versions**: This breaks compatibility even with version ranges
3. **Confusing Option<T> with optional fields**: `Option<T>` fields are still mandatory unless marked with `mandatory = false`
4. **Not testing field compatibility**: Version compatibility tests must also verify field-level compatibility
