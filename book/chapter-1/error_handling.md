# Error Handling

FlatMessage provides comprehensive error handling through the `Error` enum. Understanding these errors helps you build robust serialization code.

## Complete Error Reference

The following table lists all FlatMessage error types with their causes, typical scenarios, and recommended handling strategies:

| Error Type                                  | Parameters         | When It Occurs                                | Typical Cause                  | Recovery Strategy                   |
| ------------------------------------------- | ------------------ | --------------------------------------------- | ------------------------------ | ----------------------------------- |
| `InvalidHeaderLength(usize)`                | Buffer size        | Buffer smaller than minimum header (8 bytes)  | Truncated data, wrong format   | Check data source, validate input   |
| `InvalidMagic`                              | -                  | Magic number doesn't match "FLM\x01"          | Wrong file format, corruption  | Verify file type, check data source |
| `InvalidSize((u32, u32))`                   | (actual, expected) | Size in header doesn't match buffer size      | Partial read, corruption       | Re-read data, validate source       |
| `InvalidOffsetSize`                         | -                  | Invalid offset size encoding in header        | Corruption, unsupported format | Check format version, validate data |
| `InvalidSizeToStoreMetaData((u32, u32))`    | (actual, expected) | Buffer too small for metadata                 | Incomplete data, corruption    | Verify complete transmission        |
| `InvalidHash((u32, u32))`                   | (actual, expected) | CRC32 hash mismatch                           | Data corruption, tampering     | Re-transmit data, check integrity   |
| `InvalidSizeToStoreFieldsTable((u32, u32))` | (actual, expected) | Buffer too small for field table              | Truncated data                 | Ensure complete data transfer       |
| `IncompatibleVersion(u8)`                   | Version number     | Structure version incompatibility             | Version mismatch               | Migrate data, update code           |
| `FieldIsMissing(u32)`                       | Field hash         | Field in data not in struct definition        | Schema evolution, wrong struct | Check struct version, migrate       |
| `InvalidFieldOffset((u32, u32))`            | (actual, max)      | Field offset out of bounds                    | Corruption, format error       | Validate data integrity             |
| `FailToDeserialize(u32)`                    | Field hash         | Failed to deserialize specific field          | Type mismatch, corruption      | Check field compatibility           |
| `NameNotStored`                             | -                  | Name validation requested but not in data     | Missing metadata               | Disable validation or add metadata  |
| `UnmatchedName`                             | -                  | Structure name doesn't match stored name      | Wrong struct type              | Use correct struct, check data      |
| `ChecksumNotStored`                         | -                  | Checksum validation requested but not in data | Missing checksum               | Disable validation or add checksum  |
| `InvalidChecksum((u32, u32))`               | (actual, expected) | Checksum mismatch                             | Data corruption                | Re-transmit, validate source        |
| `ExceedMaxSize((u32, u32))`                 | (actual, max)      | Serialized size exceeds maximum               | Data too large, wrong limit    | Increase limit, reduce data size    |

## Error Categories

### Data Format Errors
- `InvalidHeaderLength`, `InvalidMagic`, `InvalidSize`, `InvalidOffsetSize`
- **Cause**: Malformed or corrupted data format
- **Recovery**: Validate data source, check file integrity

### Data Integrity Errors  
- `InvalidHash`, `InvalidChecksum`
- **Cause**: Data corruption during storage or transmission
- **Recovery**: Re-transmit data, use error correction

### Structure Compatibility Errors
- `IncompatibleVersion`, `FieldIsMissing`, `UnmatchedName`
- **Cause**: Schema evolution, version mismatches
- **Recovery**: Migrate data, update compatibility rules

### Configuration Errors
- `NameNotStored`, `ChecksumNotStored`, `ExceedMaxSize`
- **Cause**: Mismatched configuration between serialization and deserialization
- **Recovery**: Align configurations, adjust limits

### Field-Level Errors
- `InvalidFieldOffset`, `FailToDeserialize`
- **Cause**: Field-specific corruption or type mismatches
- **Recovery**: Validate individual fields, check type compatibility

## Example

The following example shows how to handle errors in a simple way when deserializing a struct.

```rust
use flat_message::*;

// consider that serialized_data is a buffer of the serialized data
let storage = Storage::from_buffer(serialized_data);

// Message is a struct that implements FlatMessage
match Message::deserialize_from(&storage) {
    Ok(restored_message) => {
        println!("Message serialized and deserialized successfully");
    }
    Err(e) => {
        panic!("Error deserializing message: {}", e);
    }
}
```