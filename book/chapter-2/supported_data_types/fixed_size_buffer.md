# Fixed Size Buffer

Fixed-size byte arrays with compile-time known length are supported for serialization and deserialization.

| Data Type                                      | Object | Slice | Vector | Option |
| ---------------------------------------------- | ------ | ----- | ------ | ------ |
| Fixed-size byte array: `[u8; N]`               | Yes    | Yes   | Yes    | Yes    |
| Reference to fixed-size byte array: `&[u8; N]` | Yes    | -     | -      | Yes    |

**Remarks:**
- Fixed-size byte arrays (`[u8; N]`) have their size known at compile time, where `N` is a constant value.
- References to fixed-size arrays (`&[u8; N]`) can be used for zero-copy deserialization, avoiding memory allocation.
- Deserialization validates that the stored size matches the expected compile-time size `N`. If the sizes don't match, deserialization will fail.
- You can use `deserialize_from_unchecked` to skip size validation if you are certain the data is valid, which improves performance.


## Example

1. Direct fixed-size arrays:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example {
        buffer_10: [u8; 10],
        buffer_4: [u8; 4],
        small_buffer: [u8; 2],
    }
    ```

2. References to fixed-size arrays:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example<'a> {
        buffer_ref: &'a [u8; 10],
    }
    ```

3. Slices of fixed-size arrays:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example<'a> {
        multiple_buffers: &'a [[u8; 3]],
    }
    ```

4. Vectors of fixed-size arrays:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example {
        buffer_collection: Vec<[u8; 8]>,
    }
    ```

5. Optional fixed-size arrays:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example<'a> {
        optional_buffer: Option<[u8; 16]>,
        optional_buffer_ref: Option<&'a [u8; 12]>,
        optional_buffer_vec: Option<Vec<[u8; 4]>>,
    }
    ```

## Performance Considerations

- Fixed-size arrays are highly efficient as their size is known at compile time
- No dynamic memory allocation is needed for the arrays themselves
- Use references (`&[u8; N]`) when possible for zero-copy deserialization
- For collections of fixed arrays, the memory layout is contiguous can easily be readu using a slice (`&[[u8; N]]`) - meaning zero-copy deserialization.
- These types of buffer are often used for hash values (such as `sha256` or `sha512`) 