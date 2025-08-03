use flat_message::*;

#[test]
fn check_flat_message_no_metadata() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(metadata = false)]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }
    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe",
        math: 100,
        engligh: 90,
        passed: true,
        average: 95.0,
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    let metadata = buf.metadata();
    assert_eq!(buf.version(), None);
    assert_eq!(metadata.timestamp(), None);
    assert_eq!(metadata.unique_id(), None);
    assert_eq!(buf.name(), Some(name!("TestStruct")));
}

// #[test]
// fn check_flat_message_metadata() {
//     #[derive(Debug, PartialEq, FlatMessage)]
//     #[flat_message_options(version = 5)]
//     struct TestStruct<'a> {
//         name: String,
//         surname: &'a str,
//         math: u8,
//         engligh: u8,
//         passed: bool,
//         average: f64,
//     }
//     let mut a = TestStruct {
//         name: "John".to_string(),
//         surname: "Doe",
//         math: 100,
//         engligh: 90,
//         passed: true,
//         average: 95.0,
//     };
//     a.update_metada(
//         MetaDataBuilder::new()
//             .timestamp(123456)
//             .unique_id(654321)
//             .build(),
//     );
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     let metadata = buf.metadata();
//     assert_eq!(buf.version(), Some(5));
//     assert_eq!(metadata.timestamp(), Some(123456));
//     assert_eq!(metadata.unique_id(), Some(654321));
//     assert_eq!(buf.name(), Some(name!("TestStruct")));
// }

#[test]
fn check_flat_message_no_metadata_no_name() {
    #[derive(Debug, PartialEq, FlatMessage)]
    #[flat_message_options(store_name = false, metadata = false)]
    struct TestStruct<'a> {
        name: String,
        surname: &'a str,
        math: u8,
        engligh: u8,
        passed: bool,
        average: f64,
    }
    let a = TestStruct {
        name: "John".to_string(),
        surname: "Doe",
        math: 100,
        engligh: 90,
        passed: true,
        average: 95.0,
    };
    let mut output = Storage::default();
    a.serialize_to(&mut output, Config::default()).unwrap();
    let buf = FlatMessageBuffer::try_from(&output).unwrap();
    let metadata = buf.metadata();
    assert_eq!(buf.version(), None);
    assert_eq!(metadata.timestamp(), None);
    assert_eq!(metadata.unique_id(), None);
    assert_eq!(buf.name(), None);
}

#[test]
fn check_unique_id() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: i32,
        id: UniqueID,
    }
    let t = Test {
        x: 1,
        id: UniqueID::with_value(123),
    };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    println!("{:?}", v.as_slice());
}
