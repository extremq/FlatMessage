use flat_message::*;

#[test]
fn check_simple_serde() {
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
    let t2 = Test::deserialize_from(&s).unwrap();
    assert_eq!(t, t2);
}

#[test]
fn check_simple_serde_repr() {
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
            3, // value of the variant Byte (3)
            2, 0, // y = 2
            1, // x = 1
            39, 64, 12, 243, // hash for Test::v
            2, 78, 12, 252, // hash for Test::y
            1, 80, 12, 253, // hash for Test::x
            16, // offset fro Test::v (offset 16)
            25, // offset for Test::y (offset 25)
            27, // offset for Test::x (offset 27)
        ]
    );
}
