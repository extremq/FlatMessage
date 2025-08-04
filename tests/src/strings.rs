use crate::*;
use flat_message::*;

#[test]
fn check_simple_object() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        s1: String,
        s2: &'a str,
    }
    let t = Test {
        s1: "Hello".to_string(),
        s2: "World",
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    let deserialized = Test::deserialize_from(&v).unwrap();
    assert_eq!(t, deserialized);
}

#[test]
fn check_string_repr() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        s1: String,
        s2: &'a str,
    }
    let t = Test {
        s1: "Hello".to_string(),
        s2: "World",
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    let buf = v.as_slice();
    assert_eq!(buf, &[
        70, 76, 77, 1, 2, 0, 0, 0, // Header
        5, // size of s1 (Hello)
        87, 111, 114, 108, 100,  // s1 - "Hello"
        5, // size of s2 (World)
        72, 101, 108, 108, 111, // s2 - "World"
        14, 180, 81, 5, // hash for s1 
        14, 184, 81, 8, // hash for s2
        8, // offset for s1 - 8
        14 // offset for s2 - 14
    ]);
}


#[test]
fn check_vector_object() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test<'a> {
        s1: Vec<String>,
        s2: Vec<&'a str>,
    }
    let t = Test {
        s1: vec!["Hello".to_string(), "World".to_string()],
        s2: vec!["abc", "xyz"],
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    let deserialized = Test::deserialize_from(&v).unwrap();
    assert_eq!(t, deserialized);
}
