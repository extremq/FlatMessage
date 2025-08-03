# Supported Data Types

FlatMessage supports a variety of data types for serialization in the following way:
1. as a direct value:
   ```rust
   struct Name { value: T } 
   ```
   where `T` is the data type.

2. as a slice of values:
   ```rust
   struct Name { value: &[T] } 
   ```
   where `T` is the data type.
   
3. as a vector of values:
    ```rust
    struct Name { value: Vec<T> } 
    ```
    where `T` is the data type.

The main difference between a slice and a vector is that a slice is a reference to an array of values, while a vector is an owned collection of values. Slices are more memory efficient, but vectors provide more flexibility in terms of resizing and ownership.
You can use them interchangeably, meaning that you can serialize an object that has a vector field and deserialize it into a slice, or vice versa. FlatMessage will handle the conversion automatically.

Keep in mind that deserialization of a slice is a `no-cost operation`, while deserialization of a vector requires allocation and copying of the data, which may incur some performance overhead.