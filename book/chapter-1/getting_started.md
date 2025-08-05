# Getting Started

FlatMessage is a **zero-copy**, **schema-less** serialization library built for Rust, offering efficient and flexible data serialization.

It is designed to be `fast`, `memory-efficient`, and `easy to use`, making it suitable for a wide range of applications, from simple data storage to complex network protocols.

A **schema-less** library means that you don't need to define a schema before you can start serializing and deserializing data (all necessary information is stored in the data itself). This means that the output buffer is larger than the total size of the data, but you are more flexible in scenarios where your data changes often.

A **zero-copy** library means that you don't need to copy the data from the serialized buffer; you can use references and slices to access the data directly. This is useful for larger datasets where copying them would add a performance penalty.