# Mandatory Fields and Default Values

FlatMessage fields are mandatory by default, meaning they must be present in the serialized data during deserialization. However, you can mark fields as optional using the `mandatory = false` attribute and provide default values. Understanding this system is crucial for designing evolvable data structures.

## Understanding Mandatory vs Optional Fields

### Mandatory Fields (Default Behavior)

By default, all fields in a FlatMessage structure are mandatory:

```rust
use flat_message::*;

#[derive(FlatMessage)]
struct UserProfile {
    name: String,      // Mandatory
    email: String,     // Mandatory
    age: u32,          // Mandatory
}
```

**What happens during deserialization:**
- FlatMessage searches for each mandatory field's hash in the serialized data
- If any mandatory field is missing, deserialization fails with `Error::FieldIsMissing(hash)`
- This happens regardless of version compatibility settings

### Optional Fields

Use `mandatory = false` to make fields optional:

```rust
#[derive(FlatMessage)]
struct UserProfile {
    name: String,      // Mandatory
    email: String,     // Mandatory
    
    #[flat_message_item(mandatory = false)]
    age: u32,          // Optional - defaults to 0 if missing
    
    #[flat_message_item(mandatory = false)]
    bio: String,       // Optional - defaults to "" if missing
}
```

**What happens during deserialization:**
- FlatMessage searches for the optional field's hash in the serialized data
- If found, the field is deserialized normally
- If not found, the field uses its default value (`Type::default()` or custom default)
- No error is thrown for missing optional fields

## Default Value Behavior

### Type Defaults

When `mandatory = false` is specified without a custom default, the field uses the type's `Default::default()` implementation:

| Type                      | Default Value   | Notes                               |
| ------------------------- | --------------- | ----------------------------------- |
| `u8`, `u16`, `u32`, `u64` | `0`             | Numeric types default to zero       |
| `i8`, `i16`, `i32`, `i64` | `0`             | Signed types also default to zero   |
| `f32`, `f64`              | `0.0`           | Floating point defaults to zero     |
| `bool`                    | `false`         | Boolean defaults to false           |
| `String`                  | `String::new()` | Empty (non allocated) String object |
| `&str`                    | `""`            | Empty string (lifetime permitting)  |
| `Vec<T>`                  | `[]`            | Empty vector                        |
| `Option<T>`               | `None`          | Option defaults to None             |

### Custom Default Values

You can specify custom default values using the `default` attribute:

```rust
#[derive(FlatMessage)]
struct ServerConfig {
    host: String,      // Mandatory
    
    #[flat_message_item(mandatory = false, default = "8080")]
    port: u16,         // Optional with custom default
    
    #[flat_message_item(mandatory = false, default = "30")]
    timeout: u32,      // Optional with custom default
    
    #[flat_message_item(mandatory = false, default = "\"production\"")]
    environment: String, // Optional with custom default (note quotes)
}
```

**String Default Syntax:**
- For string literals, use double quotes: `default = "\"production\""`
- The system expects a valid Rust expression that evaluates to the field's type

### Advanced Default Values

You can use constants, expressions, or function calls:

```rust
const DEFAULT_TIMEOUT: u32 = 60;
const DEFAULT_RETRIES: u8 = 3;

#[derive(FlatMessage)]
struct ApiConfig {
    endpoint: String,  // Mandatory
    
    #[flat_message_item(mandatory = false, default = "DEFAULT_TIMEOUT")]
    timeout: u32,      // Uses constant
    
    #[flat_message_item(mandatory = false, default = "DEFAULT_RETRIES")]
    retries: u8,       // Uses constant
    
    #[flat_message_item(mandatory = false, default = "vec![8080, 8081, 8082]")]
    allowed_ports: Vec<u16>, // Uses expression
}
```

## Important: Option<T> Fields Are Optional by Default

**Key Change:** `Option<T>` fields are automatically treated as optional (`mandatory = false`) by default:

```rust
#[derive(FlatMessage)]
struct Config {
    host: String,           // Mandatory
    port: Option<u16>,      // Automatically optional! Uses None if missing
    
    #[flat_message_item(mandatory = true)]
    timeout: Option<u32>,   // Explicitly mandatory - must be present in data
}
```

**Behavior:**
- `Option<T>` fields without explicit attributes: **Optional** (use `None` if missing)
- `Option<T>` fields with `mandatory = true`: **Mandatory** (cause `Error::FieldIsMissing` if not present)
- `Option<T>` fields with `mandatory = false`: **Optional** (use `None` if missing - same as default)

This makes `Option<T>` fields naturally compatible for version evolution since they default to being optional.

## Relationship with Versioning

Mandatory fields interact with versioning in specific ways:

### Version Compatibility is Checked First

```rust
#[derive(FlatMessage)]
#[flat_message_options(version = 2, compatible_versions = "1,2")]
struct Config {
    host: String,      // Mandatory
    port: u16,         // Mandatory
    timeout: u32,      // Mandatory - added in v2
}
```

**Deserialization process:**
1. **Version check**: Is the data version in `compatible_versions`?
   - If not → `Error::IncompatibleVersion(version)`
2. **Field validation**: Are all mandatory fields present?
   - If not → `Error::FieldIsMissing(hash)`

### Adding Mandatory Fields Breaks Compatibility

```rust
// Version 1 data serialized with this structure
#[derive(FlatMessage)]
#[flat_message_options(version = 1)]
struct Config {
    host: String,
    port: u16,
}

// Version 2 structure tries to read v1 data
#[derive(FlatMessage)]
#[flat_message_options(version = 2, compatible_versions = "1,2")]
struct Config {
    host: String,
    port: u16,
    timeout: u32,      // New mandatory field
}
```

**Result**: Even though version compatibility allows reading v1 data, deserialization will fail with `Error::FieldIsMissing` because `timeout` is mandatory but not present in v1 data.

### Adding Optional Fields Maintains Compatibility

```rust
// Version 2 structure can successfully read v1 data
#[derive(FlatMessage)]
#[flat_message_options(version = 2, compatible_versions = "1,2")]
struct Config {
    host: String,
    port: u16,
    
    #[flat_message_item(mandatory = false, default = "30")]
    timeout: u32,      // Optional field with default
}
```

**Result**: v1 data → v2 struct works (timeout = 30), v2 data → v1 struct works (ignores timeout)

## Real-World Compatibility Scenarios

Based on the test scenarios, here are the actual compatibility behaviors:

### Scenario 1: Adding Mandatory Fields
```rust
mod v1 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 1)]
    struct Config { value: u8 }
}

mod v2 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 2)]
    struct Config { 
        value: u8,
        value2: u16,  // New mandatory field
    }
}
```
- ✅ v2 data → v1 struct: Works (v1 ignores extra field)
- ❌ v1 data → v2 struct: Fails with `FieldIsMissing` (value2 not in v1 data)

### Scenario 2: Adding Optional Fields
```rust
mod v2 {
    #[derive(FlatMessage)]
    #[flat_message_options(version = 2)]
    struct Config { 
        value: u8,
        #[flat_message_item(mandatory = false, default = "3")]
        value2: u16,  // Optional field
    }
}
```
- ✅ v2 data → v1 struct: Works (v1 ignores extra field)
- ✅ v1 data → v2 struct: Works (value2 = 3 default)

### Scenario 3: Option<T> Fields (Automatically Optional)
```rust
mod v2 {
    #[derive(FlatMessage)]
    struct Config { 
        value: u8,
        value2: Option<u16>,  // Automatically optional (new default behavior)
    }
}
```
- ✅ v2 data → v1 struct: Works (v1 ignores extra field)
- ✅ v1 data → v2 struct: Works (`value2 = None` default)

### Scenario 4: Option<T> with Explicit mandatory = true
```rust
mod v2 {
    #[derive(FlatMessage)]
    struct Config { 
        value: u8,
        #[flat_message_item(mandatory = true)]
        value2: Option<u16>,  // Explicitly mandatory Option
    }
}
```
- ✅ v2 data → v1 struct: Works (v1 ignores extra field)
- ❌ v1 data → v2 struct: Fails with `FieldIsMissing` (explicitly mandatory Option<T>)

## Best Practices

1. **Default to mandatory**: Make fields mandatory unless you specifically need them optional
2. **Plan for evolution**: New fields should be optional to maintain backward compatibility
3. **Leverage Option<T> for new fields**: `Option<T>` fields are automatically optional, making them ideal for version evolution
4. **Use meaningful defaults**: Provide sensible default values that won't break application logic
5. **Test compatibility**: Always test deserialization across supported version combinations
6. **Version carefully**: Consider mandatory field additions as breaking changes


Understanding mandatory fields and the automatic optional behavior of `Option<T>` is essential for designing robust, evolvable FlatMessage structures that can grow over time while maintaining compatibility with existing data.

