use crate::*;
use flat_message::*;

#[test]
fn check_simple() {
    #[derive(Copy, Clone, FlatMessageFlags)]
    #[repr(transparent)]
    #[flags(A,B)]
    pub struct Test(u32);

    impl Test {
        pub const A: Test = Test(1);
        pub const B: Test = Test(2);
    };
}