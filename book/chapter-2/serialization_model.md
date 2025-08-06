# Serialization Model

FlatMessage uses a **field-based serialization model** that stores data in a structured binary format with hash tables for fast field lookup. The key advantage is **zero-copy deserialization** - instead of reconstructing objects in memory, it provides direct references to data within the serialized buffer.

This approach enables instant deserialization with no memory allocation, making it ideal for high-performance scenarios where read speed and memory efficiency are critical.