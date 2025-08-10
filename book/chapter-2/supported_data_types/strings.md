# Strings

Strings types are represented as UTF-8 encoded bytes.

| Data Type                | Object | Slice | Vector | Option |
| ------------------------ | ------ | ----- | ------ | ------ |
| String referemce: `&str` | Yes    | -     | Yes    | Yes    |
| String object: `String`  | Yes    | -     | Yes    | Yes    |


**Remarks:**
- deserialization using `deserialize_from` will validate if the string is a correct UTF-8 buffer. If you are certain that format is valid, you can use `deserialize_from_unchecked` to skip the validation step. This will speed up the deserialization process, but it is your responsibility to ensure that the string is actually a valid UTF-8 format.
- You can use `String` and `&str` interchangeably, meaning that you can serialize an object that has a `String` field and deserialize it into a `&str`, or vice versa. This usually speeds up the deserialization process, as `&str` is a reference to a string slice and does not require allocation and copying of the data, while `String` is an owned collection of characters that requires allocation and copying of the data.

## Example

1. Direct values:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example<'a> {
        string_value: String,
        str_value: &'a str,
    }
    ```

2. Vectors of strings or string slices:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example<'a> {
        string_values: Vec<String>,
        str_values: Vec<&'a str>,
    }
    ```

3. Using `Option` values:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example<'a> {
        string_value: Option<String>,
        str_value: Option<&'a str>,
    }
    ```