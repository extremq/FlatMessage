use flat_message::*;
use crate::*;

#[test]
fn check_packed_no_alignament_required() {
    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct Point {
        x: i32,
        y: u32,
        label: String,
    }

    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    struct Test {
        #[flat_message_item(kind = packed, align = 1)]
        point: Point,
        x: String,
    }
    validate_correct_serde(Test {
        point: Point {
            x: 10,
            y: 20,
            label: "Test".to_string(),
        },
        x: "Hello".to_string(),
    });
}

// // #[test]
// // fn check_copy_struct_repr() {
// //     #[derive(Debug, PartialEq, Eq, FlatMessagePOD, Copy, Clone)]
// //     #[repr(C, align(4))]
// //     struct Point {
// //         value: u32,
// //         x: i16,
// //         y: u16,
// //     }

// //     #[derive(Debug, PartialEq, Eq, FlatMessage)]
// //     #[flat_message_options(store_name : false)]
// //     struct Test<'a> {
// //         #[flat_message_item(kind = pod, align = 4)]
// //         point: Point,
// //         x: &'a str,
// //     }
// //     let test_struct = Test {
// //         point: Point {
// //             value: 200,
// //             x: 10,
// //             y: 20,
// //         },
// //         x: "Hello",
// //     };
// //     let mut storage = Storage::default();
// //     test_struct
// //         .serialize_to(&mut storage, Config::default())
// //         .unwrap();
// //     assert_eq!(
// //         storage.as_slice(),
// //         &[
// //             70, 76, 77, 1, 2, 0, 0, 0, // Header
// //             32, 220, 100, 118, // hash over the Point stucture
// //             8, // size of Point structure (8 bytes)
// //             0, 0, 0, // padding
// //             200, 0, 0, 0, 10, 0, 20, 0, // Point structure (200, 10, 20)
// //             5, // size of x parameter (5 bytes)
// //             72, 101, 108, 108, 111,  // x parameter - Hello
// //             0, 0, // padding
// //             29, 108, 174, 24, // hash over the x parameter
// //             14, 80, 12, 253, // hash over the point parameter
// //             8, // offset of point
// //             24, // offset of x paramter
// //         ]
// //     );
// // }
