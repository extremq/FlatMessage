use crate::*;
use flat_message::*;
use std::marker::PhantomData;

#[test]
fn check_phanton_data_serde() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: PhantomData<u32>,
    }
    validate_correct_serde(Test {
        x: 12,
        y: PhantomData,
    });
}

#[test]
fn check_phanton_data_missing() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test1 {
        x: u8,
        y: PhantomData<u32>,
    }
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test2 {
        x: u8,
    }
    let mut v1 = Storage::default();
    let t1 = Test1 {
        x: 12,
        y: PhantomData,
    };
    t1.serialize_to(&mut v1, Config::default()).unwrap();
    let mut v2 = Storage::default();
    let t2 = Test2 { x: 12 };
    t2.serialize_to(&mut v2, Config::default()).unwrap();
    assert_eq!(v1.as_slice(), v2.as_slice(),);
}
