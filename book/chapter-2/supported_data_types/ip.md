# IP Addresses

| Data Type                              | Object | Slice | Vector |
| -------------------------------------- | ------ | ----- | ------ |
| IP v4: `Ipv4Addr` (std::net::Ipv4Addr) | Yes    | -     | -      |
| IP v6: `Ipv6Addr` (std::net::Ipv6Addr) | Yes    | -     | -      |
| IP enum: `IpAddr` (std::net::IpAddr)   | Yes    | -     | -      |

**Remarks:**
- The serialization size for `Ipv4Addr` is 4 bytes, and for `Ipv6Addr` it is 16 bytes.
- The serialization size for `IpAddr` is 5 bybtes (if it is an `Ipv4Addr`) or 17 bytes (if it is an `Ipv6Addr`).

## Example

1. Direct values:
    ```rust
    use flat_message::*;
    use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};

    #[derive(FlatMessage)]
    struct Example {
        ipv4_address: Ipv4Addr,
        ipv6_address: Ipv6Addr,
        ip_address: IpAddr,
    }
    ```
