# Ignoring Fields

FlatMessage provides a powerful mechanism to exclude specific fields from serialization and deserialization using the `ignore` attribute. This feature is particularly useful for fields that contain runtime-only data, computed values, or zero-sized types that don't need to be persisted. Ignored fields have no impact on the binary representation. The serialized data will not contain any information about ignored fields.

## Basic Usage

To ignore a field during serialization and deserialization, use the `#[flat_message_item(ignore = true)]` attribute:

```rust
use flat_message::*;

#[derive(FlatMessage)]
#[flat_message_options(store_name = false)]
struct User {
    id: u32,
    name: String,
    #[flat_message_item(ignore = true)]
    cached_score: u32,  // This field will be ignored
}
```

When serializing a `User` struct, the `cached_score` field will not be included in the binary data. During deserialization, this field will be set to its default value.

## Alternative Syntax

FlatMessage also supports the `skip` attribute as an alias for `ignore`:

```rust
#[derive(FlatMessage)]
#[flat_message_options(store_name = false)]
struct Data {
    value: u8,
    #[flat_message_item(skip = true)]
    temp_data: u32,  // Equivalent to ignore = true
}
```

Both `ignore = true` and `skip = true` have identical behavior.

## Default Values

### Using Type's Default

When a field is ignored, it will be initialized with the type's default value during deserialization:

```rust
#[derive(FlatMessage)]
#[flat_message_options(store_name = false)]
struct Example {
    x: u8,
    #[flat_message_item(ignore = true)]
    y: u32,  // Will be 0 (u32's default) after deserialization
}

let data = Example { x: 1, y: 999 };
let mut storage = Storage::default();
data.serialize_to(&mut storage, Config::default()).unwrap();

let deserialized = Example::deserialize_from(&storage).unwrap();
assert_eq!(deserialized.x, 1);
assert_eq!(deserialized.y, 0);  // Default value, not 999
```

### Custom Default Values

You can specify a custom default value for ignored fields using the `default` attribute:

```rust
#[derive(FlatMessage)]
#[flat_message_options(store_name = false)]
struct Config {
    version: u8,
    #[flat_message_item(ignore = true, default = 42)]
    cache_size: u32,  // Will be 42 after deserialization
}

let config = Config { version: 1, cache_size: 100 };
let mut storage = Storage::default();
config.serialize_to(&mut storage, Config::default()).unwrap();

let deserialized = Config::deserialize_from(&storage).unwrap();
assert_eq!(deserialized.version, 1);
assert_eq!(deserialized.cache_size, 42);  // Custom default value
```

## Zero-Sized Types (ZST)

FlatMessage automatically ignores zero-sized types like `PhantomData` without requiring explicit attributes:

```rust
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false)]
struct Container<T> {
    data: u8,
    _phantom: PhantomData<T>,  // Automatically ignored
}
```

Zero-sized types are treated as if they have `ignore = true` applied automatically.

## Use Cases

### Runtime-Only Data

Ignore fields that are only meaningful during runtime:

```rust
#[derive(FlatMessage)]
struct Session {
    user_id: u32,
    created_at: u64,
    #[flat_message_item(ignore = true)]
    last_access_time: u64,  // Updated frequently, no need to persist
    #[flat_message_item(ignore = true)]
    is_active: bool,        // Runtime state
}
```

### Computed Values

Ignore fields that can be computed from other data:

```rust
#[derive(FlatMessage)]
struct Rectangle {
    width: f32,
    height: f32,
    #[flat_message_item(ignore = true, default = 0.0)]
    area: f32,  // Can be computed as width * height
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            area: width * height,
        }
    }
    
    fn compute_area(&mut self) {
        self.area = self.width * self.height;
    }
}
```

### Temporary Buffers

Ignore fields used as temporary storage:

```rust
#[derive(FlatMessage)]
struct DataProcessor {
    input: Vec<u8>,
    output: Vec<u8>,
    #[flat_message_item(ignore = true)]
    temp_buffer: Vec<u8>,  // Working space, no need to serialize
}
```


## Performance Benefits

Ignoring unnecessary fields can improve performance by:

- Reducing serialized data size
- Decreasing serialization/deserialization time
- Minimizing memory usage for persistent storage
- Enabling better compression ratios

