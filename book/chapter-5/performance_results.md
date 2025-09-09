# Performance Results

The following tests were conducted against ofther serializers and deserializers crates:
* **performance** - different structures were serialized and deserialized and the time needed for this operation was measured
* **size** - the size of the serialized data was measured for different structures


## Crates

The following crates were tested:

| Crate / method         | Version | Schema Type | Observation                                                                                                 |
| ---------------------- | ------- | ----------- | ----------------------------------------------------------------------------------------------------------- |
| flat_message           | 0.1.0   | Schema-less | For deserialization the deserialize(...) method is beng used                                                |
| flat_message_unchecked | 0.1.0   | Schema-less | For deserialization the deserialize_unchecked(...) method is beng used (meaning that no validation is done) |
| bincode                | 2.0.1   | with Schema | also use bincode_derive (2.0.1)                                                                             |
| bson                   | 3.0.0   | Schema-less |                                                                                                             |
| flexbuffers            | 25.2.10 | Schema-less |                                                                                                             |
| postcard               | 1.1.3   | with Schema |                                                                                                             |
| serde_json             | 1.0.143 | Schema-less |                                                                                                             |
| simd_json              | 0.15.1  | Schema-less |                                                                                                             |
| ciborium               | 0.2.2   | Schema-less |                                                                                                             |
| rmp                    | 0.8.14  | both        | also included rmp-serde for MessagePack (v1.3.0)                                                            |


## Methodology

Each test consists doing the following for a chosen structure:
* Serialize the structure for `n` times (repetitions) and measure the time needed to perform this operations
* Deserialize a buffer containing the serialized data for `n` times (repetitions) and measure the time needed to perform this operations
* Serialize and then deserialize the structure for `n` times (repetitions) and measure the time needed to perform this operations

The `n` parameter is usually a larger one (**>1000**) as usually de serialization/deserialization process is really fast and measuring it for a smaller number of times would not be representative.

Each repetition of "n" times is performed for "k" iterations and the times for each iterations are stored. From these, the median time is calculated. We prefer median time over average time as it is less sensitive to outliers.

The result for each tested structure (in terms of time) will be presended in the following way: `median [min - mac]`. For example: `1.5 [1.2 - 1.8]` means that the median time is **1.5ms**, the minimum time is **1.2ms** and the maximum time is **1.8ms**.

The following algorithm simulates how times are computed:

```cpp
times = []
for iteration in 0..k {
    start = GetCurrentTime()
    for repetition in 0..n {
        Serialize(structure)
    }
    end = GetCurrentTime()
    times.push(end - start)
}
return (menian(times), min(times), max(times))
```
For each structure we also compute the `Data size` (the minimum size required to store the data from that structure). That value is compared to the actual size of the serialized buffer. In most cases (since the serialized buffer is usually bigger than the data size) the percentage of increase is reported. The size value presented for each serialization method is presented as follows: `size [+/- percentage]`. For example: `355 [+69%]` means that the size of the serialized buffer is **355 bytes** and the data size is **209 bytes** (so the percentage of increase is **69%** for that method).

**Remarks**: It is important to highlight that some of the methods used are not schema-less (they will be marked with `schema` next to the name of the method). In these cases, it is possible that the actual size will be smaller than the data size (in particular if the serialization method compress some of the data)

## OSes

The tests were performed on the following OSes:
1. **Windows** - Windows 11, 64 bit,11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz (2.80 GHz), RAM 32.0 GB 
2. **MacOS** - 


