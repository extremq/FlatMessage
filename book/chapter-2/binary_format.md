# Binary Format

| Offset | Name                | Type                   | Observations                                                                                    |
| ------ | ------------------- | ---------------------- | ----------------------------------------------------------------------------------------------- |
| +0     | Magic               | (3 bytes)              | 'FLM'                                                                                           |
| +3     | Format version      | u8                     | currently value **1**                                                                           |
| +4     | Number of fields    | u16                    | Total number of fields (data members) in the structure                                          |
| +6     | Structure version   | u8                     | For structures that have multiple version, this byte holds the current version of the structure |
| +7     | Serializarion flags | u8                     | 8 bits that provide information on the data                                                     |
| +8     | Data                |                        | The actual data from the structure                                                              |
| +?     | Hash table          | u32 * Number of fields | A hash table for quick access to the data                                                       |
| +?     | Offset table        | ? * Number of fields   | A table with indexes from where the data starts                                                 |
| +?     | Timestamp           | u64                    | Only if the **TIMESTAMP** flag was set                                                          |
| +?     | Unique ID           | u64                    | Only if the **UNIQUEID** flag was set                                                           |
| +?     | Structure Name Hash | u32                    | Only if the **MAKEHASH** flag was set                                                           |
| +?     | Data checksum       | u32                    | Only if **CHECKSUM** flag was set                                                               |

