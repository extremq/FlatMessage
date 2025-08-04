use crate::*;
use flat_message::*;

#[test]
fn check_int() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    validate_correct_serde(Test {
        v1: 1,
        v2: 2,
        v3: 3,
        v4: 4,
    });
}

#[test]
fn check_int_negative() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    validate_correct_serde(Test {
        v1: -1,
        v2: -2,
        v3: -3,
        v4: -4,
    });
}

#[test]
fn check_int_min() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    validate_correct_serde(Test {
        v1: i8::MIN,
        v2: i16::MIN,
        v3: i32::MIN,
        v4: i64::MIN,
    });
}

#[test]
fn check_int_max() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    validate_correct_serde(Test {
        v1: i8::MAX,
        v2: i16::MAX,
        v3: i32::MAX,
        v4: i64::MAX,
    });
}

#[test]
fn check_uint() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: u8,
        v2: u16,
        v3: u32,
        v4: u64,
    }
    validate_correct_serde(Test {
        v1: 1,
        v2: 2,
        v3: 3,
        v4: 4,
    });
}

#[test]
fn check_uint_min() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: u8,
        v2: u16,
        v3: u32,
        v4: u64,
    }
    validate_correct_serde(Test {
        v1: u8::MIN,
        v2: u16::MIN,
        v3: u32::MIN,
        v4: u64::MIN,
    });
}

#[test]
fn check_uint_max() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: u8,
        v2: u16,
        v3: u32,
        v4: u64,
    }
    validate_correct_serde(Test {
        v1: u8::MAX,
        v2: u16::MAX,
        v3: u32::MAX,
        v4: u64::MAX,
    });
}

#[test]
fn check_int_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    let mut v = Storage::default();
    let s = Test {
        v1: 0x55,
        v2: 0x6677,
        v3: 0x11223344,
        v4: 0x55667788_99AABBCC,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 4, 0, 0, 0, // Heder
            85, // v1 - 0x55
            68, 51, 34, 17, // v3 = 0x11223344
            119, 102, // v2 = 0x6677
            204, 187, 170, 153, 136, 119, 102, 85, // v4 = 0x55667788_99AABBCC
            0, // padding
            6, 70, 74, 148, // hash for v1
            8, 73, 74, 150, // hash for v3
            7, 75, 74, 151, // hash for v2
            9, 78, 74, 153, // hash for v4
            8, // offset for v1
            9, // offset for v3
            13, // offset of v2
            15, // offset of v4
        ]
    );
}

#[test]
fn check_uint_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: i8,
        v2: i16,
        v3: i32,
        v4: i64,
    }
    let mut v = Storage::default();
    let s = Test {
        v1: 1,
        v2: 2,
        v3: 3,
        v4: 4,
    };
    s.serialize_to(&mut v, Config::default()).unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 4, 0, 0, 0, // Header
            1, // v1 - 1
            3, 0, 0, 0, // v3 = 3
            2, 0, // v2 = 2
            4, 0, 0, 0, 0, 0, 0, 0, // v4 = 4
            0, // padding
            6, 70, 74, 148, // hash for v1
            8, 73, 74, 150, // hash for v3
            7, 75, 74, 151, // hash for v2
            9, 78, 74, 153, // hash for v4
            8, // offset for v1 - 8
            9, // offset for v3 - 9
            13, // offset for v2 - 13
            15 // offset for v4 - 15
        ]
    );
}