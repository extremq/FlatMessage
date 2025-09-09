# Multiple Fields

This benchmarks compares the performance of the different algorithms when serializing and deserializing a message with multiple fields. In particula for schema-less messages, this will show how well different message formats store the schema in the message.
Fields have different basic types (string, u32, u64, i32, i64, f32, f64, bool, u8, i8, u16, i16).

```rust
pub struct MultipleFields {
    field_of_type_string: String,
    field_of_type_u32: u32,
    field_of_type_u64: u64,
    field_of_type_i32: i32,
    field_of_type_i64: i64,
    field_of_type_f32: f32,
    field_of_type_f64: f64,
    field_of_type_bool: bool,
    field_of_type_u8: u8,
    field_of_type_i8: i8,
    field_of_type_u16: u16,
    field_of_type_i16: i16,
    second_field_of_type_string: String,
    second_field_of_type_u32: u32,
    second_field_of_type_u64: u64,
    second_field_of_type_i32: i32,
    second_field_of_type_i64: i64,
    third_field_of_type_string: String,
    third_field_of_type_u32: u32,
    third_field_of_type_u64: u64,
    third_field_of_type_i32: i32,
    third_field_of_type_i64: i64,
    fourth_field_of_type_string: String,
    fourth_field_of_type_u32: u32,
    fourth_field_of_type_u64: u64,
    fourth_field_of_type_i32: i32,
    fourth_field_of_type_i64: i64,
}
```

## Test specs

* Iterations: `k = 10`
* Serialization and deserialization repetitions / iteration: `n = 1000`
* Data size: `210` bytes


## Results

### 1. Windows Execution

| Algorithm | Size (b) | Ser Time (ms) | Deser Time (ms) | Total Time (ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| FlatMessage (&#9888;&#65039;) | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;+69%]</span> |   1.50 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;1.50&nbsp;-&nbsp;&nbsp;&nbsp;1.50]</span> |   1.74 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;1.74&nbsp;-&nbsp;&nbsp;&nbsp;1.74]</span> |   3.49 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;3.49&nbsp;-&nbsp;&nbsp;&nbsp;3.49]</span> |
| FlatMessage | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;+69%]</span> |   2.05 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;2.05&nbsp;-&nbsp;&nbsp;&nbsp;2.05]</span> |   1.95 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;1.95&nbsp;-&nbsp;&nbsp;&nbsp;1.95]</span> |   3.82 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;3.82&nbsp;-&nbsp;&nbsp;&nbsp;3.82]</span> |
| bincode  <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[schema]</span>| 172 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;-19%]</span> |   1.90 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;1.90&nbsp;-&nbsp;&nbsp;&nbsp;1.90]</span> |   2.51 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;2.51&nbsp;-&nbsp;&nbsp;&nbsp;2.51]</span> |   4.91 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;4.91&nbsp;-&nbsp;&nbsp;&nbsp;4.91]</span> |
| postcard  <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[schema]</span>| 154 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;-27%]</span> |   2.27 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;2.27&nbsp;-&nbsp;&nbsp;&nbsp;2.27]</span> |   2.76 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;2.76&nbsp;-&nbsp;&nbsp;&nbsp;2.76]</span> |   5.57 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;5.57&nbsp;-&nbsp;&nbsp;&nbsp;5.57]</span> |
| rmp  <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[schema]</span>| 179 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;-15%]</span> |   4.02 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;4.02&nbsp;-&nbsp;&nbsp;&nbsp;4.02]</span> |   8.26 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;8.26&nbsp;-&nbsp;&nbsp;&nbsp;8.26]</span> |  10.76 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;10.76&nbsp;-&nbsp;&nbsp;10.76]</span> |
| rmp | 776 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[+269%]</span> |   5.20 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;5.20&nbsp;-&nbsp;&nbsp;&nbsp;5.20]</span> |   9.62 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;9.62&nbsp;-&nbsp;&nbsp;&nbsp;9.62]</span> |  15.57 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;15.57&nbsp;-&nbsp;&nbsp;15.57]</span> |
| cbor | 786 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[+274%]</span> |   6.00 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;6.00&nbsp;-&nbsp;&nbsp;&nbsp;6.00]</span> |  15.36 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;15.36&nbsp;-&nbsp;&nbsp;15.36]</span> |  21.82 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;21.82&nbsp;-&nbsp;&nbsp;21.82]</span> |
| bson | 885 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[+321%]</span> |   6.67 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;&nbsp;6.67&nbsp;-&nbsp;&nbsp;&nbsp;6.67]</span> |  16.94 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;16.94&nbsp;-&nbsp;&nbsp;16.94]</span> |  25.26 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;25.26&nbsp;-&nbsp;&nbsp;25.26]</span> |
| json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[+326%]</span> |  21.22 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;21.22&nbsp;-&nbsp;&nbsp;21.22]</span> |  15.56 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;15.56&nbsp;-&nbsp;&nbsp;15.56]</span> |  36.33 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;36.33&nbsp;-&nbsp;&nbsp;36.33]</span> |
| simd_json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[+326%]</span> |  14.64 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;14.64&nbsp;-&nbsp;&nbsp;14.64]</span> |  28.99 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;28.99&nbsp;-&nbsp;&nbsp;28.99]</span> |  45.87 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;45.87&nbsp;-&nbsp;&nbsp;45.87]</span> |
| flexbuffers | 1022 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[+386%]</span> | 108.30 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[108.30&nbsp;-&nbsp;108.30]</span> |  20.42 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;20.42&nbsp;-&nbsp;&nbsp;20.42]</span> | 126.42 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[126.42&nbsp;-&nbsp;126.42]</span> |
| toml | 894 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[+325%]</span> |  22.37 <span style="font-family:monospace; opacity:0.5; font-size:0.66em"><br>[&nbsp;22.37&nbsp;-&nbsp;&nbsp;22.37]</span> | 135.38 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[135.38&nbsp;-&nbsp;135.38]</span> | 162.84 <span style="font-family:monospace; opacity:0.5; font-size:0.66em">[162.84&nbsp;-&nbsp;162.84]</span> |
| protobuf | - | - | - | - |


### 2. MacOs Execution

