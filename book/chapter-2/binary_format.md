# Binary Format

| Offset | Name                | Type                   | Observations                                                                                    |
| ------ | ------------------- | ---------------------- | ----------------------------------------------------------------------------------------------- |
| +0     | Magic               | (3 bytes)              | 'FLM'                                                                                           |
| +3     | Format version      | u8                     | currently value **1**                                                                           |
| +4     | Number of fields    | u16                    | Total number of fields (data members) in the structure                                          |
| +6     | Structure version   | Option<NonZeroU8>      | For structures that have multiple version, this byte holds the current version of the structure |
| +7     | Serializarion flags | u8                     | 8 bits that provide information on the data                                                     |
| +8     | Data                |                        | The actual data from the structure                                                              |
| +?     | Hash table          | u32 * Number of fields | A hash table for quick access to the data                                                       |
| +?     | Offset table        | ? * Number of fields   | A table with indexes from where the data starts                                                 |
| +?     | Timestamp           | u64                    | Only if the **TIMESTAMP** flag was set                                                          |
| +?     | Unique ID           | u64                    | Only if the **UNIQUEID** flag was set                                                           |
| +?     | Structure Name Hash | u32                    | Only if the **MAKEHASH** flag was set                                                           |
| +?     | Data checksum       | u32                    | Only if **CHECKSUM** flag was set                                                               |

**Remarks:**
- The **Magic** field is used to identify the file format. It should always be 'FLM'.
- The **Structure version** field is used to indicate the version of the structure. Version `0` means that the structure has no versioning.
- The **Serializarion flags** field is a bitmask that provides information about the data. The following flags are currently supported:
  - **TIMESTAMP**: Indicates that the structure has a timestamp.
  - **UNIQUEID**: Indicates that the structure has a unique ID.
  - **MAKEHASH**: Indicates that the structure has a name hash.
  - **CHECKSUM**: Indicates that the structure has a checksum.
- The first 2 bits from the **Serializarion flags** field are use for offset size (1, 2 or 4 bytes). Smaller structus usually use 1 byte offset (meaning that the endire data is less than 255 bytes), while larger structs use a 2 or 4 bytes offset.