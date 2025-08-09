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
    };

    #[derive(FlatMessage, Debug, Eq, PartialEq)]
    struct Test {
        #[flat_message_item(kind = flags, repr = u32)]
        flags: Flags
    }    
  
    validate_correct_serde(Test { flags: Flags::A });
}