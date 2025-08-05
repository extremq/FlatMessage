use crate::*;
use flat_message::*;

#[test]
fn check_simple_array() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: [u8; 10],
    }
    validate_correct_serde(Test {
        v1: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    });
}

#[test] 
fn check_simple_array_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: [u8; 10],
    }
    let t = Test {
        v1: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(s.as_slice(), &[
        70, 76, 77, 1, 1, 0, 0, 0, // header
        10, // v1 - size of the array
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, // v1 - values   
        0, // padding
        26, 70, 74, 148, // hash for v1 (26 = type for FixArray)
        8 // offset of v1
    ]);
}
