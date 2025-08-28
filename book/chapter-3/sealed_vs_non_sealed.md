# Sealed vs Non-Sealed Types

FlatMessage supports the `#[sealed]` attribute for enums, flags, and variants, providing two different approaches to type evolution and compatibility. This attribute fundamentally changes how hash values are computed and affects compatibility behavior when types are modified.

## Understanding the Sealed Attribute

The `#[sealed]` attribute controls whether a type's hash includes all its internal structure:

- **Non-sealed (default)**: Hash includes only the type name
- **Sealed**: Hash includes type name + all internal structure (variants, flags, etc.)

This applies to three main derive macros:
- `FlatMessageEnum` with `#[sealed]`
- `FlatMessageFlags` with `#[sealed]`  
- `FlatMessageVariant` with `#[sealed]`

## Sealed vs Non-Sealed Enums

### Hash Calculation Difference

```rust
use flat_message::*;

// Non-sealed enum (default)
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Status {
    Active = 1,
    Inactive = 2,
}
// Hash: Hash("Status") - only name matters

// Sealed enum
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
#[sealed]
enum Protocol {
    Http = 1,
    Https = 2,
}
// Hash: Hash("Protocol" + "Http" + "Https") - includes all variants
```

### Compatibility Behavior

**Non-sealed enums** allow adding new variants:

```rust
// Version 1
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
}

// Version 2 - Compatible with Version 1 data
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
    Yellow = 4,    // New variant - still compatible
}
```

**Sealed enums** break compatibility when modified:

```rust
// Version 1
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
#[sealed]
enum SecurityLevel {
    Public = 1,
    Internal = 2,
    Confidential = 3,
}

// Version 2 - NOT compatible with Version 1
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
#[sealed]
enum SecurityLevel {
    Public = 1,
    Internal = 2,
    Confidential = 3,
    TopSecret = 4,  // Adding this changes the hash completely
}
```

## Sealed vs Non-Sealed Flags

Flags behave similarly to enums regarding the sealed attribute:

```rust
// Non-sealed flags (default)
#[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
#[repr(transparent)]
#[flags(Read, Write, Execute)]
pub struct Permissions(u8);
impl Permissions {
    add_flag!(Read = 1);
    add_flag!(Write = 2);
    add_flag!(Execute = 4);
}
// Hash: Hash("Permissions") - can add new flags later

// Sealed flags
#[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
#[repr(transparent)]
#[sealed]
#[flags(Admin, User, Guest)]
pub struct UserType(u8);
impl UserType {
    add_flag!(Admin = 1);
    add_flag!(User = 2);
    add_flag!(Guest = 4);
}
// Hash: Hash("UserType" + "Admin" + "User" + "Guest") - strict
```

### Flag Evolution Example

```rust
// Version 1: Non-sealed flags
#[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
#[repr(transparent)]
#[flags(Debug, Info, Warning, Error)]
pub struct LogFlags(u16);
impl LogFlags {
    add_flag!(Debug = 1);
    add_flag!(Info = 2);
    add_flag!(Warning = 4);
    add_flag!(Error = 8);
}

// Version 2: Add new flag - compatible
#[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
#[repr(transparent)]
#[flags(Debug, Info, Warning, Error, Fatal)]
pub struct LogFlags(u16);
impl LogFlags {
    add_flag!(Debug = 1);
    add_flag!(Info = 2);
    add_flag!(Warning = 4);
    add_flag!(Error = 8);
    add_flag!(Fatal = 16);  // New flag - works with old data
}
```

## Sealed vs Non-Sealed Variants

Variants (sum types) also support the sealed attribute:

```rust
// Non-sealed variant (default)
#[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
enum ApiResponse {
    Success(String),
    Error(u32),
}
// Hash: Hash("ApiResponse") - can add new variants

// Sealed variant
#[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
#[sealed]
enum DatabaseCommand {
    Insert(String),
    Update(String),
    Delete(u32),
}
// Hash: Hash("DatabaseCommand" + "Insert" + "Update" + "Delete") - strict
```

### Variant Evolution

```rust
// Version 1: Non-sealed variant
mod v1 {
    use flat_message::*;
    
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    pub enum NetworkEvent {
        Connect(String),
        Disconnect,
    }
}

// Version 2: Add new variant - compatible
mod v2 {
    use flat_message::*;
    
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    pub enum NetworkEvent {
        Connect(String),
        Disconnect,
        Timeout(u32),  // New variant - works with v1 data
    }
}
```

## When to Use Sealed Types

### Use Sealed Types When:

1. **Security-critical**: Cryptographic algorithms, security levels
2. **Stable protocols**: Fixed command sets that shouldn't change
3. **Data integrity**: Any modification should break compatibility
4. **Exact matching required**: Systems must have identical type definitions

```rust
// Good candidates for sealed
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u8)]
#[sealed]
enum CryptoAlgorithm {
    Aes256 = 1,
    ChaCha20 = 2,
    // These must not change - security critical
}

#[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
#[repr(transparent)]
#[sealed]
#[flags(Create, Read, Update, Delete)]
pub struct CrudOperations(u8);
// CRUD is stable and complete
```

### Use Non-Sealed Types When:

1. **Expected evolution**: Types will grow over time
2. **Forward compatibility**: Older code should handle newer types
3. **API evolution**: Public interfaces that might expand
4. **Configuration options**: Settings that may increase

```rust
// Good candidates for non-sealed
#[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
#[repr(u16)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
    // Many more status codes might be added
}

#[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
enum LogLevel {
    Debug,
    Info(String),
    Warning(String),
    Error(String),
    // Might add Critical, Trace, etc.
}
```

