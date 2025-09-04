# Default Values

There are several scenarios where a default value will be used to initialize a field during deserialization:
1. The field is not mandatory and is not present in the serialized data.
2. The field is present in the serialized data, but it has some issues trying to deserialize it and the `validate` attribute is set to `fallback`.
3. The field is skipped during serialization and it has to be defaulted during deserialization.

In these scenarios, FlatMessage will do one of the following:
1. Use the default value for the type if it is available (this implies that the type implements the `Default` trait).
2. If the attribute `default` is specified, it will use the value of the attribute.

Example:
1. Use the default value:
   ```rust
   #[derive(FlatMessage)]
   struct Test {
       #[flat_message_item(skip = true)]
       a: u32,
   }
   ```
   In this case, the field `a` will be initialized to `0` (the default value for `u32`).
2. Use the value of the attribute `default`:
   ```rust
   #[derive(FlatMessage)]
   struct Test {
       #[flat_message_item(skip = true, default = 10)]
       a: u32,
   }
   ```
   In this case, the field `a` will be initialized to `10`.

## Custom Default Values

When using the attribute `default`, you can specify a custom default value for the field in the following ways:
1. A constant value (e.g. `default = 10` or `default = MY_CONSTANT`).
2. A string representation of the value (e.g. `default = "10"` ). In this case the value is parsed and adjusted to fit the actual type of the field.
3. A raw string representation of the value (e.g. `default = r#"foo(1,2,3)"#`). In this case the value is use exactly as is. This is in particular useul if you want to use exprssior or a call to a functon to initialize the value.

## String representations

