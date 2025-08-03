use crate::*;
use flat_message::*;
use std::net::*;

#[test]
fn check_ipv4() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        ip1: std::net::Ipv4Addr,
        ip2: std::net::Ipv4Addr,
    }
    validate_correct_serde(Test {
        ip1: Ipv4Addr::new(192, 168, 1, 0),
        ip2: Ipv4Addr::new(10, 0, 0, 1),
    });
}

// #[test]
// fn check_ipv4_vector() {
//     #[flat_message(metadata: false, store_name: false)]
//     #[derive(Debug, PartialEq, Eq)]
//     struct Test {
//         ip1: std::net::Ipv4Addr,
//         ip2: Vec<std::net::Ipv4Addr>,
//     }
//     validate_correct_serde(Test {
//         ip1: Ipv4Addr::new(192, 168, 1, 0),
//         ip2: vec![Ipv4Addr::new(10, 0, 0, 1),Ipv4Addr::new(8, 8, 8, 8), Ipv4Addr::new(1, 2, 3, 4)],
//     });
// }

#[test]
fn check_ipv4_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        ip1: std::net::Ipv4Addr,
        ip2: std::net::Ipv4Addr,
    }
    let mut v = Storage::default();
    let s = Test {
        ip1: Ipv4Addr::new(192, 168, 1, 0),
        ip2: Ipv4Addr::new(10, 0, 0, 1),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = Test::deserialize_from(&v).unwrap();
    assert_eq!(s.ip1, ds.ip1);
    assert_eq!(s.ip2, ds.ip2);
    //println!("{:?}", v.as_slice());
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            71, 84, 72, 1, 2, 0, 0, 0, 
            // ip1 - 192.168.1.0
            192, 168, 1, 0, 
            // ip2 - 10.0.0.1
            10, 0, 0, 1, 
            // hash for ip1 (23 = ID for IPv4)
            23, 233, 1, 215, 
            // hash for ip2 (23 = ID for IPv4)
            23, 235, 1, 216,             
            // offset for ip1 - 8
            8, 
            // offset for ip2 - 12
            12
        ]
    );
}

#[test]
fn check_ipv6() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        ip1: std::net::Ipv6Addr,
        ip2: std::net::Ipv6Addr,
    }
    validate_correct_serde(Test {
        ip1: Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8),
        ip2: Ipv6Addr::new(0x1111,0x2222,0x3333,0x4444,0x5555,0x6666,0x7777,0x8888),
    });
}

#[test]
fn check_ipv6_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        ip1: std::net::Ipv6Addr,
        ip2: std::net::Ipv6Addr,
    }
    let mut v = Storage::default();
    let s = Test {
        ip1: Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8),
        ip2: Ipv6Addr::new(0x1111,0x2222,0x3333,0x4444,0x5555,0x6666,0x7777,0x8888),
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    let ds = Test::deserialize_from(&v).unwrap();
    assert_eq!(s.ip1, ds.ip1);
    assert_eq!(s.ip2, ds.ip2);
    //println!("{:?}", v.as_slice());
    assert_eq!(
        v.as_slice(),
        &[
            // Header
            71, 84, 72, 1, 2, 0, 0, 0, 
            // ip1 - 1, 2, 3, 4, 5, 6, 7, 8
            0,1, 0,2, 0,3, 0,4, 0,5, 0,6, 0,7, 0,8,
            // ip2 - 0x1111,0x2222,0x3333,0x4444,0x5555,0x6666,0x7777,0x8888,
            17, 17, 34, 34, 51, 51, 68, 68, 85, 85, 102, 102, 119, 119, 136, 136,
            // hash for ip1 (24 = ID for IPv6)
            24, 233, 1, 215, 
            // hash for ip2 (24 = ID for IPv6)
            24, 235, 1, 216,             
            // offset for ip1 - 8
            8, 
            // offset for ip2 - 24
            24
        ]
    );
}