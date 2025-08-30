use flat_message::*;

#[test]
fn check_packed_struct() {
    #[derive(Debug, PartialEq, Eq, FlatMessagePacked)]
    struct Point {
        value: u32,
        x: i32,
        y: u32,
        d: Vec<u32>,
        name: String,
    }

    // #[derive(Debug, PartialEq, Eq, FlatMessage)]
    // struct Test<'a> {
    //     #[flat_message_item(kind = pod, align = 4)]
    //     point: Point,
    //     x: &'a str,
    // }
    // let test_struct = Test {
    //     point: Point {
    //         value: 200,
    //         x: 10,
    //         y: 20,
    //     },
    //     x: "Hello",
    // };
    // let mut storage = Storage::default();
    // test_struct
    //     .serialize_to(&mut storage, Config::default())
    //     .unwrap();
    // let test_struct2 = Test::deserialize_from(&storage).unwrap();
    // assert_eq!(test_struct, test_struct2);
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
