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

When using a string representation: `default = "..."` the following steps are checked:
1. if the type is `&str` the value of the **default** attribute is kept as it is.
2. if the type is `String` th value of the **default** attribute is converted into a String
3. if the type is **NOT** a string the quotes (`"`) are removed and the actual value will be used
4. if the type is on option ( `Option<T>` ) then:
    * if the value of the **default** attribute is `None` then the field is set to `None`
    * if the value of the **default** attribute is `Some(T)` then the field is set to `Some(T)`
    * otherwise the value of the **default** attribute is converted into a `Some<value>`
  
## Examples

| Type                                    | Default value               | Actual value                                                                                                                           |
| --------------------------------------- | --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| Numeric (**u8**, **u32**, **f32**, etc) | `default = "10"`            | `10`                                                                                                                                   |
| Numeric (**u8**, **u32**, **f32**, etc) | `default = 123`             | `123`                                                                                                                                  |
| Numeric (**u8**, **u32**, **f32**, etc) | `default = MY_CONSTANT`     | `MY_CONSTANT` (it is assumed that `MY_CONSTANT` exists in the current scope)                                                           |
| Numeric (**u8**, **u32**, **f32**, etc) | `default = r#"1+2+3"#`      | `6`                                                                                                                                    |
| Boolean value (**bool**)                | `default = "true"`          | `true`                                                                                                                                 |
| Boolean value (**bool**)                | `default = false`           | `false`                                                                                                                                |
| Boolean value (**bool**)                | `default = r#"foo(a,b,c)"#` | `foo(a,b,c)` (it is assumed that `a`, `b`, `c` and the function `foo` exists in the current scope)                                     |
| String reference ( **&str** )           | `default = "hello"`         | `"hello"`                                                                                                                              |
| String ( **String** )                   | `default = "hello"`         | `String::from("hello")`                                                                                                                |
| String reference ( **&str** )           | `default = MY_CONSTANT`     | `"MY_CONSTANT"` (it is assumed that `MY_CONSTANT` exists in the current scope)                                                         |
| Option<T>                               | `default = "None"`          | `None`                                                                                                                                 |
| Option<T>                               | `default = "Some(123)"`     | `Some(123)`                                                                                                                            |
| Option<T>                               | `default = MY_CONSTANT`     | `MY_CONSTANT` (it is assumed that `MY_CONSTANT` exists in the current scope and it of type `Option<T>`)                                |
| Option<T>                               | `default = r#"foo(1+2+3)"#` | `foo(1+2+3)` (it is assumed that `foo` exists in the current scope and returns an `Option<T>`)                                         |
| Option<T>                               | `default = "4"`             | `Some(4)` (the value is automatically converted into a `Some<T>`)                                                                      |
| Option<&str>                            | `default = "Hello"`         | `Some("Hello")`                                                                                                                        |
| Option<String>                          | `default = "Hello"`         | `Some(String::from("Hello"))` (first the content of the quotes is converted into a String, then it is converted into a `Some<String>`****) |

**Remark:** If you need to be 100% sure that the value is converted into the correct type, you can use the raw string representation (e.g. `default = r#"Some(1+2+3)"#`).
