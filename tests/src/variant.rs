use super::*;
use flat_message::*;

#[test]
fn check_align_16() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u128>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 16)]
        v: MyEnum,
    }

    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Byte(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::DWord(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::String("Hello".to_string())});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Vector(vec![1, 2, 3])});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::SimpleVariant});
}

#[test]
fn check_align_16_repr() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u128>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 16)]
        v: MyEnum,
    }

    let t = Test {
        x: 1,
        y: 2,
        v: MyEnum::Byte(3),
    };

    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // header
            0, 0, 0, 0, 0, 0, 0, 0, // alignament to 16 bytes
            220, 228, 14, 121, // hash over the variant MyEnum
            1, 53, 242, 61, // hash over the variant Byte (1 = u8)
            3,  // value of the variant Byte (3)
            2, 0, // y = 2
            1, // x = 1
            39, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            16,  // offset fro Test::v (offset 16)
            25,  // offset for Test::y (offset 25)
            27,  // offset for Test::x (offset 27)
        ]
    );
}

#[test]
fn check_align_1() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u8>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyEnum,
    }

    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Byte(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::DWord(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::String("Hello".to_string())});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Vector(vec![1, 2, 3])});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::SimpleVariant});
}

#[test]
fn check_align_2() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u16>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 2)]
        v: MyEnum,
    }

    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Byte(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::DWord(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::String("Hello".to_string())});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Vector(vec![1, 2, 3])});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::SimpleVariant});
}

#[test]
fn check_align_4() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u32>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 4)]
        v: MyEnum,
    }

    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Byte(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::DWord(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::String("Hello".to_string())});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Vector(vec![1, 2, 3])});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::SimpleVariant});
}

#[test]
fn check_align_8() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u64>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 8)]
        v: MyEnum,
    }

    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Byte(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::DWord(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::String("Hello".to_string())});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::Vector(vec![1, 2, 3])});
    validate_correct_serde(Test {x: 1, y: 2, v: MyEnum::SimpleVariant});
}

#[test]
fn check_align_4_repr() {
    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyEnum {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u32>),
        SimpleVariant,
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 4)]
        v: MyEnum,
    }

    let t = Test {
        x: 1,
        y: 2,
        v: MyEnum::String("Hello".to_string()),
    };

    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap(); 
    assert_eq!(
        s.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // header
            220, 228, 14, 121, // hash over the variant MyEnum
            14, 183, 18, 153, // hash over the variant String (14 = String)
            5, 72, 101, 108, 108, 111, // value of the variant String ("Hello") size = 5
            2, 0, // Test::y = 2
            1, // Test::x = 1
            0, 0, 0, // padding to 4 bytes
            37, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            8, // offset for Test::v (offset 8)
            22, // offset for Test::y (offset 22)
            24, // offset for Test::x (offset 24)
        ]
    );
}

#[test]
fn check_combo_enum_and_flags() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A, B, C)]
    pub struct Flags(u8);
    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        add_flag!(C = 4);
    }

    #[derive(Copy, Clone, FlatMessageEnum, PartialEq, Eq, Debug)]
    #[repr(u32)]
    enum Metric {
        Km = 1,
        Liters = 10000000,
        Temperature = 1000000000,
        Pressue = 10000,
    }


    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum MyVariant {
        Byte(u8),
        StringValue(String),
        #[flat_message_item(kind = flags, repr = u8)]
        FlagsEntry(Flags),
        #[flat_message_item(kind = enum, repr = u32)]
        MetricEntry(Metric),
    }

    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name: false)]
    struct Test {
        x: u8,
        y: u16,
        #[flat_message_item(kind = variant, align = 1)]
        v: MyVariant,
    }

    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::Byte(3)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::StringValue("Hello".to_string())});
    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::FlagsEntry(Flags::A)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::FlagsEntry(Flags::B | Flags::C)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::FlagsEntry(Flags::A | Flags::B | Flags::C)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::MetricEntry(Metric::Km)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::MetricEntry(Metric::Liters)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::MetricEntry(Metric::Temperature)});
    validate_correct_serde(Test {x: 1, y: 2, v: MyVariant::MetricEntry(Metric::Pressue)});
}