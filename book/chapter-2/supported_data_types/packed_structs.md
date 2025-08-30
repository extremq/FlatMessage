# Packed Structs

Packed structs are a high-performance, memory-efficient serialization format optimized for sequential data layouts and minimal overhead. Unlike regular structs, packed structs store data in a continuous memory layout without hash tables or field lookups.

| Data Type                                          | Object | Slice | Vector | Option |
| -------------------------------------------------- | ------ | ----- | ------ | ------ |
| Custom structs with `#[derive(FlatMessagePacked)]` | Yes    | -     | -      | No     |

**Supported alignments:**
- 1-byte alignment (fields requiring only byte alignment)
- 2-byte alignment (fields requiring 16-bit alignment)
- 4-byte alignment (fields requiring 32-bit alignment - default for most cases)
- 8-byte alignment (fields requiring 64-bit alignment - such as Vec<u64>)
- 16-byte alignment (fields requiring 128-bit alignment - such as Vec<u128>)

**Key Characteristics:**
- **High Performance**: Sequential memory layout provides optimal cache performance
- **Low Overhead**: No hash tables or field lookup structures
- **Compact Size**: Minimal metadata, only structure hash for validation
- **Field Reordering**: Fields are automatically reordered by alignment (largest first) for optimal packing
- **Version Compatibility**: Uses structure hash for version validation

**Restrictions:**
- No `Option<T>` types supported
- No `Timestamp` or `UniqueID` metadata fields
- All fields must be mandatory (no `mandatory = false`)
- No default values on deserialization failure (no `validate = fallback`)
- Maximum 65,535 fields per struct
- Fields can be ignored with `#[flat_message_item(ignore = true)]`

## Usage

Packed structs must derive `FlatMessagePacked` and are used with `kind = packed` in field attributes:

```rust
use flat_message::*;

#[derive(FlatMessagePacked, Debug, PartialEq, Eq)]
struct MyPackedData {
    x: i32,
    y: u32,
    label: String,
}

#[derive(FlatMessage, Debug)]
struct Container {
    #[flat_message_item(kind = packed, align = 1)]
    data: MyPackedData,
    other_field: String,
}
```

## Examples

### 1. Basic Packed Struct (1-byte alignment)

```rust
use flat_message::*;

#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
struct Point {
    x: i32,
    y: u32,
    label: String,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
struct Test {
    #[flat_message_item(kind = packed, align = 1)]
    point: Point,
    description: String,
}

fn example() {
    let test_data = Test {
        point: Point {
            x: 10,
            y: 20,
            label: "Origin".to_string(),
        },
        description: "Test point".to_string(),
    };
    
    // Serialize and deserialize
    let mut storage = Storage::default();
    test_data.serialize_to(&mut storage, Config::default()).unwrap();
    let deserialized = Test::deserialize_from(&storage).unwrap();
    
    assert_eq!(test_data, deserialized);
}
```

### 2. Packed Struct with 4-byte Alignment

```rust
use flat_message::*;

#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
struct DataSet {
    data: Vec<u32>,
    index: u8,
}

#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
struct ComplexPoint {
    x: i8,
    y: i8,
    #[flat_message_item(kind = packed, align = 4)]
    dataset1: DataSet,
    #[flat_message_item(kind = packed, align = 4)]
    dataset2: DataSet,
}

#[derive(Debug, PartialEq, Eq, FlatMessage)]
#[flat_message_options(store_name = false)]
struct Container {
    #[flat_message_item(kind = packed, align = 4)]
    point: ComplexPoint,
    name: String,
}

fn example() {
    let container = Container {
        point: ComplexPoint {
            x: 10,
            y: 20,
            dataset1: DataSet {
                data: vec![1, 2, 3],
                index: 1,
            },
            dataset2: DataSet {
                data: vec![4, 5, 6],
                index: 2,
            },
        },
        name: "Complex data".to_string(),
    };
    
    // Efficient serialization and deserialization
    let mut storage = Storage::default();
    container.serialize_to(&mut storage, Config::default()).unwrap();
    let result = Container::deserialize_from(&storage).unwrap();
    
    assert_eq!(container, result);
}
```

### 3. Packed Struct with Ignored Fields

```rust
use flat_message::*;

#[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
struct ProcessInfo {
    pid: u32,
    name: String,
    memory_usage: u64,
    #[flat_message_item(ignore = true)]
    runtime_data: String,  // Not serialized, will be default on deserialization
}

#[derive(Debug, FlatMessage)]
struct SystemSnapshot {
    #[flat_message_item(kind = packed, align = 8)]
    processes: Vec<ProcessInfo>,
    timestamp: u64,
}

fn example() {
    let snapshot = SystemSnapshot {
        processes: vec![
            ProcessInfo {
                pid: 1234,
                name: "my_app".to_string(),
                memory_usage: 1024 * 1024 * 50, // 50MB
                runtime_data: "This will be ignored".to_string(),
            },
        ],
        timestamp: 1640995200,
    };
    
    let mut storage = Storage::default();
    snapshot.serialize_to(&mut storage, Config::default()).unwrap();
    let result = SystemSnapshot::deserialize_from(&storage).unwrap();
    
    // runtime_data will be empty string (default) after deserialization
    assert_eq!(result.processes[0].runtime_data, String::default());
    assert_eq!(result.processes[0].pid, 1234);
}
```

## Serialization Behavior

Packed structs use a fundamentally different serialization approach compared to regular structs:

### 1. **Sequential Layout**
Fields are stored sequentially in memory without indirection:
```
[Structure Hash][Field1][Padding if required][Field2][Padding if required][Field3]...
```

### 2. **Automatic Field Reordering**
Fields are automatically reordered by alignment requirements (largest alignment first) to minimize padding and optimize memory layout.

### 3. **Minimal Metadata**
- Only a structure hash (4 bytes) is stored for version validation
- No field hash tables or lookup structures
- No field offset tables

### 4. **Alignment-Based Padding**
- Padding is inserted between fields as needed for proper alignment
- The struct's overall alignment is determined by its largest field alignment requirement
- Padding follows standard C struct alignment rules

### 5. **Structure Hash Validation**
- A FNV-32 hash of the structure definition is stored at the beginning
- Hash includes field names, types, data formats, and alignment requirements
- Deserialization fails if the hash doesn't match the expected structure

## Performance Characteristics

| Aspect                    | Packed Structs | Regular Structs | Observation                        |
| ------------------------- | -------------- | --------------- | ---------------------------------- |
| **Serialization Speed**   | Very Fast      | Moderate        | Sequential writes, no hash tables  |
| **Deserialization Speed** | Very Fast      | Fast            | Sequential reads, no field lookups |
| **Memory Overhead**       | Minimal        | Moderate        | Only 4-byte hash vs hash tables    |
| **Cache Performance**     | Excellent      | Good            | Sequential access pattern          |
| **Version Flexibility**   | Limited        | High            | Strict hash matching required      |
| **Random Field Access**   | Not Supported  | Fast            | Must deserialize entire struct     |

## When to Use Packed Structs

**Choose packed structs when:**
- Performance is critical and you need maximum speed
- Memory usage must be minimized
- Data is accessed sequentially or entirely
- Structure is stable and doesn't change frequently
- You don't need optional fields or metadata

**Choose regular structs when:**
- You need flexibility with optional fields
- Structure evolves frequently
- You need metadata fields (Timestamp, UniqueID)
- Random field access is important
- Backward/forward compatibility is required

## Comparison with Regular Structs

| Feature               | Packed Structs                 | Regular Structs                |
| --------------------- | ------------------------------ | ------------------------------ |
| Derive macro          | `#[derive(FlatMessagePacked)]` | `#[derive(FlatMessageStruct)]` |
| Field attribute       | `kind = packed`                | `kind = struct`                |
| Option types          | ❌ Not supported                | ✅ Supported                    |
| Metadata fields       | ❌ Not supported                | ❌ Not supported                |
| Ignored fields        | ✅ Supported                    | ✅ Supported                    |
| Field reordering      | ✅ Automatic by alignment       | ✅ Automatic by alignment       |
| Hash table            | ❌ No (only structure hash)     | ✅ Yes (field hash table)       |
| Overhead              | Minimal (4 bytes)              | Moderate (hash tables)         |
| Version compatibility | Strict hash match              | Hash-based field lookup        |
| Performance           | Excellent                      | Good                           |


