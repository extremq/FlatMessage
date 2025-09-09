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
| FlatMessage | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;+69%]</span> |   1.54 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;1.32&nbsp;-&nbsp;&nbsp;&nbsp;1.79]</span> |   1.68 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;1.51&nbsp;-&nbsp;&nbsp;&nbsp;1.83]</span> |   3.18 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;2.96&nbsp;-&nbsp;&nbsp;&nbsp;4.01]</span> |
| FlatMessage (&#9888;&#65039;) | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;+69%]</span> |   1.21 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;1.17&nbsp;-&nbsp;&nbsp;&nbsp;1.57]</span> |   1.48 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;1.37&nbsp;-&nbsp;&nbsp;&nbsp;1.99]</span> |   3.36 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;2.80&nbsp;-&nbsp;&nbsp;&nbsp;4.25]</span> |
| *bincode* <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br><span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[schema]</span></span>| 172 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;-19%]</span> |   1.83 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;1.69&nbsp;-&nbsp;&nbsp;&nbsp;2.03]</span> |   2.60 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;2.41&nbsp;-&nbsp;&nbsp;&nbsp;2.81]</span> |   4.79 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;4.44&nbsp;-&nbsp;&nbsp;&nbsp;5.15]</span> |
| *postcard* <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br><span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[schema]</span></span>| 154 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;-27%]</span> |   2.25 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;2.08&nbsp;-&nbsp;&nbsp;&nbsp;2.80]</span> |   2.75 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;2.63&nbsp;-&nbsp;&nbsp;&nbsp;3.37]</span> |   5.48 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;5.05&nbsp;-&nbsp;&nbsp;&nbsp;7.02]</span> |
| *rmp* <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br><span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[schema]</span></span>| 179 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;-15%]</span> |   2.88 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;2.73&nbsp;-&nbsp;&nbsp;&nbsp;4.24]</span> |   4.95 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;4.36&nbsp;-&nbsp;&nbsp;&nbsp;5.65]</span> |   9.12 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;7.85&nbsp;-&nbsp;&nbsp;&nbsp;9.97]</span> |
| rmp | 776 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+269%]</span> |   5.18 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;4.79&nbsp;-&nbsp;&nbsp;&nbsp;6.00]</span> |  10.81 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;10.03&nbsp;-&nbsp;&nbsp;13.34]</span> |  16.76 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;15.91&nbsp;-&nbsp;&nbsp;17.79]</span> |
| cbor | 786 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+274%]</span> |   6.23 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;5.86&nbsp;-&nbsp;&nbsp;&nbsp;7.19]</span> |  14.87 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;13.46&nbsp;-&nbsp;&nbsp;18.00]</span> |  21.65 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;20.21&nbsp;-&nbsp;&nbsp;25.03]</span> |
| bson | 885 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+321%]</span> |   7.02 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;&nbsp;6.36&nbsp;-&nbsp;&nbsp;&nbsp;8.03]</span> |  18.12 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;17.18&nbsp;-&nbsp;&nbsp;22.84]</span> |  26.51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;25.02&nbsp;-&nbsp;&nbsp;31.99]</span> |
| json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+326%]</span> |  21.77 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;20.28&nbsp;-&nbsp;&nbsp;25.74]</span> |  13.66 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;12.46&nbsp;-&nbsp;&nbsp;16.09]</span> |  36.64 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;34.48&nbsp;-&nbsp;&nbsp;43.39]</span> |
| simd_json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+326%]</span> |  14.40 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;13.59&nbsp;-&nbsp;&nbsp;17.36]</span> |  28.51 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;27.18&nbsp;-&nbsp;&nbsp;34.44]</span> |  45.20 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;43.98&nbsp;-&nbsp;&nbsp;54.26]</span> |
| flexbuffers | 1022 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+386%]</span> | 109.11 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[104.37&nbsp;-&nbsp;119.52]</span> |  21.98 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;20.28&nbsp;-&nbsp;&nbsp;24.77]</span> | 130.84 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[124.33&nbsp;-&nbsp;193.94]</span> |
| toml | 894 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[+325%]</span> |  21.56 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[&nbsp;20.34&nbsp;-&nbsp;&nbsp;25.31]</span> | 141.05 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[134.05&nbsp;-&nbsp;163.29]</span> | 167.54 <span style="font-family:monospace; opacity:0.5; font-size:0.5em"><br>[161.27&nbsp;-&nbsp;195.29]</span> |
| protobuf | - | - | - | - |


### 2. MacOs Execution

