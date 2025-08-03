// use flat_message::*;

// #[test]
// fn check_serde_version_compatibility_check() {
//     mod v1 {
//         use flat_message::*;
//         #[derive(Debug, PartialEq, Eq, FlatMessage)]
//         #[flat_message_options(version = 1, compatible_versions = "1")]
//         pub struct TestStruct {
//             pub value: u64,
//         }
//     }
//     mod v2 {
//         use flat_message::*;
//         #[derive(Debug, PartialEq, Eq, FlatMessage)]
//         #[flat_message_options(version = 2, compatible_versions = "1,2")]
//         pub struct TestStruct {
//             pub value: u64,
//         }
//     }
//     mod v3 {
//         use flat_message::*;
//         #[derive(Debug, PartialEq, Eq, FlatMessage)]
//         #[flat_message_options(version = 3, compatible_versions = "<3")]
//         pub struct TestStruct {
//             pub value: u64,
//         }
//     }
//     let mut o1 = Storage::default();
//     let mut o2 = Storage::default();
//     let mut o3 = Storage::default();
//     {
//         let mut v3_struct = v3::TestStruct { value: 3 };
//         v3_struct.update_metada(MetaDataBuilder::new().timestamp(333).unique_id(33).build());
//         v3_struct.serialize_to(&mut o3, Config::default()).unwrap();
//     }
//     {
//         let mut v2_struct = v2::TestStruct { value: 2 };
//         v2_struct.update_metada(MetaDataBuilder::new().timestamp(222).unique_id(22).build());
//         v2_struct.serialize_to(&mut o2, Config::default()).unwrap();
//     }
//     {
//         let mut v1_struct = v1::TestStruct { value: 1 };
//         v1_struct.update_metada(MetaDataBuilder::new().timestamp(111).unique_id(11).build());
//         v1_struct.serialize_to(&mut o1, Config::default()).unwrap();
//     }
//     let v1_from_v3 = v1::TestStruct::deserialize_from(&o3);
//     let v1_from_v2 = v1::TestStruct::deserialize_from(&o2);
//     let v2_from_v3 = v2::TestStruct::deserialize_from(&o3);
//     let v3_from_v1 = v3::TestStruct::deserialize_from(&o1);
//     let v3_from_v2 = v3::TestStruct::deserialize_from(&o2);
//     let v2_from_v1 = v2::TestStruct::deserialize_from(&o1);
//     assert_eq!(
//         v1_from_v2.err(),
//         Some(flat_message::Error::IncompatibleVersion(2))
//     );
//     assert_eq!(
//         v1_from_v3.err(),
//         Some(flat_message::Error::IncompatibleVersion(3))
//     );
//     assert_eq!(
//         v2_from_v3.err(),
//         Some(flat_message::Error::IncompatibleVersion(3))
//     );
//     assert_eq!(v3_from_v1.unwrap().value, 1);
//     assert_eq!(v3_from_v2.unwrap().value, 2);
//     assert_eq!(v2_from_v1.unwrap().value, 1);
// }