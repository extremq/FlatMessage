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

| Method | Size (b) | Serialization Time (ms) | Deserialization Time (ms) | Total Time (ms) |
| ------ | -------: | ----------------------: | ------------------------: | --------------: |
| flat_message_unchecked | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;+69%]</span> |   1.47 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;1.19&nbsp;-&nbsp;&nbsp;&nbsp;1.65]</span> |   1.78 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;1.35&nbsp;-&nbsp;&nbsp;&nbsp;2.21]</span> |   3.70 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;2.86&nbsp;-&nbsp;&nbsp;&nbsp;3.95]</span> |
| flat_message | 355 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;+69%]</span> |   1.63 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;1.29&nbsp;-&nbsp;&nbsp;&nbsp;1.81]</span> |   1.92 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;1.47&nbsp;-&nbsp;&nbsp;&nbsp;2.29]</span> |   3.75 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;3.25&nbsp;-&nbsp;&nbsp;&nbsp;6.47]</span> |
| bincode  <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[schema]</span>| 238 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;+13%]</span> |   1.76 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;1.43&nbsp;-&nbsp;&nbsp;&nbsp;1.93]</span> |   4.17 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;3.13&nbsp;-&nbsp;&nbsp;&nbsp;6.27]</span> |   6.39 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;5.05&nbsp;-&nbsp;&nbsp;&nbsp;7.18]</span> |
| postcard  <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[schema]</span>| 154 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;-27%]</span> |   2.66 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;2.07&nbsp;-&nbsp;&nbsp;&nbsp;2.83]</span> |   3.32 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;2.60&nbsp;-&nbsp;&nbsp;&nbsp;3.53]</span> |   6.49 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;5.30&nbsp;-&nbsp;&nbsp;&nbsp;6.85]</span> |
| rmp_schema  <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[schema]</span>| 179 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;-15%]</span> |   3.52 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;3.01&nbsp;-&nbsp;&nbsp;&nbsp;4.64]</span> |   5.19 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;4.80&nbsp;-&nbsp;&nbsp;&nbsp;5.71]</span> |   9.58 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;8.23&nbsp;-&nbsp;&nbsp;10.56]</span> |
| rmp_schemaless | 776 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[+269%]</span> |   5.99 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;4.76&nbsp;-&nbsp;&nbsp;&nbsp;7.52]</span> |  11.36 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;10.02&nbsp;-&nbsp;&nbsp;12.62]</span> |  18.12 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;15.02&nbsp;-&nbsp;&nbsp;20.29]</span> |
| cbor | 786 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[+274%]</span> |   7.58 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;6.08&nbsp;-&nbsp;&nbsp;&nbsp;8.71]</span> |  17.75 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;14.34&nbsp;-&nbsp;&nbsp;21.51]</span> |  26.68 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;21.61&nbsp;-&nbsp;&nbsp;29.62]</span> |
| bson | 885 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[+321%]</span> |   6.93 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;&nbsp;5.74&nbsp;-&nbsp;&nbsp;&nbsp;7.31]</span> |  20.86 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;17.07&nbsp;-&nbsp;&nbsp;23.54]</span> |  29.10 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;24.70&nbsp;-&nbsp;&nbsp;31.70]</span> |
| json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[+326%]</span> |  26.60 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;22.61&nbsp;-&nbsp;&nbsp;31.36]</span> |  15.65 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;12.04&nbsp;-&nbsp;&nbsp;16.25]</span> |  43.69 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;35.96&nbsp;-&nbsp;&nbsp;46.42]</span> |
| simd_json | 895 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[+326%]</span> |  17.30 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;13.95&nbsp;-&nbsp;&nbsp;18.40]</span> |  35.51 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;27.20&nbsp;-&nbsp;&nbsp;37.58]</span> |  55.36 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;42.97&nbsp;-&nbsp;&nbsp;60.42]</span> |
| flexbuffers | 1022 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[+386%]</span> | 125.06 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[104.57&nbsp;-&nbsp;137.95]</span> |  26.28 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[&nbsp;20.95&nbsp;-&nbsp;&nbsp;27.78]</span> | 156.67 <span style="font-family:monospace; opacity:0.5; font-size:0.75em">[127.87&nbsp;-&nbsp;173.90]</span> |

### 2. MacOs Execution

