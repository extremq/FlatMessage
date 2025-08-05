# Timestamp

| Data Type                                            | Object | Slice | Vector | Option |
| ---------------------------------------------------- | ------ | ----- | ------ | ------ |
| Timestamp (`Timestamp` or `flat_message::Timestamp`) | Yes    | -     | -      | -      |


**Remarks:**
- A `Timestamp` is a 64-bit value that represents time in milliseconds since the UNIX epoch (January 1, 1970, 00:00:00 UTC) and is intended to appear only once in a struct. The following example will not compile as it uses two fields with the type `Timestamp`:

    ```rust
    use flat_message::*;

    // code will not compile (two timestamps)
    #[derive(FlatMessage)]
    struct Example {
        created_at: Timestamp,
        updated_at: Timestamp
    }
    ```
- The use of `Timestamp` is optional—you don't need to include it in your structure unless you want to track timing information.
- Since `Timestamp` can be used only once per struct, its field name is not stored; it will be automatically mapped to any field with the same type.
- The timestamp value is stored as an unsigned 64-bit integer, providing a range that can represent dates far into the future.
- When system time retrieval fails, the timestamp defaults to 0 (representing the UNIX epoch).
- The `Timestamp` type implements common traits like `Copy`, `Clone`, `Debug`, `Eq`, `PartialEq`, `Ord`, and `PartialOrd`, making it suitable for comparisons and sorting operations.

## Methods

The following methods are available for a Timestamp:

| Method                               | Purpose                                                                                                                                                        |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `with_value(value: u64)`             | Creates a new Timestamp instance with a manually provided value in milliseconds since the UNIX epoch. Use this when you have a pre-existing timestamp value.   |
| `now()`                              | Creates a new Timestamp with the current system time in milliseconds since the UNIX epoch. Returns a timestamp with value 0 if system time cannot be obtained. |
| `from_system_time(time: SystemTime)` | Creates a new Timestamp from a `std::time::SystemTime` value. Returns a timestamp with value 0 if the conversion fails.                                        |
| `value(&self)`                       | Returns the underlying 64-bit integer value of the Timestamp in milliseconds since the UNIX epoch. Useful for storing or transmitting the timestamp value.     |

## Example

```rust
use flat_message::*;

#[derive(FlatMessage)]
struct LogEntry {
    message: String,
    level: u8,
    created_at: Timestamp
}

// Usage examples
let entry = LogEntry {
    message: "Application started".to_string(),
    level: 1,
    created_at: Timestamp::now()
};

// Create with specific timestamp
let historical_entry = LogEntry {
    message: "System boot".to_string(),
    level: 0,
    created_at: Timestamp::with_value(1640995200000) // Jan 1, 2022 00:00:00 UTC
};
```

