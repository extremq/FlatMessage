use crate::*;
use flat_message::*;


#[test]
fn check_flags() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A,B,C)]
    pub struct Flags(u32);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
        pub const C: Flags = Flags(4);
    }

    let mut f = Flags::A | Flags::B;
    assert!(f.all_set(Flags::A));
    assert!(f.all_set(Flags::B));
    f.clear();
    f.set(Flags::C);
    assert!(f.all_set(Flags::C));
    assert!(!f.any_set(Flags::A | Flags::B));
    assert!(f & Flags::C == Flags::C);
}

#[test]
fn check_simple() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A,B)]
    pub struct Flags(u32);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u32)]
        flags: Flags
    }    
  
    validate_correct_serde(Test { flags: Flags::A });
}

#[test]
fn check_simple_repr() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A,B)]
    pub struct Flags(u32);

    impl Flags {
        add_flag!(A = 1);
        add_flag!(B = 2);
        // pub const A: Flags = Flags(1);
        // pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    #[flat_message_options(store_name = false)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u32)]
        flags: Flags
    }    
    let t = Test { flags: Flags::A };
    let mut s = Storage::default();
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(s.as_slice(), &[
        70, 76, 77, 1, 1, 0, 0, 0, // Header
        190, 110, 196, 202, // Hash over Name (Test)
        1, 0, 0, 0, // value of field flags
        29, 122, 103, 156, // hash over field flags
        8 // offset of field flags
    ]);
}

#[test]
fn check_slice() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A,B)]
    pub struct Flags(u8);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test<'a> {
        #[flat_message_item(kind = flags, repr = u8)]
        flags: &'a [Flags],
    }    
    let mut s = Storage::default();
    let t = Test { flags: &[Flags::A, Flags::B] };
    t.serialize_to(&mut s, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&s).unwrap();
    assert_eq!(t,t2);
}

#[test]
fn check_vector() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A,B)]
    pub struct Flags(u8);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u8)]
        flags: Vec<Flags>,
    }    
    let mut s = Storage::default();
    let t = Test { flags: vec![Flags::A, Flags::B | Flags::A , Flags::B] };
    t.serialize_to(&mut s, Config::default()).unwrap();
    let t2 = Test::deserialize_from(&s).unwrap();
    assert_eq!(t,t2);
}

#[test]
fn check_slice_repr() {
    #[derive(Copy, Clone, FlatMessageFlags, Eq, PartialEq, Debug)]
    #[repr(transparent)]
    #[flags(A,B)]
    pub struct Flags(u8);

    impl Flags {
        pub const A: Flags = Flags(1);
        pub const B: Flags = Flags(2);
    }

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    #[flat_message_options(store_name = false)]
    struct Test<'a> {
        #[flat_message_item(kind = flags, repr = u8)]
        flags: &'a [Flags],
    }    
    let mut s = Storage::default();
    let t = Test { flags: &[Flags::A, Flags::B, Flags::A | Flags::B] };
    t.serialize_to(&mut s, Config::default()).unwrap();
    assert_eq!(s.as_slice(), &[
        70, 76, 77, 1, 1, 0, 0, 0, // Header
        190, 110, 196, 202, // hash for the Flags
        3, 1, 2, 3, // 3 elements, with value 1,2,3
        155, 122, 103, 156,  // hash for the field "flags"
        8 // offset of "flags"
    ]);
}