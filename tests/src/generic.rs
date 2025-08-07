use super::*;
use flat_message::*;
use std::fmt::Debug;

// macro_rules! check_field_value {
//     ($field_name: expr, $type: ty, $value: expr, $flat_message_buffer: expr) => {
//         let val: $type = $flat_message_buffer.get($field_name).unwrap();
//         assert_eq!(val, $value);
//     };
// }
// macro_rules! check_field_value_unsafe {
//     ($field_name: expr, $type: ty, $value: expr, $flat_message_buffer: expr) => {
//         let val: $type = unsafe { $flat_message_buffer.get_unchecked($field_name).unwrap() };
//         assert_eq!(val, $value);
//     };
// }

// #[test]
// fn check_flat_message_buffer_one_field_i32() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     struct TestStruct {
//         my_field: i32,
//     }
//     let a = TestStruct {
//         my_field: 123456,
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("my_field"), i32, 123456, buf);
// }

// #[test]
// fn check_flat_message_buffer_one_field_str() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     struct TestStruct {
//         my_field: String,
//     }
//     let a = TestStruct {
//         my_field: "Hello, World!".to_string(),
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("my_field"), &str, "Hello, World!", buf);
// }

// #[test]
// fn check_flat_message_buffer_two_fields_i32_i8() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     struct TestStruct {
//         size: i32,
//         dimension: i8,
//     }
//     let a = TestStruct {
//         size: -12345,
//         dimension: -100,
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("size"), i32, -12345, buf);
//     check_field_value!(name!("dimension"), i8, -100, buf);
// }

// #[test]
// fn check_flat_message_buffer_two_fields_string_string() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     struct TestStruct<'a> {
//         name: String,
//         surname: &'a str,
//     }
//     let a = TestStruct {
//         name: "John".to_string(),
//         surname: "Doe",
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("name"), &str, "John", buf);
//     check_field_value!(name!("surname"), &str, "Doe", buf);
// }

// #[test]
// fn check_flat_message_buffer_safe() {
//     #[derive(Debug, PartialEq, FlatMessage)]
//     struct TestStruct<'a> {
//         name: String,
//         surname: &'a str,
//         math: u8,
//         engligh: u8,
//         passed: bool,
//         average: f64,
//     }
//     let a = TestStruct {
//         name: "John".to_string(),
//         surname: "Doe",
//         math: 100,
//         engligh: 90,
//         passed: true,
//         average: 95.0,
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value!(name!("name"), &str, "John", buf);
//     check_field_value!(name!("surname"), &str, "Doe", buf);
//     check_field_value!(name!("math"), u8, 100, buf);
//     check_field_value!(name!("engligh"), u8, 90, buf);
//     check_field_value!(name!("passed"), bool, true, buf);
//     check_field_value!(name!("average"), f64, 95.0, buf);
// }

// #[test]
// fn check_flat_message_buffer_unsafe() {
//     #[derive(Debug, PartialEq, FlatMessage)]
//     struct TestStruct<'a> {
//         name: String,
//         surname: &'a str,
//         math: u8,
//         engligh: u8,
//         passed: bool,
//         average: f64,
//     }
//     let a = TestStruct {
//         name: "John".to_string(),
//         surname: "Doe",
//         math: 100,
//         engligh: 90,
//         passed: true,
//         average: 95.0,
//     };
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let buf = FlatMessageBuffer::try_from(&output).unwrap();
//     check_field_value_unsafe!(name!("name"), &str, "John", buf);
//     check_field_value_unsafe!(name!("surname"), &str, "Doe", buf);
//     check_field_value_unsafe!(name!("math"), u8, 100, buf);
//     check_field_value_unsafe!(name!("engligh"), u8, 90, buf);
//     check_field_value_unsafe!(name!("passed"), bool, true, buf);
//     check_field_value_unsafe!(name!("average"), f64, 95.0, buf);
// }

// // #[test]
// // fn check_serde_full() {
// //     #[derive(Debug, PartialEq, FlatMessage)]
// //     struct TestStruct<'a> {
// //         name: String,
// //         surname: &'a str,
// //         math: u8,
// //         engligh: u8,
// //         passed: bool,
// //         average: f64,
// //     }
// //     let mut a = TestStruct {
// //         name: "John".to_string(),
// //         surname: "Doe",
// //         math: 100,
// //         engligh: 90,
// //         passed: true,
// //         average: 95.0,
// //     };
// //     a.update_metada(MetaDataBuilder::new()
// //         .timestamp(123456)
// //         .unique_id(654321)
// //         .build());
// //     let mut output = Storage::default();
// //     a.serialize_to(&mut output, Config::default()).unwrap();
// //     let b = TestStruct::deserialize_from(&output).unwrap();
// //     assert_eq!(a.name, b.name);
// //     assert_eq!(a.surname, b.surname);
// //     assert_eq!(a.math, b.math);
// //     assert_eq!(a.engligh, b.engligh);
// //     assert_eq!(a.passed, b.passed);
// //     assert_eq!(a.average, b.average);
// //     assert_eq!(a.metadata().timestamp(), b.metadata().timestamp());
// //     assert_eq!(a.metadata().unique_id(), b.metadata().unique_id());
// // }

// // #[test]
// // fn check_serde_into_smaller_struct() {
// //     #[derive(Debug, PartialEq, FlatMessage)]
// //     struct TestStruct<'a> {
// //         name: String,
// //         surname: &'a str,
// //         math: u8,
// //         engligh: u8,
// //         passed: bool,
// //         average: f64,
// //     }

// //     #[derive(Debug, PartialEq, FlatMessage)]
// //     #[flat_message_options(metadata = false)]
// //     struct TestSmallerStruct {
// //         name: String,
// //         math: u8,
// //         engligh: u8,
// //         average: f64,
// //     }

// //     let mut a = TestStruct {
// //         name: "John".to_string(),
// //         surname: "Doe",
// //         math: 100,
// //         engligh: 90,
// //         passed: true,
// //         average: 95.0,
// //     };
// //     a.update_metada(MetaDataBuilder::new()
// //         .timestamp(123456)
// //         .unique_id(654321)
// //         .build());
// //     let mut output = Storage::default();
// //     a.serialize_to(&mut output, Config::default()).unwrap();
// //     let b = TestSmallerStruct::deserialize_from(&output).unwrap();
// //     assert_eq!(a.name, b.name);
// //     assert_eq!(a.math, b.math);
// //     assert_eq!(a.engligh, b.engligh);
// //     assert_eq!(a.average, b.average);
// // }

// // #[test]
// // fn check_serde_into_different_struct() {
// //     #[derive(Debug, PartialEq, FlatMessage)]
// //     struct TestStruct<'a> {
// //         name: String,
// //         surname: &'a str,
// //         math: u8,
// //         engligh: u8,
// //         passed: bool,
// //         average: f64,
// //     }

// //     #[derive(Debug, PartialEq, FlatMessage)]
// //     #[flat_message_options(metadata = false)]
// //     struct TestSmallerStruct {
// //         a: u8,
// //         b: u16,
// //         math: u16,
// //     }

// //     let mut a = TestStruct {
// //         name: "John".to_string(),
// //         surname: "Doe",
// //         math: 100,
// //         engligh: 90,
// //         passed: true,
// //         average: 95.0,
// //     };
// //     a.update_metada(MetaDataBuilder::new()
// //         .timestamp(123456)
// //         .unique_id(654321)
// //         .build());
// //     let mut output = Storage::default();
// //     a.serialize_to(&mut output, Config::default()).unwrap();
// //     let b = TestSmallerStruct::deserialize_from(&output);
// //     assert_eq!(b.is_err(), true);
// // }

// // #[test]
// // fn check_serde_into_different_type() {
// //     #[derive(Debug, PartialEq, FlatMessage)]
// //     struct TestStruct<'a> {
// //         name: String,
// //         surname: &'a str,
// //         math: u8,
// //         engligh: u8,
// //         passed: bool,
// //         average: f64,
// //     }

// //     #[derive(Debug, PartialEq, FlatMessage)]
// //     struct TestStruct2<'a> {
// //         name: String,
// //         surname: &'a str,
// //         math: u8,
// //         engligh: u16, // english is not the same type
// //         passed: bool,
// //         average: f64,
// //     }

// //     let mut a = TestStruct {
// //         name: "John".to_string(),
// //         surname: "Doe",
// //         math: 100,
// //         engligh: 90,
// //         passed: true,
// //         average: 95.0,
// //     };
// //     a.update_metada(MetaDataBuilder::new()
// //         .timestamp(123456)
// //         .unique_id(654321)
// //         .build());
// //     let mut output = Storage::default();
// //     a.serialize_to(&mut output, Config::default()).unwrap();
// //     let b = TestStruct2::deserialize_from(&output);
// //     assert_eq!(b.is_err(), true);
// // }

// #[test]
// fn check_serde_full_unchecked() {
//     #[derive(Debug, PartialEq, FlatMessage)]
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
//     a.update_metada(MetaDataBuilder::new()
//         .timestamp(123456)
//         .unique_id(654321)
//         .build());
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let b = unsafe { TestStruct::deserialize_from_unchecked(&output).unwrap() };
//     assert_eq!(a.name, b.name);
//     assert_eq!(a.surname, b.surname);
//     assert_eq!(a.math, b.math);
//     assert_eq!(a.engligh, b.engligh);
//     assert_eq!(a.passed, b.passed);
//     assert_eq!(a.average, b.average);
//     assert_eq!(a.metadata().timestamp(), b.metadata().timestamp());
//     assert_eq!(a.metadata().unique_id(), b.metadata().unique_id());
// }

// #[test]
// fn check_structure_information() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(version = 12)]
//     struct TestStruct {
//         a: u64,
//         b: u32,
//     }
//     let mut a = TestStruct {
//         a: 12,
//         b: 34,
//     };
//     a.update_metada(MetaDataBuilder::new()
//         .timestamp(123456)
//         .unique_id(654321)
//         .build());
//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let si = StructureInformation::try_from(&output).unwrap();
//     assert_eq!(si.timestamp(), Some(123456));
//     assert_eq!(si.unique_id(), Some(654321));
//     assert_eq!(si.version(), Some(12));
//     assert_eq!(si.name(), Some(name!("TestStruct")));
// }

// #[test]
// fn check_structure_information_with_match() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(metadata = false)]
//     struct TestStruct {
//         a: u64,
//     }
//     let a = TestStruct { a: 12 };

//     let mut output = Storage::default();
//     a.serialize_to(&mut output, Config::default()).unwrap();
//     let si = StructureInformation::try_from(&output).unwrap();
//     assert_eq!(si.timestamp(), None);
//     assert_eq!(si.unique_id(), None);
//     assert_eq!(si.version(), None);
//     if let Some(name) = si.name() {
//         match name {
//             name!("TestStruct") => {}
//             name!("TestStruct2") => panic!("Invalid name"),
//             _ => panic!("Invalid name"),
//         }
//     }
// }

// #[test]
// fn check_serde_name_validation() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(metadata = false, validate_name = true)]
//     struct TestStruct1 {
//         value: u64,
//     }
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(metadata = false)]
//     struct TestStruct2 {
//         value: u64,
//     }
//     let a_1 = TestStruct1 { value: 12 };
//     let a_2 = TestStruct2 { value: 24 };

//     let mut output_1 = Storage::default();
//     let mut output_2 = Storage::default();
//     a_1.serialize_to(&mut output_1, Config::default()).unwrap();
//     a_2.serialize_to(&mut output_2, Config::default()).unwrap();

//     // from TestStruct1 to TestStruct1
//     let b = TestStruct1::deserialize_from(&output_1).unwrap();
//     assert_eq!(a_1.value, b.value);

//     // from TestStruct1 to TestStruct2 (no validation name required -> should be possible)
//     let b = TestStruct2::deserialize_from(&output_1).unwrap();
//     assert_eq!(a_1.value, b.value);

//     // from TestStruct2 to TestStruct1 (validation name required -> should not be possible)
//     let b = TestStruct1::deserialize_from(&output_2);
//     assert_eq!(b.is_err(), true);

//     // from TestStruct2 to TestStruct2
//     let b = TestStruct2::deserialize_from(&output_2).unwrap();
//     assert_eq!(a_2.value, b.value);
// }

// #[test]
// fn check_clone() {
//     #[derive(Clone, Debug, Eq, PartialEq, FlatMessage)]
//     struct TestStruct {
//         a: String,
//         b: String,
//     }
//     let mut v1 = TestStruct {
//         a: "Hello".to_string(),
//         b: "World".to_string(),
//     };
//     v1.update_metada(MetaDataBuilder::new().timestamp(1).unique_id(2).build());
//     let v2 = v1.clone();
//     assert_eq!(v1.a, v2.a);
//     assert_eq!(v1.b, v2.b);
//     assert_eq!(v1.metadata(), v2.metadata());
//     assert_eq!(v1, v2);
//     let mut storage = Storage::default();
//     v1.serialize_to(&mut storage, Config::default()).unwrap();
//     let v3 = TestStruct::deserialize_from(&storage).unwrap();
//     assert_eq!(v1, v3);
// }

// #[test]
// fn check_serialization_checksum() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(checksum = true, store_name = false, metadata = false)]
//     struct TestStruct1 {
//         value: u32,
//     }
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(metadata = false, store_name = false)]
//     struct TestStruct2 {
//         value: u32,
//     }
//     let v1 = TestStruct1 { value: 123456 };
//     let v2 = TestStruct2 { value: 123456 };
//     let mut storage = Storage::default();
//     v1.serialize_to(&mut storage, Config::default()).unwrap();
//     let expected_output = vec![
//         71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
//     ];
//     assert_eq!(storage.as_slice(), expected_output);
//     let len_v1 = storage.len();
//     v2.serialize_to(&mut storage, Config::default()).unwrap();
//     let expected_output = vec![71, 84, 72, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8];
//     assert_eq!(storage.as_slice(), expected_output);
//     let len_v2 = storage.len();
//     // TestStruct1 has 4 bytes more than TestStruct2 (for the checksum)
//     assert_eq!(len_v1, len_v2 + 4);
// }

// #[test]
// fn check_serde_with_checksum() {
//     #[derive(Debug, PartialEq, FlatMessage)]
//     #[flat_message_options(checksum = true, store_name = false, metadata = false)]
//     struct TestStruct<'a> {
//         value: u32,
//         b: bool,
//         name: String,
//         surname: &'a str,
//         age: i32,
//     }
//     let s = TestStruct {
//         value: 123456,
//         b: true,
//         name: "John".to_string(),
//         surname: "Doe",
//         age: 30,
//     };
//     let mut storage = Storage::default();
//     s.serialize_to(&mut storage, Config::default()).unwrap();
//     let ds = TestStruct::deserialize_from(&storage).unwrap();
//     assert_eq!(s.age, ds.age);
//     assert_eq!(s.b, ds.b);
//     assert_eq!(s.value, ds.value);
//     assert_eq!(s.name, ds.name);
//     assert_eq!(s.surname, ds.surname);
// }

// #[test]
// fn check_deserialization_checksum_always() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(checksum = true, store_name = false, metadata = false, validate_checksum = always)]
//     struct TestStruct {
//         value: u32,
//     }
//     let buffer = Storage::from_buffer(&[
//         71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
//     ]);
//     let v = TestStruct::deserialize_from(&buffer).unwrap();
//     assert_eq!(v.value, 123456);
//     let buffer = Storage::from_buffer(&[
//         71, 84, 72, 1, 1, 0, 0, 4, 255, 255, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
//     ]);
//     let v = TestStruct::deserialize_from(&buffer);
//     match v.err() {
//         Some(flat_message::Error::InvalidChecksum(_)) => {}
//         _ => panic!("Invalid error - expected InvalidChecksum"),
//     }
// }

// #[test]
// fn check_deserialization_checksum_auto() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(checksum = true, store_name = false, metadata = false)]
//     struct TestStruct {
//         value: u32,
//     }
//     // valid checksum
//     let buffer = Storage::from_buffer(&[
//         71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
//     ]);
//     let v = TestStruct::deserialize_from(&buffer).unwrap();
//     assert_eq!(v.value, 123456);
//     // invalid checksum
//     let buffer = Storage::from_buffer(&[
//         71, 84, 72, 1, 1, 0, 0, 4, 255, 255, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
//     ]);
//     let v = TestStruct::deserialize_from(&buffer);
//     match v.err() {
//         Some(flat_message::Error::InvalidChecksum(_)) => {}
//         _ => panic!("Invalid error - expected InvalidChecksum"),
//     }
//     // checksum is missing
//     let buffer =
//         Storage::from_buffer(&[71, 84, 72, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
//     let v = TestStruct::deserialize_from(&buffer).unwrap();
//     assert_eq!(v.value, 123456);
// }

// #[test]
// fn check_deserialization_checksum_ignore() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(checksum = true, store_name = false, metadata = false, validate_checksum = ignore)]
//     struct TestStruct {
//         value: u32,
//     }
//     // valid checksum
//     let buffer = Storage::from_buffer(&[
//         71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
//     ]);
//     let v = TestStruct::deserialize_from(&buffer).unwrap();
//     assert_eq!(v.value, 123456);
//     // invalid checksum (deserialization should still happen)
//     let buffer = Storage::from_buffer(&[
//         71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 255, 255, 255, 255,
//     ]);
//     let v = TestStruct::deserialize_from(&buffer).unwrap();
//     assert_eq!(v.value, 123456);
//     // checksum is missing
//     let buffer =
//         Storage::from_buffer(&[71, 84, 72, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
//     let v = TestStruct::deserialize_from(&buffer).unwrap();
//     assert_eq!(v.value, 123456);
// }

// #[test]
// fn check_deserialization_checksum_unchecked_always() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(checksum = true, store_name = false, metadata = false, validate_checksum = always)]
//     struct TestStruct {
//         value: u32,
//     }
//     // valid checksum
//     let buffer = Storage::from_buffer(&[
//         71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 149, 163, 180, 132,
//     ]);
//     let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
//     assert_eq!(v.value, 123456);
//     // invalid checksum (deserialization should still happen)
//     let buffer = Storage::from_buffer(&[
//         71, 84, 72, 1, 1, 0, 0, 4, 64, 226, 1, 0, 3, 211, 94, 66, 8, 255, 255, 255, 255,
//     ]);
//     let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
//     assert_eq!(v.value, 123456);
//     // checksum is missing (deserialization should still happen)
//     let buffer =
//         Storage::from_buffer(&[71, 84, 72, 1, 1, 0, 0, 0, 64, 226, 1, 0, 3, 211, 94, 66, 8]);
//     let v = unsafe { TestStruct::deserialize_from_unchecked(&buffer).unwrap() };
//     assert_eq!(v.value, 123456);
// }

// #[test]
// fn check_serde_128_bits_alignament() {
//     #[derive(Debug, PartialEq, Eq, FlatMessage)]
//     #[flat_message_options(metadata = false, store_name = false)]
//     struct TestStruct {
//         b6: Vec<u128>,
//         b4: Vec<u64>,
//         b5: Vec<u32>,
//         name: String,
//     }
//     let mut v = Storage::default();
//     let s = TestStruct {
//         b6: [1, 2, 3].to_vec(),
//         b4: [10, 20].to_vec(),
//         b5: [40, 41, 42, 43].to_vec(),
//         name: "Hello".to_string(),
//     };
//     s.serialize_to(&mut v, Config::default()).unwrap();
//     let expected = &[
//         71, 84, 72, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 10,
//         0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 40, 0, 0, 0, 41, 0, 0, 0, 42, 0,
//         0, 0, 43, 0, 0, 0, 5, 72, 101, 108, 108, 111, 0, 0, 131, 30, 44, 136, 132, 32, 44, 137,
//         133, 35, 44, 139, 14, 189, 57, 141, 104, 80, 16, 124,
//     ];
//     assert_eq!(v.as_slice(), expected);
// }

#[test]
fn check_max_size_for_serialization() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    struct TestStruct {
        value: u32,
    }
    let mut v = Storage::default();
    let s = TestStruct { value: 123456 };
    let result = s.serialize_to(&mut v, Config::default());
    assert!(result.is_ok());
    let result = s.serialize_to(&mut v, ConfigBuilder::new().max_size(4).build());
    assert!(result.is_err());
    match result.err() {
        Some(flat_message::Error::ExceedMaxSize(_)) => {}
        _ => panic!("Invalid error - expected MaxSizeExceeded"),
    }
}

#[test]
fn check_simple_struct_width_comments() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Point {
        // x coordinate
        x: i32,
        // y coordinate
        y: i32,
    }
    validate_correct_serde(Point { x: 10, y: 20 });
}

#[test]
fn check_simple_struct_width_documentation() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name = false)]
    struct Point {
        /// x coordinate that is used to store the position
        /// in the 2D space
        x: i32,
        /// y coordinate that is used to store the position
        /// in the 2D space
        y: i32,
    }
    validate_correct_serde(Point { x: 10, y: 20 });
}

#[test]
fn check_task_example() {
    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u8)]
    enum Priority {
        Low = 1,
        Medium = 2,
        High = 3,
    }

    #[derive(FlatMessage, Debug, PartialEq)]
    #[flat_message_options(version = 1, store_name = true, checksum = true)]
    struct Task {
        title: String,
        description: Option<String>,
        completed: bool,

        #[flat_message_item(repr = u8, kind = enum)]
        priority: Priority,

        tags: Vec<String>,
    }

    let task = Task {
        title: "Learn FlatMessage".to_string(),
        description: Some("Read the documentation".to_string()),
        completed: false,
        priority: Priority::High,
        tags: vec!["learning".to_string(), "rust".to_string()],
    };

    // Create a serialization storage buffer
    let mut storage = Storage::default();
    if let Err(e) = task.serialize_to(&mut storage, Config::default()) {
        panic!("Error serializing task: {}", e);
    }

    // print the buffer
    println!("Buffer: {:?}", storage.as_slice());

    // Deserialize from buffer
    match Task::deserialize_from(&storage) {
        Ok(restored_task) => {
            assert_eq!(task, restored_task);
            println!("Task serialized and deserialized successfully");
        }
        Err(e) => {
            panic!("Error deserializing task: {}", e);
        }
    }
}

#[test]
fn check_config_max_size() {
    #[derive(FlatMessage)]
    #[flat_message_options(optimized_unchecked_code = false)]
    struct Data {
        content: Vec<u8>,
    }

    let data = Data {
        content: vec![1, 2, 3],
    };
    let mut storage = Storage::default();
    let config = ConfigBuilder::new().max_size(10).build();
    let result = data.serialize_to(&mut storage, config);
    assert_eq!(result, Err(flat_message::Error::ExceedMaxSize((21, 10))));
}



