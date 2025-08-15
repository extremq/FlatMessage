use flat_message::*;

#[test]
fn check_simple_serde() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&storage).unwrap();
    assert_eq!(t, t2);
}

#[test]
fn check_simple_serde_repr() {
    #[derive(FlatMessageStruct, Debug, PartialEq, Eq)]
    struct MyDataV1 {
        a: u8,
        b: u32,
        c: u16,
        d: String,
    }
    #[derive(FlatMessage, Debug, PartialEq, Eq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        x: u8,
        #[flat_message_item(align = 4, kind = struct)]
        d: MyDataV1,
        a: u8,
    }
    let t = Test {
        x: 1,
        d: MyDataV1 {
            a: 2,
            b: 3,
            c: 4,
            d: "Hello".to_string(),
        },
        a: 5,
    };
    let mut storage = Storage::default();
    t.serialize_to(&mut storage, Config::default()).unwrap();
    assert_eq!(
        storage.as_slice(),
        &[
            70, 76, 77, 1, 3, 0, 0, 0, // Header for Test structure
            146, 207, 36, 196, // Hash for MyDataV1
            16, 44, 0, 0, // MyDataV1 - > 16 >> 2 = 4 fields, size = 44 octeti
            5, 72, 101, 108, 108, 111, // Test::d::d -> Hello (size 5)
            2, // Test::d::a 
            4, 0, // Test::d::c
            3, 0, 0, 0, // Test::d::b
            0, 0, 0, // Padding
            14, 36, 12, 225, // Hash for Test::d::d (14 = string)
            1, 41, 12, 228, // Hash for Test::d::a (1 = u8)
            2, 44, 12, 230, // Hash for Test::d::c (2 = u16)
            3, 45, 12, 231, // Hash for Test::d::b (3 = u32)
            8, // Offset of Test::d::d 
            14, // Offset of Test::d::a 
            15, // Offset of Test::d::c 
            17, // Offset of Test::d::b 
            5, // Test::a
            1, // Test::x
            0, 0, // Padding
            32, 36, 12, 225, // Hash for Test::d (32 = Struct4)
            1, 41, 12, 228, // Hash for Test::a (1 = u8)
            1, 80, 12, 253, // Hash for Test::x (1 = u8)
            8, // Offset of Test::d
            52, // Offset of Test::a
            53 // // Offset of Test::x
        ]
    );
}
