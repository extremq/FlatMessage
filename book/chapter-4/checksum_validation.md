# Checksum Validation

FlatMessage provides built-in data integrity features through checksums and various validation levels to ensure your data hasn't been corrupted during storage or transmission.

## Checksum Basics

Enable checksums to detect data corruption:

```rust
use flat_message::*;

#[derive(FlatMessage, Debug, PartialEq)]
#[flat_message_options(checksum = true)]
struct ImportantData {
    value: u64,
    message: String,
}

fn checksum_example() -> Result<(), Error> {
    let data = ImportantData {
        value: 12345,
        message: "Critical information".to_string(),
    };

    let mut storage = Storage::default();
    data.serialize_to(&mut storage, Config::default())?;

    // Checksum is automatically calculated and stored
    println!("Serialized with checksum: {} bytes", storage.len());

    // Checksum is automatically verified during deserialization
    let restored = ImportantData::deserialize_from(&storage)?;
    assert_eq!(data, restored);

    Ok(())
}
```

## Checksum Validation Modes

The `validate_checksum` option controls when checksums are validated. You can set it to:
* `always` --> checksum should always be validated
* `never` --> checksum is ignored
* `auto` --> chcksum is checked only if present in the deserialized data

```rust
// Always validate checksums
#[derive(FlatMessage)]
#[flat_message_options(checksum = true, validate_checksum = "always")]
struct AlwaysValidated {
    data: Vec<u8>,
}

// Never validate checksums (performance optimization)
#[derive(FlatMessage)]
#[flat_message_options(checksum = true, validate_checksum = "never")]
struct NeverValidated {
    data: Vec<u8>,
}

// Auto validation (default) - validate only if checksum is present
#[derive(FlatMessage)]
#[flat_message_options(checksum = true, validate_checksum = "auto")]
struct AutoValidated {
    data: Vec<u8>,
}
```

### Validation Mode Behavior

| Mode       | Checksum Present | Checksum Missing | Behavior                 |
| ---------- | ---------------- | ---------------- | ------------------------ |
| `"always"` | ✅ Validates      | ❌ Error          | Always requires checksum |
| `"never"`  | ⚡ Skips          | ✅ Continues      | Never validates          |
| `"auto"`   | ✅ Validates      | ✅ Continues      | Validates if available   |

## Corruption Detection

Checksums detect various types of data corruption:

```rust
fn corruption_detection_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create and serialize data
    let original = ImportantData {
        value: 42,
        message: "Hello World".to_string(),
    };

    let mut storage = Storage::default();
    original.serialize_to(&mut storage, Config::default())?;

    // Simulate corruption by modifying bytes
    let mut corrupted_bytes = storage.as_slice().to_vec();
    corrupted_bytes[10] = 0xFF;  // Corrupt a byte
    let corrupted_storage = Storage::from_buffer(&corrupted_bytes);

    // Attempt to deserialize corrupted data
    match ImportantData::deserialize_from(&corrupted_storage) {
        Err(Error::InvalidChecksum((actual, expected))) => {
            println!("Corruption detected!");
            println!("Expected checksum: 0x{:08X}", expected);
            println!("Actual checksum: 0x{:08X}", actual);
        }
        Ok(_) => panic!("Should have detected corruption"),
        Err(e) => println!("Other error: {}", e),
    }

    Ok(())
}
```

## Performance vs Safety Trade-offs

It is important to note that checksum validation are only performed when using `deserialize_from()`. If you use `deserialize_from_unchecked()`, the checksum validation is skipped (as the data is considered trusted).

This means that `deserialize_from_unchecked()` is always faster than `deserialize_from()`, as it skips the checksum validation and other checks, but it is unsafe as if used incorrectly, it can lead to data corruption.
