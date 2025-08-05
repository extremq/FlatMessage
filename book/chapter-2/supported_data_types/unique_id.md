# Unique ID

| Data Type                                                 | Object | Slice | Vector | Option |
| --------------------------------------------------------- | ------ | ----- | ------ | ------ |
| Unique identfier (`UniqueID` or `flat_message::UniqueID`) | Yes    | -     | -      | -      |


**Remarks:**
- A `UniqueID` is a 64-bit value that must be non-zero and is intended to appear only once in a struct. It serves as a unique identifier for a message. The following example will not compile as it uses two fields with the type `UniqueID`:

    ```rust
    use flat_message::*;

    // code will not compile (two ids)
    #[derive(FlatMessage)]
    struct Example {
        id1: UniqueID
        id2: UniqueID
    }
    ```
- The use of `UniqueID` is optional—you don't need to include it in your structure unless you want to store messages in a database or require a unique identifier.
- Since `UniqueID` can be used only once per struct, its field name is not stored; it will be automatically mapped to any field with the same type.

## Methods

The following methods are available for an UniqueID:

| Method                   | Purpose                                                                                                                                                             |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `new()`                  | Creates a new UniqueID instance with a globally unique, non-zero 64-bit value. It uses an atomic counter (GLOBAL_ID) to ensure each call produces a distinct value. |
| `with_value(value: u64)` | Creates a UniqueID from a manually provided 64-bit value. This method bypasses automatic generation and should be used only when you already have a valid ID.       |
| `value(&self)`           | Returns the underlying 64-bit integer value of the UniqueID. Useful for reading or storing the ID in external systems (e.g., databases).                            |

## Example

```rust
use flat_message::*;

#[derive(FlatMessage)]
struct Example {
    name: String,
    grade: u32,
    id: UniqueID
}
```
