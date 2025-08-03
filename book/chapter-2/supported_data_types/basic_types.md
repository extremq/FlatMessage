# Basic Types

| Data Type                                                             | Supported | Slice | Vector |
| --------------------------------------------------------------------- | --------- | ----- | ------ |
| Boolean values: `bool`                                                | Yes       | Yes   | Yes    |
| Integer value: `u8`, `u16`, `u32`, `u128`, `i8`, `i16`, `i32`, `i128` | Yes       | Yes   | Yes    |
| Float values: `f32`, `f64`                                            | Yes       | Yes   | Yes    |

**Remarks:**
- for `bool` values, deserialization using `deserialize_from` will validate if the value is `0` or `1`, and will return an error if the value is not valid. If you are certain that the value is valid, you can use `deserialize_from_unchecked` to skip the validation step. This will speed up the deserialization process, but it is your responsibility to ensure that the value is valid.

## Example

1. Direct values:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example {
        boolean_value: bool,
        integer_value: u32,
        float_value: f64,
    }
    ```

2. Slices of values:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example {
        boolean_values: &[bool],
        integer_values: &[u32],
        float_values: &[f64],
    }
    ```

3. Vectors of values:
    ```rust
    use flat_message::*;

    #[derive(FlatMessage)]
    struct Example {
        boolean_values: Vec<bool>,
        integer_values: Vec<u32>,
        float_values: Vec<f64>,
    }
    ```