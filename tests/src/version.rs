use flat_message::*;

mod scenario_1 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1, compatible_versions = "1")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2, compatible_versions = "1,2")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
    pub mod v3 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 3, compatible_versions = "<3")]
        pub struct TestStruct {
            pub value: u64,
        }
    }
}

mod scenario_2 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1, compatible_versions = "1")]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2, compatible_versions = "1,2")]
        pub struct TestStruct {
            pub value: u8,
            pub value2: u16, // new mandatory field added
        }
    }
}

mod scenario_3 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            pub value2: u16, // new mandatory field added
        }
    }
}

mod scenario_4 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(mandatory = false, default = 3)]
            pub value2: u16, // new optional field added
        }
    }
}

mod scenario_5 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            pub value2: Option<u16>, // new optional field added
        }
    }
}

mod scenario_6 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(mandatory = false, default = 3)]
            pub value2: Option<u16>, // new optional field added
        }
    }
}

mod scenario_7 {
    pub mod v1 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 1)]
        pub struct TestStruct {
            pub value: u8,
        }
    }
    pub mod v2 {
        use flat_message::*;
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(version = 2)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(mandatory = true)]
            pub value2: Option<u16>, // new mandatory field
        }
    }
}

mod scenario_1_enum {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub color: Color,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellow = 200,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub color: Color,
        }
    } 
}

mod scenario_2_enum {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum, validate = strict)]
            pub color: Option<Color>,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellow = 200,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true)]
            pub color: Option<Color>,
        }
    } 
}

mod scenario_3_enum {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum, validate = fallback)]
            pub color: Color,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum Color {
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellow = 200,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub color: Color,
        }
    } 
}

mod scenario_4_enum {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum)]
            pub color: Option<Color>,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug, Default)]
        #[repr(u8)]
        pub enum Color {
            #[default]
            Red = 1,
            Green = 10,
            Blue = 100,
            Yellow = 200,
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = enum, mandatory = true)]
            pub color: Option<Color>,
        }
    } 
}


mod scenario_1_flags {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B)]
        pub struct Flags(u8);
        impl Flags {
            add_flag!(A = 1);
            add_flag!(B = 2);
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = flags)]
            pub flags: Flags,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Flags(u8);
        impl Flags {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = flags)]
            pub flags: Flags,
        }
    } 
}


mod scenario_3_flags {
    pub mod v1 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B)]
        pub struct Flags(u8);
        impl Flags {
            add_flag!(A = 1);
            add_flag!(B = 2);
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = flags, validate = fallback)]
            pub flags: Flags,
        }
    } 
    pub mod v2 {
        use flat_message::*;
        #[derive(Copy, Clone, FlatMessageFlags, PartialEq, Eq, Debug, Default)]
        #[repr(transparent)]
        #[flags(A,B,C)]
        pub struct Flags(u8);
        impl Flags {
            add_flag!(A = 1);
            add_flag!(B = 2);
            add_flag!(C = 4);
        }
    
        #[derive(Debug, PartialEq, Eq, FlatMessage)]
        #[flat_message_options(store_name = false)]
        pub struct TestStruct {
            pub value: u8,
            #[flat_message_item(repr = u8, kind = flags)]
            pub flags: Flags,
        }
    } 
}

#[test]
fn check_serde_version_compatibility_check() {
    use scenario_1::{v1, v2, v3};
    let mut o1 = Storage::default();
    let mut o2 = Storage::default();
    let mut o3 = Storage::default();
    {
        let v3_struct = v3::TestStruct { value: 3u64 };
        v3_struct.serialize_to(&mut o3, Config::default()).unwrap();
    }
    {
        let v2_struct = v2::TestStruct { value: 2u64 };
        v2_struct.serialize_to(&mut o2, Config::default()).unwrap();
    }
    {
        let v1_struct = v1::TestStruct { value: 1 };
        v1_struct.serialize_to(&mut o1, Config::default()).unwrap();
    }
    let v1_from_v3 = v1::TestStruct::deserialize_from(&o3);
    let v1_from_v2 = v1::TestStruct::deserialize_from(&o2);
    let v2_from_v3 = v2::TestStruct::deserialize_from(&o3);
    let v3_from_v1 = v3::TestStruct::deserialize_from(&o1);
    let v3_from_v2 = v3::TestStruct::deserialize_from(&o2);
    let v2_from_v1 = v2::TestStruct::deserialize_from(&o1);
    assert_eq!(
        v1_from_v2.err(),
        Some(flat_message::Error::IncompatibleVersion(2))
    );
    assert_eq!(
        v1_from_v3.err(),
        Some(flat_message::Error::IncompatibleVersion(3))
    );
    assert_eq!(
        v2_from_v3.err(),
        Some(flat_message::Error::IncompatibleVersion(3))
    );
    assert_eq!(v3_from_v1.unwrap().value, 1);
    assert_eq!(v3_from_v2.unwrap().value, 2);
    assert_eq!(v2_from_v1.unwrap().value, 1);
}

#[test]
fn check_version_buffer() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name: false, version = 11)]
    struct Test {
        x: i8,
    }
    let t = Test { x: 1 };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    //println!("{:?}", v.as_slice());
    assert_eq!(
        v.as_slice(),
        &[
            70, 76, 77, 1, 1, 0, 11, 0, // Header
            1, // x
            0, 0, 0, // alignament padding
            6, 80, 12, 253, // hash for x
            8,   // offset for x
        ]
    );
}

#[test]
fn check_version_from_structure_info() {
    #[derive(FlatMessage)]
    #[flat_message_options(store_name: false, version = 11)]
    struct Test {
        x: i8,
    }
    let t = Test { x: 1 };
    let mut v = Storage::default();
    t.serialize_to(&mut v, Config::default()).unwrap();
    let si = StructureInformation::try_from(&v).unwrap();
    assert_eq!(si.version(), Some(11));
}

#[test]
fn check_v1_to_v2_scenario_2_using_compatible_versions() {
    use scenario_2::*;
    // v1 to v2 for scenario 2 should fail even if v2 has compatible_versions = "1,2"
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // v2 contsins a mandatory field "value2" that is not present in v1 -> Error::MissingField
    assert!(result.is_err());
    //println!("{:?}", result);
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_2_using_compatible_versions() {
    use scenario_2::*;
    // v2 to v1 for scenario 2 should work correctly (v1 only needs the field 'value' from v2)
    // however, this deserialization will fail as v1 only accepts the version "1" (from check_v1_to_v2_scenario_2)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: 2 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    //println!("{:?}", result);
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::IncompatibleVersion(2))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_3_not_using_compatible_versions() {
    use scenario_3::*;
    // v1 to v2 for scenario 3 should fail base valuef2 is mandatory
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // v2 contsins a mandatory field "value2" that is not present in v1 -> Error::MissingField
    assert!(result.is_err());
    //println!("{:?}", result);
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_3_not_using_compatible_versions() {
    use scenario_3::*;
    // v2 to v1 for scenario 3 should work correctly (v1 only needs the field 'value' from v2)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: 2 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());    
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}


#[test]
fn check_v1_to_v2_scenario_4_not_using_compatible_versions_with_mandatory_false() {
    use scenario_4::*;
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.value2, 3);
}
#[test]
fn check_v2_to_v1_scenario_4_not_using_compatible_versions_with_mandatory_false() {
    use scenario_4::*;
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: 2 };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());    
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}

#[test]
fn check_v2_to_v1_scenario_5_not_using_compatible_versions_with_option_field_without_mandatory_false() {
    use scenario_5::*;
    // v2 to v1 for scenario 5 should work correctly (v1 only needs the field 'value' from v2)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: Some(2) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}

#[test]
fn check_v1_to_v2_scenario_5_not_using_compatible_versions_with_option_field_without_mandatory_false() {
    use scenario_5::*;
    // v1 to v2 for scenario 5 should work correctly (v2 only needs the field 'value' from v1)
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    // By default, Option<T> is marked with mandatory = false, so it will be defaulted to None if it is not present
    // so the deserialization should be successful
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.value2, None);
}

#[test]
fn check_v2_to_v1_scenario_6_not_using_compatible_versions_with_option_field_with_mandatory_false_field() {
    use scenario_6::*;
    // v2 to v1 for scenario 6 should work correctly (v1 only needs the field 'value' from v2)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: Some(2) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}

#[test]
fn check_v1_to_v2_scenario_6_not_using_compatible_versions_with_option_field_with_mandatory_false_field() {
    use scenario_6::*;
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.value2, Some(3));
}

#[test]
fn check_v2_to_v1_scenario_7_not_using_compatible_versions_with_a_mandatory_option_field() {
    use scenario_7::*;
    // v2 to v1 for scenario 7 the code will work as `v1` does not require the field value2
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, value2: Some(2) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
}

#[test]
fn check_v1_to_v2_scenario_7_not_using_compatible_versions_with_a_mandatory_option_field() {
    use scenario_7::*;
    // v1 to v2 for scenario 7 the code will fail because the field value2 is mandatory
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1 };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FieldIsMissing(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_1_enum() {
    use scenario_1_enum::*;
    // v1 to v2 for scenario 1_enum the code will work as `v1::Color` is compatible with `v2::Color`
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1, color: v1::Color::Green };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.color, v2::Color::Green);
}

#[test]
fn check_v2_to_v1_scenario_1_enum_without_yellow() {
    use scenario_1_enum::*;
    // v2 to v1 for scenario 1_enum the code will work as `v2::Color` uses a value that is not present in `v1::Color`
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: v2::Color::Green };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.color, v1::Color::Green);
}

#[test]
fn check_v2_to_v1_scenario_1_enum_with_yellow() {
    use scenario_1_enum::*;
    // v2 to v1 for scenario 1_enum the code will fail becase there is no variant `Yellow` in `v1::Color`
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: v2::Color::Yellow };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v1_to_v2_scenario_2_enum() {
    use scenario_2_enum::*;
    // v1 to v2 for scenario 2_enum the code will work as `v1::Color` is compatible with `v2::Color`
    // color is Option<Color> and it is maarked as mandatory
    let mut storage = Storage::default();
    let d_v1 = v1::TestStruct { value: 1, color: Some(v1::Color::Green) };
    d_v1.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v2::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v2 = result.unwrap();
    assert_eq!(d_v2.value, 1);
    assert_eq!(d_v2.color, Some(v2::Color::Green));
}

#[test]
fn check_v2_to_v1_scenario_2_enum_without_yellow() {
    use scenario_2_enum::*;
    // v2 to v1 for scenario 2_enum the code will work as `v2::Color` uses a value that is not present in `v1::Color`
    // color is Option<Color> and it is maarked as mandatory
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: Some(v2::Color::Green) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.color, Some(v1::Color::Green));
}

#[test]
fn check_v2_to_v1_scenario_2_enum_with_yellow() {
    use scenario_2_enum::*;
    // v2 to v1 for scenario 2_enum the code will fail becase there is no variant `Yellow` in `v1::Color`
    // color is Option<Color> and it has validate = true
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: Some(v2::Color::Yellow) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_3_enum_with_yellow() {
    use scenario_3_enum::*;
    // v2 to v1 for scenario 3_enum the code will not fail becase v1 has the field color with validate = fallback (so if the deserialization fails, the default will be applied)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: v2::Color::Yellow };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let d_v1 = v1::TestStruct::deserialize_from(&mut storage).unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.color, v1::Color::Red); // Red is the default color
}

#[test]
fn check_v2_to_v1_scenario_4_enum_with_yellow() {
    use scenario_4_enum::*;
    // v2 to v1 for scenario 4_enum the code will not fail becase even if v1 does not have the field Yellow, by default for all Option validate is set to fallback and as such it will be defaulted to None
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, color: Some(v2::Color::Yellow) };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let d_v1 = v1::TestStruct::deserialize_from(&mut storage).unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.color, None); // None is the default for Option<T>
}

#[test]
fn check_v2_to_v1_scenario_1_flags_without_c() {
    use scenario_1_flags::*;
    // v2 to v1 for scenario 1 flags - will work because A and B flags are also present in v1 (and as such there is no compatibility issue)
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, flags: v2::Flags::A | v2::Flags::B };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert_eq!(d_v1.flags, v1::Flags::A | v1::Flags::B);
}

#[test]
fn check_v2_to_v1_scenario_1_flags_with_c() {
    use scenario_1_flags::*;
    // v2 to v1 for scenario 1 flags - will fail because v2 is set with C and C is not recognized in v1
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, flags: v2::Flags::C };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_err());
    assert_eq!(
        matches!(result.err(), Some(flat_message::Error::FailToDeserialize(_))),
        true
    );
}

#[test]
fn check_v2_to_v1_scenario_3_flags_with_c() {
    use scenario_3_flags::*;
    // v2 to v1 for scenario 3 flags - will work - there is no flag C in v1::Flags, however the validate is set to fallback and as such the default value (in this case no flags) will be set
    let mut storage = Storage::default();
    let d_v2 = v2::TestStruct { value: 1, flags: v2::Flags::C  | v2::Flags::B };
    d_v2.serialize_to(&mut storage, Config::default()).unwrap();
    let result = v1::TestStruct::deserialize_from(&mut storage);
    assert!(result.is_ok());
    let d_v1 = result.unwrap();
    assert_eq!(d_v1.value, 1);
    assert!(d_v1.flags.is_empty()); // default value for flags
}